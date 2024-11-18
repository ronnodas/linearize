use linearize::LinearizeExt;

#[test]
fn from_linear() {
    assert_eq!(bool::from_linear(0), Some(false));
    assert_eq!(bool::from_linear(1), Some(true));
    assert_eq!(bool::from_linear(2), None);
}

#[test]
fn variants() {
    let mut variants = bool::variants();
    assert_eq!(variants.next(), Some(false));
    assert_eq!(variants.next(), Some(true));
    assert_eq!(variants.next(), None);
}

#[test]
fn linearized() {
    assert_eq!(false.linearized().get(), 0);
    assert_eq!(true.linearized().get(), 1);
}
