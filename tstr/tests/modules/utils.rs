#[cfg(feature = "const_generics")]
macro_rules! test_case {
    ($input:tt, $tytup:ty, $chars:ty, $string:expr $(,)*) => {
        const _: tstr::TStr<tstr::___<$string>> = ts!($input);
    };
}

#[cfg(all(feature = "min_const_generics", not(feature = "const_generics")))]
macro_rules! test_case {
    ($input:tt, $tytup:ty, $chars:ty, $string:expr $(,)*) => {
        const _: tstr::TStr<$chars> = ts!($input);
    };
}

#[cfg(not(feature = "min_const_generics"))]
macro_rules! test_case {
    ($input:tt, $tytup:ty, $chars:ty, $string:expr $(,)*) => {
        const _: tstr::TStr<$tytup> = ts!($input);
    };
}
