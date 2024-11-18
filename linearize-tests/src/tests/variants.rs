use {
    linearize::{Linearize, LinearizeExt},
    std::{cmp::Ordering, convert::Infallible},
};

#[test]
fn bool() {
    let v: Vec<_> = bool::variants().collect();
    assert_eq!(v, [false, true]);
}

#[test]
fn ordering() {
    let v: Vec<_> = Ordering::variants().collect();
    assert_eq!(v, [Ordering::Less, Ordering::Equal, Ordering::Greater]);
}

#[test]
fn u8() {
    let v: Vec<_> = u8::variants().collect();
    assert_eq!(v, (0..=255u8).collect::<Vec<_>>());
}

#[test]
fn derived_1() {
    #[derive(Linearize, PartialEq, Debug)]
    enum A {
        A,
        B,
    }
    let v: Vec<_> = A::variants().collect();
    assert_eq!(v, [A::A, A::B]);
}

#[test]
fn derived_2() {
    #[derive(Linearize, PartialEq, Debug)]
    enum A {
        A,
        B(bool),
        C(Infallible),
    }
    let v: Vec<_> = A::variants().collect();
    assert_eq!(v, [A::A, A::B(false), A::B(true)]);
}

#[test]
fn derived_3() {
    #[derive(Linearize, PartialEq, Debug)]
    struct A(bool, bool);
    let v: Vec<_> = A::variants().collect();
    assert_eq!(
        v,
        [
            A(false, false),
            A(false, true),
            A(true, false),
            A(true, true),
        ]
    );
}

#[test]
fn clone() {
    let mut iter = bool::variants();
    assert_eq!(iter.next(), Some(false));
    let mut iter2 = iter.clone();
    assert_eq!(iter.next(), Some(true));
    assert_eq!(iter.next(), None);
    assert_eq!(iter2.next(), Some(true));
    assert_eq!(iter2.next(), None);
}

#[test]
fn size_hint() {
    let iter = bool::variants();
    assert_eq!(iter.size_hint(), (2, Some(2)));
}

#[test]
fn count() {
    let iter = bool::variants();
    assert_eq!(iter.count(), 2);
    let mut iter = bool::variants();
    iter.next();
    assert_eq!(iter.count(), 1);
}

#[test]
fn last() {
    let iter = bool::variants();
    assert_eq!(iter.last(), Some(true));
}

#[test]
fn nth() {
    let mut iter = bool::variants();
    assert_eq!(iter.nth(0), Some(false));
    assert_eq!(iter.nth(0), Some(true));
    assert_eq!(iter.nth(0), None);
    let mut iter = bool::variants();
    assert_eq!(iter.nth(1), Some(true));
    assert_eq!(iter.nth(0), None);
    let mut iter = bool::variants();
    assert_eq!(iter.nth(2), None);
}

#[test]
fn next_back() {
    let mut iter = bool::variants();
    assert_eq!(iter.next_back(), Some(true));
    assert_eq!(iter.next_back(), Some(false));
    assert_eq!(iter.next_back(), None);
}

#[test]
fn nth_back() {
    let mut iter = bool::variants();
    assert_eq!(iter.nth_back(0), Some(true));
    assert_eq!(iter.nth_back(0), Some(false));
    assert_eq!(iter.nth_back(0), None);
    let mut iter = bool::variants();
    assert_eq!(iter.nth_back(1), Some(false));
    assert_eq!(iter.nth_back(0), None);
    let mut iter = bool::variants();
    assert_eq!(iter.nth_back(2), None);
}
