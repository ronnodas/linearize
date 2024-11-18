use {
    linearize::{Linearize, LinearizeExt},
    std::cmp::Ordering,
};

#[test]
fn linearized() {
    assert_eq!(
        Ordering::Less.linearized().get(),
        Ordering::Less.linearize()
    );
    assert_eq!(
        Ordering::Equal.linearized().get(),
        Ordering::Equal.linearize()
    );
    assert_eq!(
        Ordering::Greater.linearized().get(),
        Ordering::Greater.linearize()
    );
    assert_eq!(Ordering::Less.linearized().delinearize(), Ordering::Less);
    assert_eq!(Ordering::Equal.linearized().delinearize(), Ordering::Equal);
    assert_eq!(
        Ordering::Greater.linearized().delinearize(),
        Ordering::Greater
    );
}
