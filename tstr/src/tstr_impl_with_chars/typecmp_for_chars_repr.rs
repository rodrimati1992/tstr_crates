use super::{__TStrRepr, KindWitness, TyKindFn};

use typewit::{
    HasTypeWitness, Identity, TypeCmp,
    const_marker::{Char, Usize},
};

pub trait __TStrReprBinaryArgs {
    type Lhs: __TStrRepr;
    type Rhs: __TStrRepr;
}

pub trait __TStrReprBinary: __TStrReprBinaryArgs {
    const TYPE_CMP: TypeCmp<Self::Lhs, Self::Rhs>;
}

//////////////////////////////////////////////////////////////////

pub struct EqKindCmp<LhsDetail, Lhs, Rhs>(LhsDetail, Lhs, Rhs);

impl<LhsDetail, Lhs, Rhs> __TStrReprBinaryArgs for EqKindCmp<LhsDetail, Lhs, Rhs>
where
    Lhs: __TStrRepr,
    Rhs: __TStrRepr,
{
    type Lhs = Lhs;
    type Rhs = Rhs;
}

typewit::inj_type_fn! {
    struct ArgsWLenFn;

    impl<S, const LEN: usize> (S, Usize<LEN>) => crate::___<S, LEN>
}

impl<Lhs, LS, const LLEN: usize, RS, const RLEN: usize> __TStrReprBinary
    for EqKindCmp<crate::___<LS, LLEN>, Lhs, crate::___<RS, RLEN>>
where
    Lhs: __TStrRepr + Identity<Type = crate::___<LS, LLEN>>,
    LS: __TStrRepr,
    RS: __TStrRepr,
{
    const TYPE_CMP: TypeCmp<Self::Lhs, Self::Rhs> = {
        TypeCmp::zip(
            <LS::__WithRhs<RS>>::TYPE_CMP,
            Usize::<LLEN>.equals(Usize::<RLEN>),
        )
        .map(ArgsWLenFn)
        .join_left(Lhs::TYPE_EQ)
    };
}

///////

impl<Lhs> __TStrReprBinary for EqKindCmp<(), Lhs, ()>
where
    Lhs: __TStrRepr + Identity<Type = ()>,
{
    const TYPE_CMP: TypeCmp<Self::Lhs, Self::Rhs> = Lhs::TYPE_EQ.to_cmp();
}

macro_rules! with_idents_inner {
    (
        ($($ident:ident)*) ($($identg:ident)*);
        $(
            ($l0:ident $r0:ident)
            ($l1:ident $r1:ident)
            ($l2:ident $r2:ident)
            ($l3:ident $r3:ident)
        )*
    ) => (
        typewit::inj_type_fn!{
            struct CharsFn;

            impl<$(const $ident: char,)*> ($((Char<$l0>, Char<$l1>, Char<$l2>, Char<$l3>),)*)
            => crate::__<$($ident,)*>
        }

        impl<Lhs, $(const $ident: char,)* $(const $identg: char,)*>
            __TStrReprBinary
        for EqKindCmp<crate::__<$($ident,)*>, Lhs, crate::__<$($identg,)*>>
        where
            Lhs: __TStrRepr + Identity<Type = crate::__<$($ident,)*>>,
        {
            const TYPE_CMP: TypeCmp<Self::Lhs, Self::Rhs> = {
                TypeCmp::zip(
                    $(
                        TypeCmp::zip4(
                            Char::<$l0>.equals(Char::<$r0>),
                            Char::<$l1>.equals(Char::<$r1>),
                            Char::<$l2>.equals(Char::<$r2>),
                            Char::<$l3>.equals(Char::<$r3>),
                        ),
                    )*
                ).map(CharsFn)
                .join_left(Lhs::TYPE_EQ)
            };
        }

        /////////////////////////////////////////////////////////////////////

        typewit::inj_type_fn!{
            struct Tuple8Fn;

            impl<$($ident: __TStrRepr,)*> ($(($l0, $l1, $l2, $l3),)*) => ($($ident,)*)
        }

        impl<Lhs, $($ident: __TStrRepr,)* $($identg: __TStrRepr,)*>
            __TStrReprBinary
        for EqKindCmp<($($ident,)*), Lhs, ($($identg,)*)>
        where
            Lhs: __TStrRepr + Identity<Type = ($($ident,)*)>,
        {
            const TYPE_CMP: TypeCmp<Self::Lhs, Self::Rhs> = {
                TypeCmp::zip(
                    $(
                        TypeCmp::zip4(
                            <$l0::__WithRhs<$r0>>::TYPE_CMP,
                            <$l1::__WithRhs<$r1>>::TYPE_CMP,
                            <$l2::__WithRhs<$r2>>::TYPE_CMP,
                            <$l3::__WithRhs<$r3>>::TYPE_CMP,
                        ),
                    )*
                ).map(Tuple8Fn)
                .join_left(Lhs::TYPE_EQ)
            };
        }
    )
}

macro_rules! with_idents {(($($ident:ident)*) ($($identg:ident)*)) => (
    with_idents_inner!{
        ($($ident)*) ($($identg)*);
        $(($ident $identg))*
    }
)}

crate::private_macros::with_elem_count_idents2! { with_idents!{} }

//////////////////////////////////////////////////////////////////

/// Helper for getting a proof that Lhs != Rhs
pub struct NeKindCmp<Lhs, Rhs>(Lhs, Rhs);

impl<Lhs, Rhs> __TStrReprBinaryArgs for NeKindCmp<Lhs, Rhs>
where
    Lhs: __TStrRepr,
    Rhs: __TStrRepr,
{
    type Lhs = Lhs;
    type Rhs = Rhs;
}

impl<Lhs, Rhs> __TStrReprBinary for NeKindCmp<Lhs, Rhs>
where
    Lhs: __TStrRepr,
    Rhs: __TStrRepr,
{
    const TYPE_CMP: TypeCmp<Self::Lhs, Self::Rhs> = {
        let lhs: KindWitness<Lhs::__Kind> = <Lhs::__Kind>::WITNESS;
        let rhs: KindWitness<Rhs::__Kind> = <Rhs::__Kind>::WITNESS;

        // `unwrap_ne` doesn't panic because all uses of this make sure that
        // the two type arguments have different kinds.
        lhs.equals(rhs).unwrap_ne().map_to_arg(TyKindFn).to_cmp()
    };
}

//////////////////////////////////////////////////////////////////
