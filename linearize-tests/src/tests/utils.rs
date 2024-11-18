use linearize::Linearize;

macro_rules! impl_assert {
    ($ty:ty) => {
        static_assertions::assert_type_eq_all! {
            <$ty as ::linearize::Linearize>::Storage<u8>,
            <$ty as ::linearize::Linearize>::CopyStorage<u8>,
            [u8; <$ty as ::linearize::Linearize>::LENGTH],
        }
    };
    ($ty:ty, $length:expr) => {
        impl_assert!($ty);

        static_assertions::const_assert_eq! {
            <$ty as ::linearize::Linearize>::LENGTH,
            $length,
        }
    };
}

pub fn test_delinearize<T: Linearize>(_base: &T, v: usize) -> T {
    unsafe { T::from_linear_unchecked(v) }
}

macro_rules! assert_roundtrip {
    ($t:expr) => {{
        let t = $t;
        let l = Linearize::linearize(&t);
        assert_eq!(crate::tests::utils::test_delinearize(&t, l), t);
    }};
    ($t:expr, $l:expr) => {{
        let t = $t;
        let l = Linearize::linearize(&t);
        assert_eq!(l, $l);
        assert_eq!(crate::tests::utils::test_delinearize(&t, l), t);
    }};
}

macro_rules! test_enumerated {
    ($ty:ty: $(($($expr:tt)*),)*) => {{
        use ::linearize::Linearize;
        impl_assert!($ty);
        fn _exhaustive(v: $ty) {
            match v {
                $($($expr)* => {},)*
            }
        }
        let variants: [$ty; <$ty>::LENGTH] = [$($($expr)*),*];
        for (idx, variant) in variants.into_iter().enumerate() {
            assert_roundtrip!(variant, idx);
        }
    }};
}
