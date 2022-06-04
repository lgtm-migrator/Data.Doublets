use bumpalo::Bump;

use std::alloc::System;

use std::time::Instant;

use crate::mem::splited;
use crate::{Doublets, LinksError};

use mem::{AllocMem, GlobalMem, RawMem, TempFileMem};
//use crate::decorators::{CascadeUsagesResolver, NonNullDeletionResolver};
//use crate::mem::splited;
use crate::mem::united::Store;

/*
#[test]
fn random_creations_and_deletions() {
    std::fs::remove_file("db.links");

    let mem = make_mem().unwrap();
    let mut links = make_links(mem).unwrap();
    let mut links = links.decorators_kit();

    let instant = Instant::now();
    links.test_random_creations_and_deletions(1000);

    println!("{:?}", instant.elapsed());
}
*/
// TODO: use `tempfile` as dev
// #[test]
// fn mapping() {
//     std::fs::remove_file("mapping_file");
//     let mut mem = FileMappedMem::new("mapping_file").unwrap();
//
//     mem.alloc(13370).unwrap();
//     mem.alloc(2280).unwrap();
//
//     let mut file = std::fs::File::open("mapping_file").unwrap();
//     assert_eq!(13370, file.metadata().unwrap().len() as usize);
// }

#[test]
fn billion_points_file_mapped() {
    std::fs::remove_file("mem_bpfm").ok();

    let mem = TempFileMem::new().unwrap();
    let mut links = Store::<usize, _>::new(mem).unwrap();

    let instant = Instant::now();

    for _ in 0..1_000_000 {
        links.create_point().unwrap();
    }

    println!("{:?}", instant.elapsed());
}

#[test]
fn billion_points_heap_mem() -> Result<(), LinksError<usize>> {
    let mem = GlobalMem::new()?;
    let mut links = Store::<usize, _>::new(mem)?;

    let instant = Instant::now();

    for _ in 0..1_000_000 {
        links.create_point()?;
    }

    println!("{:?}", instant.elapsed());
    Ok(())
}

#[test]
fn billion_points_bump_alloc() {
    let bump = Bump::new();
    let mem = AllocMem::new(&bump).unwrap();
    let mut links = Store::<usize, _>::new(mem).unwrap();

    let instant = Instant::now();

    for _ in 0..1_000_000 {
        links.create_point().unwrap();
    }

    println!("{:?}", instant.elapsed());
}

#[test]
fn many_points_and_searches() {
    let _bump = Bump::new();
    let mem = AllocMem::new(System).unwrap();
    let mut links = Store::<usize, _>::new(mem).unwrap();

    let instant = Instant::now();
    for _ in 0..1_000_000 {
        links.create_point().unwrap();
    }
    println!("{:?}", instant.elapsed());

    let instant = Instant::now();
    for i in 1..=1_000_000 {
        // links.search_or(i, i, 0);
        links.get_link(i);
    }
    println!("{:?}", instant.elapsed());
}

// TODO: Create `TempFileMappedMem`

#[test]
fn billion_points_file_mapped_splited() {
    let mem = TempFileMem::new().unwrap();
    let index = TempFileMem::new().unwrap();
    let mut links = splited::Store::<usize, _, _>::new(mem, index).unwrap();

    let instant = Instant::now();

    for _ in 0..1_000_000 {
        links.create_point().unwrap();
    }

    println!("{:?}", instant.elapsed());
}

#[test]
fn billion_points_heap_mem_splited() {
    let mem = GlobalMem::new().unwrap();
    let index = GlobalMem::new().unwrap();
    let mut links = splited::Store::<usize, _, _>::new(mem, index).unwrap();

    let instant = Instant::now();

    for _ in 0..1_000_000 {
        links.create_point().unwrap();
    }

    println!("{:?}", instant.elapsed());
}

#[test]
fn billion_points_bump_alloc_splited() {
    let bump = Bump::new();
    let mem = AllocMem::new(&bump).unwrap();
    let mut index = AllocMem::new(&bump).unwrap();
    index.alloc(1023 * 1023).unwrap();
    let mut links = splited::Store::<usize, _, _>::new(mem, index).unwrap();

    let instant = Instant::now();

    for _ in 0..1_000_000 {
        links.create_point().unwrap();
    }

    println!("{:?}", instant.elapsed());
}

#[test]
fn many_points_and_searches_splited() {
    let mem = AllocMem::new(System).unwrap();
    let index = AllocMem::new(System).unwrap();
    let mut links = splited::Store::<usize, _, _>::new(mem, index).unwrap();

    let instant = Instant::now();
    for _ in 0..1_000_000 {
        links.create_point().unwrap();
    }
    println!("{:?}", instant.elapsed());

    let instant = Instant::now();
    for i in 1..=1_000_000 {
        links.search_or(i, i, 0);
    }
    println!("{:?}", instant.elapsed());
}
