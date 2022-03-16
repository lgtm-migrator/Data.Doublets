use std::backtrace::Backtrace;
use std::default::default;
use std::ops::{ControlFlow, Try};
use std::result;

use num_traits::{one, zero};
use rand::{thread_rng, Rng};
use smallvec::SmallVec;

use crate::doublets::data::{Links, ReadHandler, ToQuery, WriteHandler};
use crate::doublets::data::{LinksConstants, Point, Query};
use crate::doublets::error::LinksError;
use crate::doublets::link::Link;
use crate::doublets::StoppedHandler;
use crate::doublets::{data, Doublet, Flow};
use crate::num::LinkType;
use crate::query;
use std::error::Error;
use ControlFlow::{Break, Continue};

use crate::doublets::decorators::{
    CascadeUniqueResolver, CascadeUsagesResolver, NonNullDeletionResolver,
};

pub type Result<T, E = LinksError<T>> = std::result::Result<T, E>;

fn IGNORE<T: LinkType>(_: Link<T>, _: Link<T>) -> Result<(), ()> {
    Err(())
}

pub trait Doublets<T: LinkType> {
    fn constants(&self) -> LinksConstants<T>;

    fn count_by(&self, query: impl ToQuery<T>) -> T;

    fn count(&self) -> T {
        self.count_by([])
    }

    fn create_by_with<F, R>(
        &mut self,
        query: impl ToQuery<T>,
        handler: F,
    ) -> Result<R, LinksError<T>>
    where
        F: FnMut(Link<T>, Link<T>) -> R,
        R: Try<Output = ()>;

    fn create_by(&mut self, query: impl ToQuery<T>) -> Result<T, LinksError<T>> {
        let mut index = default();
        self.create_by_with(query, |before, link| {
            index = link.index;
            Flow::Continue
        })
        .map(|_| index)
    }

    fn create_with<F, R>(&mut self, handler: F) -> Result<R, LinksError<T>>
    where
        F: FnMut(Link<T>, Link<T>) -> R,
        R: Try<Output = ()>,
    {
        self.create_by_with([], handler)
    }

    fn create(&mut self) -> Result<T> {
        self.create_by([])
    }

    fn try_each_by<F, R>(&self, restrictions: impl ToQuery<T>, handler: F) -> R
    where
        F: FnMut(Link<T>) -> R,
        R: Try<Output = ()>;

    fn try_each<F, R>(&self, handler: F) -> R
    where
        F: FnMut(Link<T>) -> R,
        R: Try<Output = ()>,
    {
        self.try_each_by([], handler)
    }

    fn each_by<H>(&self, restrictions: impl ToQuery<T>, mut handler: H) -> T
    where
        H: FnMut(Link<T>) -> T,
    {
        let result = self.try_each_by(restrictions, |link| {
            let result = handler(link);
            if result == self.constants().r#continue {
                Continue(())
            } else {
                Break(result)
            }
        });

