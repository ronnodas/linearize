use {
    arbitrary::{unstructured::Unstructured, Arbitrary},
    linearize::{static_copy_map, static_map, StaticCopyMap, StaticMap},
};

#[test]
fn arbitrary() {
    let data = [1u8, 2];
    assert_eq!(StaticMap::<bool, u8>::size_hint(0).0, 2);
    assert_eq!(StaticCopyMap::<bool, u8>::size_hint(0).0, 2);
    assert_eq!(
        StaticMap::<bool, u8>::arbitrary(&mut Unstructured::new(&data)).unwrap(),
        static_map! {
            false => 1,
            true => 2,
        },
    );
    assert_eq!(
        StaticCopyMap::<bool, u8>::arbitrary(&mut Unstructured::new(&data)).unwrap(),
        static_copy_map! {
            false => 1,
            true => 2,
        },
    );
}
