use std::default::default;
use std::marker::PhantomData;
use std::ops::Try;

use doublets::data::LinksConstants;
use doublets::data::ToQuery;
use doublets::num::LinkType;

use doublets::Error;
use doublets::{Doublets, Link};

pub struct NonNullDeletionResolver<T: LinkType, L: Doublets<T>> {
    links: L,

    _phantom: PhantomData<T>,
}

impl<T: LinkType, L: Doublets<T>> NonNullDeletionResolver<T, L> {
    pub fn new(links: L) -> Self {
        Self {
            links,
            _phantom: default(),
        }
    }
}

impl<T: LinkType, L: Doublets<T>> Doublets<T> for NonNullDeletionResolver<T, L> {
    fn constants(&self) -> LinksConstants<T> {
        self.links.constants()
    }

    fn count_by(&self, query: impl ToQuery<T>) -> T {
        self.links.count_by(query)
    }

    fn create_by_with<F, R>(&mut self, query: impl ToQuery<T>, handler: F) -> Result<R, Error<T>>
    where
        F: FnMut(Link<T>, Link<T>) -> R,
        R: Try<Output = ()>,
    {
        self.links.create_by_with(query, handler)
    }

    fn each_by<F, R>(&self, restrictions: impl ToQuery<T>, handler: F) -> R
    where
        F: FnMut(Link<T>) -> R,
        R: Try<Output = ()>,
    {
        self.links.each_by(restrictions, handler)
    }

    fn update_by_with<F, R>(
        &mut self,
        query: impl ToQuery<T>,
        change: impl ToQuery<T>,
        handler: F,
    ) -> Result<R, Error<T>>
    where
        F: FnMut(Link<T>, Link<T>) -> R,
        R: Try<Output = ()>,
    {
        self.links.update_by_with(query, change, handler)
    }

    fn delete_by_with<F, R>(
        &mut self,
        query: impl ToQuery<T>,
        mut handler: F,
    ) -> Result<R, Error<T>>
    where
        F: FnMut(Link<T>, Link<T>) -> R,
        R: Try<Output = ()>,
    {
        let null = self.links.constants().null;
        let query = query.to_query();
        self.links
            .update_by_with(query.to_query(), [query[0], null, null], &mut handler)?; // TODO: MAY BE STANGE BEHAVIOUR
        self.links.delete_by_with(query, handler)
    }
}