        match result {
            Continue(_) => self.constants().r#continue,
            Break(result) => result,
        }
    }

    fn each<H>(&self, handler: H) -> T
    where
        H: FnMut(Link<T>) -> T,
    {
        self.each_by([], handler)
    }

    fn update_by_with<F, R>(
        &mut self,
        query: impl ToQuery<T>,
        replacement: impl ToQuery<T>,
        handler: F,
    ) -> Result<R, LinksError<T>>
    where
        F: FnMut(Link<T>, Link<T>) -> R,
        R: Try<Output = ()>;

    fn update_by(&mut self, query: impl ToQuery<T>, replacement: impl ToQuery<T>) -> Result<T> {
        let r#continue = self.constants().r#continue;
        let mut result = default();
        self.update_by_with(query, replacement, |before, after| {
            result = after.index;
            Flow::Continue
        })
        .map(|_| result)
    }

    fn update_with<F, R>(
        &mut self,
        index: T,
        source: T,
        target: T,
        mut handler: F,
    ) -> Result<R, LinksError<T>>
    where
        F: FnMut(Link<T>, Link<T>) -> R,
        R: Try<Output = ()>,
    {
        self.update_by_with([index], [index, source, target], handler)
    }

    fn update(&mut self, index: T, source: T, target: T) -> Result<T> {
        self.update_by([index], [index, source, target])
    }

    fn delete_by_with<F, R>(
        &mut self,
        query: impl ToQuery<T>,
        handler: F,
    ) -> Result<R, LinksError<T>>
    where
        F: FnMut(Link<T>, Link<T>) -> R,
        R: Try<Output = ()>;

    fn delete_by(&mut self, query: impl ToQuery<T>) -> Result<T> {
        let r#continue = self.constants().r#continue;
        let mut result = default();
        self.delete_by_with(query, |before, after| {
            result = after.index;
            Flow::Continue
        })
        .map(|_| result)
    }

    fn delete_with<F, R>(&mut self, index: T, mut handler: F) -> Result<R, LinksError<T>>
    where
        F: FnMut(Link<T>, Link<T>) -> R,
        R: Try<Output = ()>,
    {
        self.delete_by_with([index], handler)
    }

    fn delete(&mut self, index: T) -> Result<T> {
        self.delete_by([index])
    }

    fn try_get_link(&self, index: T) -> Result<Link<T>, LinksError<T>> {
        self.get_link(index).ok_or(LinksError::NotExists(index))
    }

    fn get_link(&self, index: T) -> Option<Link<T>> {
        let constants = self.constants();
        if constants.is_external(index) {
            Some(Link::point(index))
        } else {
            let mut slice = None;
            self.each_by([index], |link| {
                slice = Some(link);
                constants.r#break
            });
            slice
        }
    }

    fn delete_all(&mut self) -> Result<(), LinksError<T>> {
        let mut count = self.count();
        while count > zero() {
            self.delete(count)?;
            let sup_count = self.count();
            if sup_count != count - one() {
                count = sup_count
            } else {
                count = count - one()
            }
        }
        Ok(())
    }

    fn delete_query_with<F, R>(
        &mut self,
        query: impl ToQuery<T>,
        mut handler: F,
    ) -> Result<(), LinksError<T>>
    where
        F: FnMut(Link<T>, Link<T>) -> R,
        R: Try<Output = ()>,
    {
        let query = query.to_query();
        let constants = self.constants();
        let len = self.count_by(query.to_query()).as_();
        let mut vec = Vec::with_capacity(len);

        self.each_by(query, |link| {
            vec.push(link.index);
            constants.r#continue
        });

        let mut handler = StoppedHandler::new(handler);
        for index in vec.into_iter().rev() {
            self.delete_with(index, &mut handler)?;
        }
        Ok(())
    }

    fn delete_usages_with<F, R>(&mut self, index: T, mut handler: F) -> Result<(), LinksError<T>>
    where
        F: FnMut(Link<T>, Link<T>) -> R,
        R: Try<Output = ()>,
    {
        let any = self.constants().any;
        let mut to_delete = Vec::with_capacity(
            self.count_by([any, index, any]).as_() + self.count_by([any, any, index]).as_(),
        );
        self.try_each_by([any, index, any], |link| {
            if link.index != index {
                to_delete.push(link.index);
            }
            Flow::Continue
        });

        self.try_each_by([any, any, index], |link| {
            if link.index != index {
                to_delete.push(link.index);
            }
            Flow::Continue
        });

        let mut handler = StoppedHandler::new(handler);
        for index in to_delete.into_iter().rev() {
            self.delete_with(index, &mut handler)?;
        }
        Ok(())
    }

    fn delete_usages(&mut self, index: T) -> Result<(), LinksError<T>> {
        self.delete_usages_with(index, IGNORE)
    }

    fn create_point(&mut self) -> Result<T> {
        let new = self.create()?;
        self.update(new, new, new)
    }

    #[deprecated(note = "use `create_link` instead")]
    fn create_and_update(&mut self, source: T, target: T) -> Result<T> {
        self.create_link(source, target)
    }

    fn create_link_with<F, R>(
        &mut self,
        source: T,
        target: T,
        mut handler: F,
    ) -> Result<Flow, LinksError<T>>
    where
        F: FnMut(Link<T>, Link<T>) -> R,
        R: Try<Output = ()>,
    {
        let mut new = default();
        let mut handler = StoppedHandler::new(handler);
        self.create_with(|before, after| {
            new = after.index;
            handler(before, after);
            Flow::Continue
        })?;

        self.update_with(new, source, target, handler)
    }

    fn create_link(&mut self, source: T, target: T) -> Result<T> {
        let mut result = default();
        self.create_link_with(source, target, |_, link| {
            result = link.index;
            Flow::Continue
        })
        .map(|_| result)
    }

    #[deprecated(note = "use `links.search(source, target).unwrap_or(or)`")]
    fn search_or(&self, source: T, target: T, or: T) -> T {
        self.search(source, target).unwrap_or(or)
    }

    fn found(&self, query: impl ToQuery<T>) -> bool {
        self.count_by(query) != zero()
    }

    fn find(&self, query: impl ToQuery<T>) -> Option<T> {
        let mut result = None;
        self.try_each_by(query, |link| {
            result = Some(link.index);
            Flow::Break
        });
        result
    }

    fn search(&self, source: T, target: T) -> Option<T> {
        self.find([self.constants().any, source, target])
    }

    fn single(&self, query: impl ToQuery<T>) -> Option<Link<T>> {
        let query = query.to_query();
        let constants = self.constants();

        let mut result = None;
        let mut marker = false;
        self.try_each_by(query, |link| {
            if !marker {
                result = Some(link);
                marker = true;
                Flow::Continue
            } else {
                result = None;
                Flow::Break
            }
        });
        result
    }

    // TODO: use later `links.iter().map(|link| link.index).collect()`
    fn all_indices(&self, query: impl ToQuery<T>) -> Vec<T> {
        let query = query.to_query();
        let len = self.count_by(query.to_query()).as_();
        let mut vec = Vec::with_capacity(len);
        self.each_by(query, |link| {
            vec.push(link.index);
            self.constants().r#continue
        });
        vec
    }

    fn get_or_create(&mut self, source: T, target: T) -> Result<T> {
        if let Some(link) = self.search(source, target) {
            Ok(link)
        } else {
            self.create_and_update(source, target)
        }
    }

    fn count_usages(&self, index: T) -> Result<T> {
        let constants = self.constants();
        let any = constants.any;
        // TODO: delegate error
        let link = self.try_get_link(index)?;
        let mut usage_source = self.count_by([any, index, any]);
        if index == link.source {
            usage_source = usage_source - one();
        }

        let mut usage_target = self.count_by([any, any, index]);
        if index == link.target {
            usage_target = usage_target - one();
        }

        Ok(usage_source + usage_target)
    }

    fn exist(&self, link: T) -> bool {
        let constants = self.constants();
        if constants.is_external(link) {
            true
        } else {
            constants.is_internal(link) && self.count_by([link]) != zero()
        }
    }

    fn has_usages(&self, link: T) -> bool {
        self.count_usages(link).map_or(false, |link| link != zero())
    }

    fn rebase_with<F, R>(&mut self, old: T, new: T, mut handler: F) -> Result<(), LinksError<T>>
    where
        F: FnMut(Link<T>, Link<T>) -> R,
        R: Try<Output = ()>,
    {
        if old == new {
            return Ok(());
        }

        let any = self.constants().any;
        let as_source = [any, old, any];
        let as_target = [any, any, old];

        let sources_count: usize = self.count_by(as_source).as_();
        let targets_count: usize = self.count_by(as_target).as_();

        // not borrowed
        if sources_count + targets_count == 0 {
            return Ok(());
        }

        let mut handler = StoppedHandler::new(handler);

        let mut usages = Vec::with_capacity(sources_count);
        self.try_each_by(as_source, |link| {
            usages.push(link);
            Flow::Continue
        });

        for usage in usages {
            if usage.index != old {
                self.update_with(usage.index, new, usage.target, &mut handler)?;
            }
        }

        let mut usages = Vec::with_capacity(sources_count);
        self.try_each_by(as_target, |link| {
            usages.push(link);
            Flow::Continue
        });

        for usage in usages {
            if usage.index != old {
                self.update_with(usage.index, usage.source, new, &mut handler)?;
            }
        }
        Ok(())
    }

    fn rebase(&mut self, old: T, new: T) -> Result<T> {
        let link = self.try_get_link(old)?;

        if old == new {
            return Ok(new);
        }

        let constants = self.constants();
        let any = constants.any;

        let sources_count = self.count_by([any, old, any]).as_();
        let targets_count = self.count_by([any, any, old]).as_();
        if sources_count == 0 && targets_count == 0 && link.is_full() {
            return Ok(new);
        }

        let total = sources_count + targets_count;
        if total == 0 {
            return Ok(new);
        }

        let mut usages = Vec::with_capacity(sources_count);
        self.each_by([any, old, any], |link| {
            usages.push(link.index);
            constants.r#continue
        });

        for index in usages {
            if index != old {
                let usage = self.try_get_link(index)?;
                self.update(index, new, usage.target)?;
            }
        }

        let mut usages = Vec::with_capacity(sources_count);
        self.each_by([any, any, old], |link| {
            usages.push(link.index);
            constants.r#continue
        });

        for index in usages {
            if index != old {
                let usage = self.try_get_link(index)?;
                self.update(index, usage.source, new)?;
            }
        }
        Ok(new)
    }

    fn rebase_and_delete(&mut self, old: T, new: T) -> Result<T> {
        if old == new {
            Ok(new)
        } else {
            self.rebase(old, new)?;
            self.delete(old)
        }
    }

    fn reset(&mut self, link: T) -> Result<T, LinksError<T>> {
        self.update(link, T::zero(), T::zero())
    }

    fn format(&self, link: T) -> Option<String> {
        self.get_link(link).map(|link| link.to_string())
    }

    fn decorators_kit(
        self,
    ) -> CascadeUniqueResolver<T, NonNullDeletionResolver<T, CascadeUsagesResolver<T, Self>>>
    where
        Self: Sized,
    {
        let links = self;
        let links = CascadeUsagesResolver::new(links);
        let links = NonNullDeletionResolver::new(links);
        CascadeUniqueResolver::new(links)
    }

    #[deprecated(note = "use `links.try_get_link(...)?.is_full()`")]
    fn is_full_point(&self, link: T) -> Option<bool> {
        self.get_link(link).map(|link| link.is_full())
    }

    #[deprecated(note = "use `links.try_get_link(...)?.is_partial()`")]
    fn is_partial_point(&self, link: T) -> Option<bool> {
        self.get_link(link).map(|link| link.is_partial())
    }

    #[deprecated(note = "only development")]
    fn continue_break(&self) -> (T, T) {
        (self.constants().r#continue, self.constants().r#break)
    }
}

