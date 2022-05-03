use num_traits::{one, zero};
use std::ops::Try;

use crate::mem::ilinks_list_methods::ILinksListMethods;
use crate::mem::ilinks_tree_methods::ILinksTreeMethods;
use crate::mem::links_header::LinksHeader;
use crate::mem::united::generic::links_size_balanced_tree_base::{
    LinksSizeBalancedTreeBase, LinksSizeBalancedTreeBaseAbstract,
};
use crate::mem::united::generic::UpdatePointers;
use crate::mem::united::raw_link::RawLink;
use crate::Link;
use data::LinksConstants;
use methods::SizeBalancedTreeMethods;

use num::LinkType;

pub struct LinksTargetsSizeBalancedTree<T: LinkType> {
    base: LinksSizeBalancedTreeBase<T>,
}

impl<T: LinkType> LinksTargetsSizeBalancedTree<T> {
    pub fn new(constants: LinksConstants<T>, links: *mut u8, header: *mut u8) -> Self {
        Self {
            base: LinksSizeBalancedTreeBase::new(constants, links, header),
        }
    }
}

impl<T: LinkType> SizeBalancedTreeMethods<T> for LinksTargetsSizeBalancedTree<T> {
    fn get_left_reference(&self, node: T) -> *const T {
        &self.get_link(node).left_as_target
    }

    fn get_right_reference(&self, node: T) -> *const T {
        &self.get_link(node).right_as_target
    }

    fn get_mut_left_reference(&mut self, node: T) -> *mut T {
        &mut self.get_mut_link(node).left_as_target
    }

    fn get_mut_right_reference(&mut self, node: T) -> *mut T {
        &mut self.get_mut_link(node).right_as_target
    }

    fn get_left(&self, node: T) -> T {
        self.get_link(node).left_as_target
    }

    fn get_right(&self, node: T) -> T {
        self.get_link(node).right_as_target
    }

    fn get_size(&self, node: T) -> T {
        self.get_link(node).size_as_target
    }

    fn set_left(&mut self, node: T, left: T) {
        self.get_mut_link(node).left_as_target = left
    }

    fn set_right(&mut self, node: T, right: T) {
        self.get_mut_link(node).right_as_target = right
    }

    fn set_size(&mut self, node: T, size: T) {
        self.get_mut_link(node).size_as_target = size
    }

    fn first_is_to_the_left_of_second(&self, first: T, second: T) -> bool {
        let first = self.get_link(first);
        let second = self.get_link(second);
        (first.target < second.target)
            || (first.target == second.target && first.source < second.source)
    }

    fn first_is_to_the_right_of_second(&self, first: T, second: T) -> bool {
        let first = self.get_link(first);
        let second = self.get_link(second);
        (first.target > second.target)
            || (first.target == second.target && first.source > second.source)
    }

    fn clear_node(&mut self, node: T) {
        let link = self.get_mut_link(node);
        link.left_as_target = zero();
        link.right_as_target = zero();
        link.size_as_target = zero();
    }
}

impl<T: LinkType> LinksSizeBalancedTreeBaseAbstract<T> for LinksTargetsSizeBalancedTree<T> {
    fn get_header(&self) -> &LinksHeader<T> {
        unsafe { &*(self.base.header as *const LinksHeader<T>) }
    }

    fn get_mut_header(&mut self) -> &mut LinksHeader<T> {
        unsafe { &mut *(self.base.header as *mut LinksHeader<T>) }
    }

    fn get_link(&self, link: T) -> &RawLink<T> {
        unsafe { &*((self.base.links as *const RawLink<T>).add(link.as_())) }
    }

    fn get_mut_link(&mut self, link: T) -> &mut RawLink<T> {
        unsafe { &mut *((self.base.links as *mut RawLink<T>).add(link.as_())) }
    }

    fn get_tree_root(&self) -> T {
        self.get_header().root_as_target
    }

    fn get_base_part(&self, link: T) -> T {
        self.get_link(link).target
    }

    fn first_is_to_the_left_of_second_4(
        &self,
        first_source: T,
        first_target: T,
        second_source: T,
        second_target: T,
    ) -> bool {
        (first_target < second_target)
            || (first_target == second_target && first_source < second_source)
    }

    fn first_is_to_the_right_of_second_4(
        &self,
        first_source: T,
        first_target: T,
        second_source: T,
        second_target: T,
    ) -> bool {
        (first_target > second_target)
            || (first_target == second_target && first_source > second_source)
    }
}

fn each_usages_core<T: LinkType, R: Try<Output = ()>, H: FnMut(Link<T>) -> R>(
    _self: &LinksTargetsSizeBalancedTree<T>,
    root: T,
    link: T,
    handler: &mut H,
) -> R {
    let base = root;

    if link == zero() {
        return R::from_output(());
    }

    let base_part = _self.get_base_part(link);
    if base_part > base {
        each_usages_core(_self, base, _self.get_left(link), handler)?;
    }
    if base_part < base {
        each_usages_core(_self, base, _self.get_right(link), handler)?;
    }

    let values = _self.get_link_value(link);
    handler(values)?;
    each_usages_core(_self, base, _self.get_left(link), handler)?;
    each_usages_core(_self, base, _self.get_right(link), handler)?;
    R::from_output(())
}

impl<T: LinkType> UpdatePointers for LinksTargetsSizeBalancedTree<T> {
    fn update_pointers(&mut self, links: *mut u8, header: *mut u8) {
        self.base.links = links;
        self.base.header = header;
    }
}

impl<T: LinkType> ILinksTreeMethods<T> for LinksTargetsSizeBalancedTree<T> {
    fn count_usages(&self, link: T) -> T {
        let mut root = self.get_tree_root();
        let total = self.get_size(root);

        let mut total_right_ignore = zero();
        while root != zero() {
            let base = self.get_base_part(root);
            if base <= link {
                root = self.get_right(root);
            } else {
                total_right_ignore =
                    total_right_ignore + (self.get_size(self.get_right(root)) + one());
                root = self.get_left(root);
            }
        }

        root = self.get_tree_root();
        let mut total_left_ignore = zero();
        while root != zero() {
            let base = self.get_base_part(root);
            if base >= link {
                root = self.get_left(root);
            } else {
                total_left_ignore =
                    total_left_ignore + (self.get_size(self.get_left(root)) + one());
                root = self.get_right(root);
            }
        }

        total - total_right_ignore - total_left_ignore
    }

    fn search(&self, source: T, target: T) -> T {
        let mut root = self.get_tree_root();
        while root != zero() {
            let root_link = self.get_link(root);
            let root_source = root_link.source;
            let root_target = root_link.target;

            if self.first_is_to_the_left_of_second_4(source, target, root_source, root_target) {
                root = self.get_left(root);
            } else if self.first_is_to_the_right_of_second_4(
                source,
                target,
                root_source,
                root_target,
            ) {
                root = self.get_right(root);
            } else {
                return root;
            }
        }
        zero()
    }

    fn each_usages<H: FnMut(Link<T>) -> R, R: Try<Output = ()>>(
        &self,
        root: T,
        mut handler: H,
    ) -> R {
        each_usages_core(self, root, self.get_tree_root(), &mut handler)
    }

    fn detach(&mut self, root: &mut T, index: T) {
        SizeBalancedTreeMethods::detach(self, root, index)
    }

    fn attach(&mut self, root: &mut T, index: T) {
        SizeBalancedTreeMethods::attach(self, root, index)
    }
}
