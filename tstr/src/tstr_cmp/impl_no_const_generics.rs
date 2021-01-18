use crate::for_tupled_reprs::{
    classify::{
        Classify, HasSameKindNumber, Tuple0, Tuple1, Tuple2, Tuple3, Tuple4, Tuple5, Tuple6,
        Tuple7, Tuple8,
    },
    False, True,
};

#[allow(unused_imports)]
use crate::for_tupled_reprs::classify::{
    Chars1, Chars2, Chars3, Chars4, Chars5, Chars6, Chars7, Chars8,
};

#[cfg(not(feature = "min_const_generics"))]
use super::U8Repr;

use super::TStrEq;

pub trait PrivTStrEq<R, EqKind> {
    const EQ: bool;
}
impl<L, R> PrivTStrEq<R, False> for L {
    const EQ: bool = false;
}

pub trait TStrEqTupInner<R, IsByteL, IsByteR> {
    const EQ_INNER: bool;
}
impl<L, R> TStrEqTupInner<R, False, True> for L {
    const EQ_INNER: bool = false;
}
impl<L, R> TStrEqTupInner<R, True, False> for L {
    const EQ_INNER: bool = false;
}

#[cfg(feature = "min_const_generics")]
macro_rules! char_array_impls {
    (
        [$fch:ident $($ch:ident)*],
        [$ofch:ident $($och:ident)*],
        $chars_structs:ident,
        $char_kind_num:ident
    )=>{
        impl<const $fch: char, $(const $ch: char,)*>
            Classify
        for crate::$chars_structs<$fch, $($ch,)*>
        {
            type IsByte = False;
            type KindNumber = $char_kind_num;
        }

        impl<Rhs, DI, const $fch: char, $(const $ch: char,)*>
            TStrEq<Rhs>
        for crate::$chars_structs<$fch, $($ch,)*>
        where
            Self: HasSameKindNumber<Rhs, DoesIt = DI>,
            Self: PrivTStrEq<Rhs, DI>,
        {
            const EQ: bool = <Self as PrivTStrEq<Rhs, DI>>::EQ;
        }

        impl<const $fch: char, $(const $ch: char,)* const $ofch: char, $(const $och: char,)*>
            PrivTStrEq<crate::$chars_structs<$ofch, $($och,)*>, True>
        for crate::$chars_structs<$fch, $($ch,)*>
        {
            const EQ: bool = {
                $fch as u32 == $ofch as u32
                $( && $ch as u32 == $och as u32 )*
            };
        }
    }
}

macro_rules! tuple_impl {
    (
        [$fty:ident $($ty:ident)*],
        [$ofty:ident $($oty:ident)*],
        $tup_kind_num:ident,
        $char_arr_struct:ident,
        $char_kind_num:ident
    ) => (
        #[cfg(feature = "min_const_generics")]
        char_array_impls!{
            [$fty $($ty)*],
            [$ofty $($oty)*],
            $char_arr_struct,
            $char_kind_num
        }

        impl<$fty, $($ty,)*> Classify for ($fty, $($ty,)*) {
            type IsByte = False;
            type KindNumber = $tup_kind_num;
        }

        impl<$fty, $($ty,)* DI, Rhs> TStrEq<Rhs> for ($fty, $($ty,)*)
        where
            Self: HasSameKindNumber<Rhs, DoesIt = DI>,
            Self: PrivTStrEq<Rhs, DI>,
        {
            const EQ: bool = <Self as PrivTStrEq<Rhs, DI>>::EQ;
        }

        impl<$fty, $($ty,)* $ofty, $($oty,)*>
            PrivTStrEq<($ofty, $($oty,)*), True>
        for ($fty, $($ty,)*)
        where
            $fty: Classify,
            $ofty: Classify,
            Self: TStrEqTupInner<($ofty, $($oty,)*), $fty::IsByte, $ofty::IsByte>
        {
            const EQ: bool = Self::EQ_INNER;
        }

        #[cfg(not(feature = "min_const_generics"))]
        impl<$fty, $($ty,)* $ofty, $($oty,)*>
            TStrEqTupInner<($ofty, $($oty,)*), True, True>
        for ($fty, $($ty,)*)
        where
            $fty: U8Repr,
            $ofty: U8Repr,
            $($ty: U8Repr,)*
            $($oty: U8Repr,)*
        {
            const EQ_INNER: bool = {
                $fty::REPR == $ofty::REPR
                $( && $ty::REPR == $oty::REPR )*
            };
        }

        impl<$fty, $($ty,)* $ofty, $($oty,)*>
            TStrEqTupInner<($ofty, $($oty,)*), False, False>
        for ($fty, $($ty,)*)
        where
            $fty: TStrEq<$ofty>,
            $($ty: TStrEq<$oty>,)*
        {
            const EQ_INNER: bool = {
                $fty::EQ
                $( && $ty::EQ )*
            };
        }
    )
}

/*
fn main(){
    for len in 1..=8 {
        print!("tuple_impl! {{ [");
        for i in 0..len {
            print!("L{} ", i)
        }
        print!("], [");
        for i in 0..len {
            print!("R{} ", i);
        }
        println!("], Tuple{} }}", len);
    }
}
*/

tuple_impl! { [L0], [R0], Tuple1, __a, Chars1 }
tuple_impl! { [L0 L1 ], [R0 R1 ], Tuple2, __b, Chars2 }
tuple_impl! { [L0 L1 L2 ], [R0 R1 R2 ], Tuple3, __c, Chars3 }
tuple_impl! { [L0 L1 L2 L3 ], [R0 R1 R2 R3 ], Tuple4, __d, Chars4 }
tuple_impl! { [L0 L1 L2 L3 L4 ], [R0 R1 R2 R3 R4 ], Tuple5, __e, Chars5 }
tuple_impl! { [L0 L1 L2 L3 L4 L5 ], [R0 R1 R2 R3 R4 R5 ], Tuple6, __f, Chars6 }
tuple_impl! { [L0 L1 L2 L3 L4 L5 L6 ], [R0 R1 R2 R3 R4 R5 R6 ], Tuple7, __g, Chars7 }
tuple_impl! { [L0 L1 L2 L3 L4 L5 L6 L7 ], [R0 R1 R2 R3 R4 R5 R6 R7 ], Tuple8, __, Chars8 }

impl Classify for () {
    type IsByte = False;
    type KindNumber = Tuple0;
}

impl<Rhs, DI> TStrEq<Rhs> for ()
where
    Self: HasSameKindNumber<Rhs, DoesIt = DI>,
    Self: PrivTStrEq<Rhs, DI>,
{
    const EQ: bool = <Self as PrivTStrEq<Rhs, DI>>::EQ;
}

impl PrivTStrEq<(), True> for () {
    const EQ: bool = true;
}
