use crate::doublets::data::links_constants::LinksConstants;
use crate::num::LinkType;
use num_traits::zero;
use libc::labs;
use crate::doublets::data::point::Point;

pub trait IGenericLinks<T: LinkType> {
    fn constants(&self) -> LinksConstants<T> {
        LinksConstants::new()
    }

    fn count_generic<L>(&self, restrictions: L) -> T
        where L: IntoIterator<Item=T, IntoIter: ExactSizeIterator>;

    fn each_generic<F, L>(&self, handler: F, restrictions: L) -> T
        where
            F: FnMut(&[T]) -> T,
            L: IntoIterator<Item=T, IntoIter: ExactSizeIterator>;

    fn create_generic<L>(&mut self, restrictions: L) -> T
        where L: IntoIterator<Item=T, IntoIter: ExactSizeIterator>;

    fn update_generic<Lr, Ls>(&mut self, restrictions: Lr, substitution: Ls) -> T
        where
            Lr: IntoIterator<Item=T, IntoIter: ExactSizeIterator>,
            Ls: IntoIterator<Item=T, IntoIter: ExactSizeIterator>;

    fn delete_generic<L>(&mut self, restrictions: L)
        where L: IntoIterator<Item=T, IntoIter: ExactSizeIterator>;
}

pub trait IGenericLinksExtensions<T: LinkType>: IGenericLinks<T> {
    fn exist(&self, link: T) -> bool
    {
        let constants = self.constants();

        if constants.is_external_reference(link) {
            self.count_generic([link]) != zero()
        } else {
            constants.is_internal_reference(link)
        }
    }

    fn get_link(&self, link: T) -> Box<dyn ExactSizeIterator<Item=T>> {
        let constants = self.constants();
        if constants.is_external_reference(link) {
            box Point::new(link, constants.target_part.as_() + 1).into_iter()
        } else {
            let mut slice: Vec<_> = vec![];
            self.each_generic(
                |link| {
                    slice = link.to_vec();
                    return constants.r#break;
                }, [link]);
            box slice.into_iter()
        }
    }

    fn is_full_point(&self, link: T) -> bool {
        let constants = self.constants();
        if constants.is_external_reference(link) {
            true
        } else {
            assert!(self.exist(link)); // TODO: add message
            Point::is_full(self.get_link(link))
        }
    }

    fn is_partial_point(&self, link: T) -> bool {
        let constants = self.constants();
        if constants.is_external_reference(link) {
            true
        } else {
            assert!(self.exist(link)); // TODO: add message
            Point::is_partial(self.get_link(link))
        }
    }
}

impl<T: LinkType, All: IGenericLinks<T>> IGenericLinksExtensions<T> for All {}
