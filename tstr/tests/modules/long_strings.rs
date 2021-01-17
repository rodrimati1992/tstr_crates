use tstr::*;

macro_rules! long_str_test {
    ($string:tt, $tuple:ty, $chars:ty $(,)*) => {
        test_case!($string, $tuple, $chars, $string);
    };
}

#[cfg(not(feature = "min_const_generics"))]
#[allow(dead_code)]
type ZeroToSeven = (__0, __1, __2, __3, __4, __5, __6, __7);

#[cfg(all(feature = "min_const_generics", not(feature = "const_generics")))]
#[allow(dead_code)]
type ZeroToSeven = __<'0', '1', '2', '3', '4', '5', '6', '7'>;

#[cfg(not(feature = "min_const_generics"))]
#[allow(dead_code)]
type ZeroToSix = (__0, __1, __2, __3, __4, __5, __6);

#[cfg(all(feature = "min_const_generics", not(feature = "const_generics")))]
#[allow(dead_code)]
type ZeroToSix = __g<'0', '1', '2', '3', '4', '5', '6'>;

#[cfg(not(feature = "min_const_generics"))]
#[allow(dead_code)]
type AToG = (__a, __b, __c, __d, __e, __f, __g);

#[cfg(all(feature = "min_const_generics", not(feature = "const_generics")))]
#[allow(dead_code)]
type AToG = __g<'a', 'b', 'c', 'd', 'e', 'f', 'g'>;

#[cfg(not(feature = "min_const_generics"))]
#[allow(dead_code)]
type AToH = (__a, __b, __c, __d, __e, __f, __g, __h);

#[cfg(all(feature = "min_const_generics", not(feature = "const_generics")))]
#[allow(dead_code)]
type AToH = __<'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'>;

str_test_case! {"abcdefgh", AToH}
str_test_case! {"01234567", ZeroToSeven}

long_str_test! {"abcdefghi", (AToH, (__i,)), (AToH, __a<'i'>)}

#[cfg(not(feature = "const_generics"))]
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

#[cfg(not(feature = "const_generics"))]
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
     (Len504Plus<ZeroToSeven>, (__9,)),
     (Len504Plus<ZeroToSeven>, __a<'9'>),
}

// Just making sure that this module is compiled.
#[test]
fn testing_long_strings() {}
