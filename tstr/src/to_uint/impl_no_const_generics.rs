use crate::__;

use super::{Sealed, ToUint};

pub trait PrivToU128 {
    const U128: u128;
    const DIGITS: u32;
}

macro_rules! impl_to_digit {
    ($($ty:ident = $val:tt,)*) => (
        $(
            impl PrivToU128 for crate::$ty {
                const U128: u128 = $val;
                const DIGITS: u32 = 1;
            }
        )*
    )
}

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

const fn ten_pow(power: usize) -> u128 {
    POW_TEN[power]
}

impl<T> Sealed for __<T> where T: PrivToU128 {}

impl<T> ToUint for __<T>
where
    T: PrivToU128,
{
    const U128: u128 = T::U128;
    const DIGITS: u32 = T::DIGITS;
}

macro_rules! tuple_impl {
    ($($ty:ident)*) => (
        #[doc(hidden)]
        impl<$($ty,)*> PrivToU128 for ($($ty,)*)
        where
            $($ty: PrivToU128,)*
        {
            const U128: u128 = {
                #[allow(unused_mut)]
                let mut sum = 0u128;
                $(
                    sum = $ty::U128 + sum * ten_pow($ty::DIGITS as usize);
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
