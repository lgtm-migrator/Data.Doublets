use crate::test_extensions::ILinksTestExtensions;
use crate::tests::{make_links, make_mem, typed_links};
use crate::Doublets;

use data::{AddrToRaw, LinksConstants, RawToAddr};

#[test]
fn non_exist_reference() {
    let mem = make_mem().unwrap();
    let mut links = make_links(mem).unwrap();

    let link = links.create().unwrap();
    links.update(link, usize::MAX, usize::MAX).unwrap();

    let mut result = 0;
    links.each_by([links.constants().any, usize::MAX, usize::MAX], |found| {
        result = found.index;
        links.constants().r#break
    });

    assert_eq!(result, link);
    assert_eq!(links.count_by([usize::MAX]), 0);
    links.delete(link).unwrap();
}

#[test]
fn raw_numbers() {
    let mem = make_mem().unwrap();
    let mut links = make_links(mem).unwrap();

    links.test_raw_numbers_crud();
}

#[test]
fn u128_raw_numbers() {
    let mem = make_mem().unwrap();
    //let mut links = typed_links::<u128, _>(mem);
    let mut links = typed_links(mem).unwrap();

    links.get_link(0_u128);

    let constants = LinksConstants::via_only_external(true);
    let to_raw = AddrToRaw;
    let to_adr = RawToAddr;

    let raw = to_raw.convert(1_u128);
    assert!(constants.is_external(raw));

    let adr = to_adr.convert(raw);
    assert!(constants.is_internal(adr));

    let source = to_raw.convert(228_1337_1754_177013_666_777_u128);
    let target = to_raw.convert(10_1011_0011_0111_0101_u128);
    let address = links.create_and_update(source, target).unwrap();

    let link = links.get_link(address).unwrap();
    assert_eq!(
        to_adr.convert(link.source),
        228_1337_1754_177013_666_777_u128
    );
    assert_eq!(to_adr.convert(link.target), 10_1011_0011_0111_0101_u128);
}