// TODO: Remove it
#[deprecated(note = "use `ILinks`")]
pub trait ILinksExtensions<T: LinkType>: Doublets<T> {}

impl<T: LinkType, All: Doublets<T>> ILinksExtensions<T> for All {}

impl<T: LinkType, All: Doublets<T>> Links<T> for All {
    fn constants_links(&self) -> LinksConstants<T> {
        self.constants()
    }

    fn count_links(&self, query: &[T]) -> T {
        self.count_by(query)
    }

    fn create_links(
        &mut self,
        query: &[T],
        handler: WriteHandler<T>,
    ) -> Result<Flow, Box<dyn Error>> {
        self.create_by_with(query, |before, after| handler(&before[..], &after[..]))
            .map_err(|err| err.into())
    }

    fn each_links(&self, query: &[T], handler: ReadHandler<T>) -> Result<Flow, Box<dyn Error>> {
        Ok(self.try_each_by(query, |link| handler(&link[..])))
    }

    fn update_links(
        &mut self,
        query: &[T],
        replacement: &[T],
        handler: WriteHandler<T>,
    ) -> Result<Flow, Box<dyn Error>> {
        self.update_by_with(query, replacement, |before, after| {
            handler(&before[..], &after[..])
        })
        .map_err(|err| err.into())
    }

    fn delete_links(
        &mut self,
        query: &[T],
        handler: WriteHandler<T>,
    ) -> result::Result<Flow, Box<dyn Error>> {
        self.delete_by_with(query, |before, after| handler(&before[..], &after[..]))
            .map_err(|err| err.into())
    }
}