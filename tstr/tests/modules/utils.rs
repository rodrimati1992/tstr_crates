#[cfg(feature = "str_generics")]
macro_rules! test_case {
    ($input:tt, $chars:ty, $string:expr $(,)*) => {
        const _: tstr::TStr<tstr::___<$string>> = ts!($input);
    };
}

#[cfg(not(feature = "str_generics"))]
macro_rules! test_case {
    ($input:tt, $chars:ty, $string:expr $(,)*) => {
        const _: tstr::TStr<___<$chars, { $string.len() }>> = ts!($input);
    };
}

macro_rules! str_test_case {
    ($string:tt, $tuple:ty $(,)*) => {
        test_case!($string, $tuple, $string);
    };
}
