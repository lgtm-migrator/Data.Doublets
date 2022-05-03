use crate::mem::united::{
    LinksSourcesRecursionlessSizeBalancedTree, LinksTargetsRecursionlessSizeBalancedTree, Store,
    UnusedLinks,
};
use crate::mem::{splited, united};
use crate::{Doublets, LinksError};
use data::LinksConstants;
use mem::{AllocMem, GlobalMem, RawMem};
use num::LinkType;
use std::alloc::Global;
use std::io;
use std::time::Instant;

// TODO: cfg!
//pub fn make_mem() -> io::Result<HeapMem> {
//    Ok(HeapMem::new()?)
//}

pub fn make_mem() -> io::Result<(GlobalMem, GlobalMem)> {
    Ok((GlobalMem::new()?, GlobalMem::new()?))
}

//pub fn make_links<M: ResizeableMem>(mem: M) -> Result<Links<usize, M>, LinksError<usize>> {
//    united::Links::<usize, _>::new(mem)
//}

pub fn make_links<M1: RawMem, M2: RawMem>(
    mem: (M1, M2),
) -> Result<splited::Store<usize, M1, M2>, LinksError<usize>> {
    let constants = LinksConstants::via_only_external(true);
    splited::Store::<usize, _, _>::with_constants(mem.0, mem.1, constants)
}

//pub fn typed_links<T: LinkType, M: ResizeableMem>(mem: M) -> Result<Links<T, M>, LinksError<T>> {
//    united::Links::<T, _>::with_constants(mem, LinksConstants::via_only_external(true))
//}

pub fn typed_links<T: LinkType, M1: RawMem, M2: RawMem>(
    mem: (M1, M2),
) -> Result<splited::Store<T, M1, M2>, LinksError<T>> {
    splited::Store::<T, _, _>::with_constants(mem.0, mem.1, LinksConstants::via_only_external(true))
}

// TODO: use the follow style:
//  #[cfg(test)]
//  mod tests {
//     mods...
//  }

#[cfg(test)]
mod crud;
#[cfg(test)]
mod hybrid;
#[cfg(test)]
mod ilinks_basic;
#[cfg(test)]
mod sync;
#[cfg(test)]
mod trees;
