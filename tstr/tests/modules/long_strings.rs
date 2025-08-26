use tstr::*;

macro_rules! long_str_test {
    ($string:tt, $chars:ty $(,)*) => {
        test_case!($string, $chars, $string);
    };
}

#[cfg(not(feature = "str_generics"))]
#[allow(dead_code)]
type ZeroToSeven = __<'0', '1', '2', '3', '4', '5', '6', '7'>;

#[cfg(not(feature = "str_generics"))]
#[allow(dead_code)]
type ZeroToSix = __<'0', '1', '2', '3', '4', '5', '6'>;

#[cfg(not(feature = "str_generics"))]
#[allow(dead_code)]
type AToG = __<'a', 'b', 'c', 'd', 'e', 'f', 'g'>;

#[cfg(not(feature = "str_generics"))]
#[allow(dead_code)]
type AToH = __<'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'>;

str_test_case! {"abcdefgh", AToH}
str_test_case! {"01234567", ZeroToSeven}

long_str_test! {"abcdefghi", (AToH, __<'i'>, (), (), (), (), (), ())}

#[cfg(not(feature = "str_generics"))]
#[allow(dead_code)]
type Len56Plus<T> = (AToH, AToH, AToH, AToH, AToH, AToH, AToH, T);

str_test_case! {
    "abcdefgh\
     abcdefgh\
     abcdefgh\
     abcdefgh\
     abcdefgh\
     abcdefgh\
     abcdefgh\
     abcdefg",
    Len56Plus<AToG>,
}

str_test_case! {
    "abcdefgh\
     abcdefgh\
     abcdefgh\
     abcdefgh\
     abcdefgh\
     abcdefgh\
     abcdefgh\
     01234567",
    Len56Plus<ZeroToSeven>,
}

#[cfg(not(feature = "str_generics"))]
#[allow(dead_code)]
type Len504Plus<T> = (
    Len56Plus<ZeroToSeven>,
    Len56Plus<ZeroToSeven>,
    Len56Plus<ZeroToSeven>,
    Len56Plus<ZeroToSeven>,
    Len56Plus<ZeroToSeven>,
    Len56Plus<ZeroToSeven>,
    Len56Plus<ZeroToSeven>,
    Len56Plus<T>,
);

str_test_case! {
    "abcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefgh01234567\
     abcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefgh01234567\
     abcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefgh01234567\
     abcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefgh01234567\
     abcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefgh01234567\
     abcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefgh01234567\
     abcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefgh01234567\
     abcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefgh0123456\
     ",
     Len504Plus<ZeroToSix>
}

str_test_case! {
    "abcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefgh01234567\
     abcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefgh01234567\
     abcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefgh01234567\
     abcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefgh01234567\
     abcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefgh01234567\
     abcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefgh01234567\
     abcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefgh01234567\
     abcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefgh01234567\
     ",
     Len504Plus<ZeroToSeven>
}

long_str_test! {
    "abcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefgh01234567\
     abcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefgh01234567\
     abcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefgh01234567\
     abcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefgh01234567\
     abcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefgh01234567\
     abcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefgh01234567\
     abcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefgh01234567\
     abcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefgh01234567\
     9",
     (Len504Plus<ZeroToSeven>, __<'9'>, (), (), (), (), (), ()),
}

// Just making sure that this module is compiled.
#[test]
fn testing_long_strings() {}
