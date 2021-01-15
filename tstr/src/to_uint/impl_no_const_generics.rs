use super::{Sealed, ToUint};

#[cfg(not(feature = "min_const_generics"))]
macro_rules! impl_to_digit {
    ($($ty:ident = $val:tt,)*) => (
        $(
            impl Sealed for crate::$ty {}

            impl ToUint for crate::$ty {
                const U128: u128 = $val;
                const USIZE: usize = $val;
                const DIGITS: u32 = 1;
            }
        )*
    )
}

#[cfg(not(feature = "min_const_generics"))]
impl_to_digit! {
    __0 = 0,
    __1 = 1,
    __2 = 2,
    __3 = 3,
    __4 = 4,
    __5 = 5,
    __6 = 6,
    __7 = 7,
    __8 = 8,
    __9 = 9,
}

#[cfg(feature = "min_const_generics")]
macro_rules! impl_to_digits_const {
    (
        [$($digit:literal => $value:literal,)*]
        [
            $( ($chars_structs:ident, [$($chars:ident),*], $len:expr) ,)*
        ]
    )=>{
        use crate::__a;

        $(
            impl Sealed for __a<$digit> {}

            impl ToUint for __a<$digit> {
                const USIZE: usize = $value;
                const U128: u128 = $value;
                const DIGITS: u32 = 1;
            }
        )*

        $(
            impl<$(const $chars: char,)*> Sealed for crate::$chars_structs<$($chars,)*> {}

            impl<$(const $chars: char,)*> ToUint for crate::$chars_structs<$($chars,)*>
            where
                $(__a<$chars>: ToUint,)*
            {
                const U128: u128 = {
                    #[allow(unused_mut)]
                    let mut sum = 0u128;
                    $(
                        sum = __a::<$chars>::U128 + sum * ten_pow(__a::<$chars>::DIGITS);
                    )*
                    sum
                };
                const DIGITS: u32 = $len;
            }
        )*
    }
}

#[cfg(feature = "min_const_generics")]
impl_to_digits_const! {
    [
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
    ]
    [
        (__b, [A,B], 2),
        (__c, [A,B,C], 3),
        (__d, [A,B,C,D], 4),
        (__e, [A,B,C,D,E], 5),
        (__f, [A,B,C,D,E,F], 6),
        (__g, [A,B,C,D,E,F,G], 7),
        (__ , [A,B,C,D,E,F,G,H], 8),
    ]
}

/*
fn main(){
    let mut accum = 1u128;
    println!("    {},", accum);
    while let Some(next) = accum.checked_mul(10) {
        println!("    {},", next);
        accum = next;
    }
}
*/
const POW_TEN: &[u128; 39] = &[
    1,
    10,
    100,
    1000,
    10000,
    100000,
    1000000,
    10000000,
    100000000,
    1000000000,
    10000000000,
    100000000000,
    1000000000000,
    10000000000000,
    100000000000000,
    1000000000000000,
    10000000000000000,
    100000000000000000,
    1000000000000000000,
    10000000000000000000,
    100000000000000000000,
    1000000000000000000000,
    10000000000000000000000,
    100000000000000000000000,
    1000000000000000000000000,
    10000000000000000000000000,
    100000000000000000000000000,
    1000000000000000000000000000,
    10000000000000000000000000000,
    100000000000000000000000000000,
    1000000000000000000000000000000,
    10000000000000000000000000000000,
    100000000000000000000000000000000,
    1000000000000000000000000000000000,
    10000000000000000000000000000000000,
    100000000000000000000000000000000000,
    1000000000000000000000000000000000000,
    10000000000000000000000000000000000000,
    100000000000000000000000000000000000000,
];

const fn ten_pow(power: u32) -> u128 {
    POW_TEN[power as usize]
}

macro_rules! tuple_impl {
    ($($ty:ident)*) => (
        impl<$($ty,)*> Sealed for ($($ty,)*)
        where
            $($ty: Sealed,)*
        {}

        #[doc(hidden)]
        impl<$($ty,)*> ToUint for ($($ty,)*)
        where
            $($ty: ToUint,)*
        {
            const U128: u128 = {
                #[allow(unused_mut)]
                let mut sum = 0u128;
                $(
                    sum = $ty::U128 + sum * ten_pow($ty::DIGITS);
                )*
                sum
            };
            const DIGITS: u32 = 0 $( + $ty::DIGITS )*;
        }
    )
}

tuple_impl! {}
tuple_impl! {A }
tuple_impl! {A B}
tuple_impl! {A B C}
tuple_impl! {A B C D}
tuple_impl! {A B C D E}
tuple_impl! {A B C D E F}
tuple_impl! {A B C D E F G}
tuple_impl! {A B C D E F G H}
