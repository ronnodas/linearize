use linearize::{static_map, Linearize, StaticMap};

#[test]
fn binding() {
    let map: StaticMap<bool, _> = static_map! {
        v => Box::new(v as u32),
    };
    assert_eq!(*map[false], 0);
    assert_eq!(*map[true], 1);
}

#[test]
fn given() {
    let map: StaticMap<bool, _> = static_map! {
        false => Box::new(0),
        true => Box::new(1),
    };
    assert_eq!(*map[false], 0);
    assert_eq!(*map[true], 1);
}

#[test]
#[cfg(more_const_functions)]
fn of_type_binding() {
    #[derive(Linearize)]
    #[linearize(const)]
    enum L {
        False,
        True,
    }
    let map: StaticMap<L, _> = static_map! {
        of type L:
        v => Box::new(v as u32),
    };
    assert_eq!(*map[L::False], 0);
    assert_eq!(*map[L::True], 1);
}

#[test]
#[cfg(more_const_functions)]
fn of_type_given() {
    #[derive(Linearize)]
    #[linearize(const)]
    enum L {
        False,
        True,
    }
    let map: StaticMap<L, _> = static_map! {
        of type L:
        L::False => Box::new(0),
        L::True => Box::new(1),
    };
    assert_eq!(*map[L::False], 0);
    assert_eq!(*map[L::True], 1);
}

#[test]
#[cfg(more_const_functions)]
fn constants_of_type_given() {
    #[derive(Linearize)]
    #[linearize(const)]
    enum L {
        False,
        True,
    }
    let map: StaticMap<L, _> = static_map! {
        constants of type L:
        L::False => Box::new(0),
        L::True => Box::new(1),
    };
    assert_eq!(*map[L::False], 0);
    assert_eq!(*map[L::True], 1);
}

mod copy_macro {
    use linearize::{static_copy_map, Linearize, StaticCopyMap};

    #[test]
    fn test() {
        let _: StaticCopyMap<bool, u8> = static_copy_map! {
            false => 1,
            _ => 1,
        };
    }

    #[test]
    #[cfg(more_const_functions)]
    fn test_constant() {
        #[derive(Linearize)]
        #[linearize(const)]
        enum L {
            False,
            True,
        }
        let _: StaticCopyMap<L, u8> = static_copy_map! {
            of type L:
            _ => 1,
        };
        let _: StaticCopyMap<L, u8> = static_copy_map! {
            constants of type L:
            L::True => 1,
            L::False => 1,
        };
    }
}
