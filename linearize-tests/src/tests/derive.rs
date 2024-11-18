use {
    linearize::Linearize,
    std::{cmp::Ordering, convert::Infallible},
};

#[test]
fn unit_struct() {
    #[derive(Linearize, Debug, PartialEq)]
    struct S;

    test_enumerated! {
        S:
        (S),
    }
}

#[test]
fn custom_crate() {
    use linearize as custom;

    #[derive(Linearize, Debug, PartialEq)]
    #[linearize(crate = custom)]
    enum A {
        A,
        B(bool),
        C { a: bool },
    }

    test_enumerated! {
        A:
        (A::A),
        (A::B(false)),
        (A::B(true)),
        (A::C { a: false }),
        (A::C { a: true }),
    }
}

#[test]
fn tuple_struct_1() {
    #[derive(Linearize, Debug, PartialEq)]
    struct S(bool);

    test_enumerated! {
        S:
        (S(false)),
        (S(true)),
    }
}

#[test]
fn tuple_struct_2() {
    #[derive(Linearize, Debug, PartialEq)]
    struct S(bool, Ordering);

    test_enumerated! {
        S:
        (S(false, Ordering::Less)),
        (S(false, Ordering::Equal)),
        (S(false, Ordering::Greater)),
        (S(true, Ordering::Less)),
        (S(true, Ordering::Equal)),
        (S(true, Ordering::Greater)),
    }
}

#[test]
fn empty_struct() {
    #[derive(Linearize, Debug, PartialEq)]
    struct S {}

    test_enumerated! {
        S:
        (S {}),
    }
}

#[test]
fn struct_1() {
    #[derive(Linearize, Debug, PartialEq)]
    struct S {
        a: bool,
    }

    test_enumerated! {
        S:
        (S { a: false }),
        (S { a: true }),
    }
}

#[test]
fn struct_2() {
    #[derive(Linearize, Debug, PartialEq)]
    struct S {
        a: bool,
        b: Ordering,
    }

    test_enumerated! {
        S:
        (S { a: false, b: Ordering::Less }),
        (S { a: false, b: Ordering::Equal }),
        (S { a: false, b: Ordering::Greater }),
        (S { a: true, b: Ordering::Less }),
        (S { a: true, b: Ordering::Equal }),
        (S { a: true, b: Ordering::Greater }),
    }
}

#[test]
fn struct_infallible_1() {
    #[derive(Linearize, Debug, PartialEq)]
    struct S {
        a: bool,
        b: Ordering,
        c: Infallible,
    }

    #[allow(unreachable_code)]
    {
        test_enumerated! {
            S:
        }
    }
}

#[test]
fn struct_infallible_2() {
    #[derive(Linearize, Debug, PartialEq)]
    struct S {
        a: bool,
        c: Infallible,
        b: Ordering,
    }

    #[allow(unreachable_code)]
    {
        test_enumerated! {
            S:
        }
    }
}

#[test]
fn struct_infallible_3() {
    #[derive(Linearize, Debug, PartialEq)]
    struct S {
        c: Infallible,
        a: bool,
        b: Ordering,
    }

    #[allow(unreachable_code)]
    {
        test_enumerated! {
            S:
        }
    }
}

#[test]
fn enum_empty() {
    #[derive(Linearize, Debug, PartialEq)]
    enum E {}

    #[allow(unreachable_code)]
    {
        test_enumerated! {
            E:
        }
    }
}

#[test]
fn enum_1() {
    #[derive(Linearize, Debug, PartialEq)]
    enum E {
        A,
        B(bool),
        C(bool, Ordering),
        D { a: bool, b: Ordering },
    }

    test_enumerated! {
        E:
        (E::A),
        (E::B(false)),
        (E::B(true)),
        (E::C(false, Ordering::Less)),
        (E::C(false, Ordering::Equal)),
        (E::C(false, Ordering::Greater)),
        (E::C(true, Ordering::Less)),
        (E::C(true, Ordering::Equal)),
        (E::C(true, Ordering::Greater)),
        (E::D { a: false, b: Ordering::Less }),
        (E::D { a: false, b: Ordering::Equal }),
        (E::D { a: false, b: Ordering::Greater }),
        (E::D { a: true, b: Ordering::Less }),
        (E::D { a: true, b: Ordering::Equal }),
        (E::D { a: true, b: Ordering::Greater }),
    }
}

