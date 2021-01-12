#[cfg(feature = "const_generics")]
macro_rules! test_case {
    ($input:tt, $chars:ty, $string:expr $(,)*) => {
        const _: tstr::TStr<tstr::__<$string>> = ts!($input);
    };
}

#[cfg(not(feature = "const_generics"))]
macro_rules! test_case {
    ($input:tt, $chars:ty, $string:expr $(,)*) => {
        const _: tstr::TStr<tstr::__<$chars>> = ts!($input);
    };
}
