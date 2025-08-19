macro_rules! assert_equals_string {
    ($tstr_ty:ty, $string:expr) => {
        const _: () = {
            const X: &[u8] = $string.as_bytes();
            assert!(matches!(<$tstr_ty>::new().to_str().as_bytes(), X));
            assert!(matches!(<$tstr_ty>::new().to_bytes(), X));
        };
    };
}

#[cfg(feature = "str_generics")]
macro_rules! test_case {
    ($input:tt, $chars:ty, $string:expr $(,)*) => {
        const _: tstr::TStr<tstr::___<$string>> = ts!($input);
        assert_equals_string! {tstr::TStr<tstr::___<$string>>, $string}
    };
}

#[cfg(not(feature = "str_generics"))]
macro_rules! test_case {
    ($input:tt, $chars:ty, $string:expr $(,)*) => {
        const _: tstr::TStr<___<$chars, { $string.len() }>> = ts!($input);
        assert_equals_string! {tstr::TStr<___<$chars, { $string.len() }>>, $string}
    };
}

macro_rules! str_test_case {
    ($string:tt, $tuple:ty $(,)*) => {
        test_case!($string, $tuple, $string);
    };
}