#[test]
fn enum_single() {
    #[derive(Linearize, Debug, PartialEq)]
    enum E {
        A,
    }

    test_enumerated! {
        E:
        (E::A),
    }
}

#[test]
fn enum_two() {
    #[derive(Linearize, Debug, PartialEq)]
    enum E {
        A,
        B,
    }

    test_enumerated! {
        E:
        (E::A),
        (E::B),
    }
}

#[test]
fn enum_many() {
    #[derive(Linearize, Debug, PartialEq)]
    #[rustfmt::skip]
    enum E {
        A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, A15, A16, A17,
        A18, A19, A20, A21, A22, A23, A24, A25, A26, A27, A28, A29, A30, A31, A32, A33,
        A34, A35, A36, A37, A38, A39, A40, A41, A42, A43, A44, A45, A46, A47, A48, A49,
        A50, A51, A52, A53, A54, A55, A56, A57, A58, A59, A60, A61, A62, A63, A64, A65,
        A66, A67, A68, A69, A70, A71, A72, A73, A74, A75, A76, A77, A78, A79, A80, A81,
        A82, A83, A84, A85, A86, A87, A88, A89, A90, A91, A92, A93, A94, A95, A96, A97,
        A98, A99,
    }

    test_enumerated! {
        E:
        (E::A0), (E::A1), (E::A2), (E::A3), (E::A4), (E::A5), (E::A6), (E::A7), (E::A8), (E::A9), (E::A10),
        (E::A11), (E::A12), (E::A13), (E::A14), (E::A15), (E::A16), (E::A17), (E::A18), (E::A19), (E::A20),
        (E::A21), (E::A22), (E::A23), (E::A24), (E::A25), (E::A26), (E::A27), (E::A28), (E::A29), (E::A30),
        (E::A31), (E::A32), (E::A33), (E::A34), (E::A35), (E::A36), (E::A37), (E::A38), (E::A39), (E::A40),
        (E::A41), (E::A42), (E::A43), (E::A44), (E::A45), (E::A46), (E::A47), (E::A48), (E::A49), (E::A50),
        (E::A51), (E::A52), (E::A53), (E::A54), (E::A55), (E::A56), (E::A57), (E::A58), (E::A59), (E::A60),
        (E::A61), (E::A62), (E::A63), (E::A64), (E::A65), (E::A66), (E::A67), (E::A68), (E::A69), (E::A70),
        (E::A71), (E::A72), (E::A73), (E::A74), (E::A75), (E::A76), (E::A77), (E::A78), (E::A79), (E::A80),
        (E::A81), (E::A82), (E::A83), (E::A84), (E::A85), (E::A86), (E::A87), (E::A88), (E::A89), (E::A90),
        (E::A91), (E::A92), (E::A93), (E::A94), (E::A95), (E::A96), (E::A97), (E::A98), (E::A99),
    }
}

#[test]
fn enum_infallible_1() {
    #[derive(Linearize, PartialEq, Debug)]
    enum E {
        A,
        B,
        C(Infallible),
    }

    test_enumerated! {
        E:
        (E::A),
        (E::B),
    }
}

#[test]
fn enum_infallible_2() {
    #[derive(Linearize, PartialEq, Debug)]
    enum E {
        A,
        C(Infallible),
        B,
    }

    test_enumerated! {
        E:
        (E::A),
        (E::B),
    }
}

#[test]
fn enum_infallible_3() {
    #[derive(Linearize, PartialEq, Debug)]
    enum E {
        C(Infallible),
        A,
        B,
    }

    test_enumerated! {
        E:
        (E::A),
        (E::B),
    }
}
