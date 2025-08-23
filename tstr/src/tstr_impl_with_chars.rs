use crate::{___, __TStrArg, __TStrArgBinary, TStr};

use typewit::{HasTypeWitness, Identity};

mod typecmp_for_chars_repr;
use self::typecmp_for_chars_repr::{__TStrReprBinary, EqKindCmp, NeKindCmp};

impl<S, const LEN: usize> __TStrArg for ___<S, LEN>
where
    S: __TStrRepr + 'static,
{
    #[doc(hidden)]
    const __LENGTH: usize = LEN;

    #[doc(hidden)]
    const __BYTES: &[u8] = &{
        let_bytes! {let ret, S, LEN}
        ret
    };

    #[doc(hidden)]
    const __STR: &str = match core::str::from_utf8(Self::__BYTES) {
        Ok(x) => x,
        Err(_) => unreachable!(),
    };

    #[doc(hidden)]
    type __WithRhs<Rhs: __TStrArg> = <Rhs as __TStrArg>::__WithLhsArgs<S, LEN>;

    #[doc(hidden)]
    type __WithLhsArgs<S1: __TStrRepr, const LEN1: usize> = (___<S1, LEN1>, ___<S, LEN>);
}

macro_rules! let_bytes {
    (let $bytes:ident, $Repr:ty, $LEN:expr) => {
        let mut $bytes = [0u8; $LEN];
        let mut slice: &mut [u8] = &mut $bytes;
        write_bytes::<$Repr>(&mut slice);
    };
}
use let_bytes;

macro_rules! declare_write_bytes{(($($ident:ident)*)) => {
    const fn write_bytes<S: __TStrRepr>(bytes: &mut &mut [u8]) {
        match S::__KIND {
            Kind::Unit => {}
            Kind::Chars => {
                let [$($ident,)*] = S::__CHARS;
                $( write_char(bytes, $ident); )*
            }
            Kind::Tuple8 => {
                $( write_bytes::<S::$ident>(bytes); )*
            }
        }
    }
}}
crate::private_macros::with_elem_count_idents! { declare_write_bytes!{} }

const fn write_char<'a>(this: &mut &'a mut [u8], c: char) {
    if this.is_empty() {
        return;
    }

    let written = c.encode_utf8(this).len();

    let (_, after) = core::mem::replace(this, &mut []).split_at_mut(written);

    *this = after;
}

////////////////////////////////////////////////////////////////////////////////

macro_rules! with_two_encoded {
    (|$lhs:ident, $rhs:ident| -> $ret_ty:ty $block:block) => {{
        const fn func<S1, const LEN1: usize, S2, const LEN2: usize>() -> $ret_ty
        where
            S1: __TStrRepr,
            S2: __TStrRepr,
        {
            let_bytes! {let $lhs, S1, LEN1}
            let_bytes! {let $rhs, S2, LEN2}

            $block
        }
        func::<S1, LEN1, S2, LEN2>()
    }};
}

impl<S1, const LEN1: usize, S2, const LEN2: usize> __TStrArgBinary
    for (___<S1, LEN1>, ___<S2, LEN2>)
where
    S1: __TStrRepr,
    S2: __TStrRepr,
{
    type Lhs = ___<S1, LEN1>;
    type Rhs = ___<S2, LEN2>;

    #[doc(hidden)]
    const __EQ: bool = {
        LEN1 == LEN2
            && with_two_encoded! {
                |lhs, rhs| -> bool { crate::utils::u8_slice_eq(&lhs, &rhs) }
            }
    };

    #[doc(hidden)]
    const __CMP: core::cmp::Ordering = with_two_encoded! {
        |lhs, rhs| -> core::cmp::Ordering { crate::utils::u8_slice_cmp(&lhs, &rhs) }
    };

    #[doc(hidden)]
    const __TYPE_CMP: typewit::TypeCmp<TStr<___<S1, LEN1>>, TStr<___<S2, LEN2>>> = {
        <<___<S1, LEN1> as __TStrRepr>::__WithRhs<___<S2, LEN2>> as __TStrReprBinary>::TYPE_CMP
            .map(crate::tstr_trait::TStrFn)
    };
}

////////////////////////////////////////////////////////////////////////////////

typewit::type_fn! {
    struct TyKindFn;

    impl<R: __TStrRepr> R => R::__Kind;
}

// defined only for `__TStrRepr::__Kind`,
pub enum KArgsWLen {}
pub enum KUnit {}
pub enum KChars {}
pub enum KTuple8 {}

typewit::simple_type_witness! {
    // defined purely for getting a proof of the inequality of
    // two `__TStrRepr`s in `NeKindCmp::TYPE_CMP`
    derive(Equals)
    pub enum KindWitness {
        ArgsWLen = KArgsWLen,
        Unit = KUnit,
        Chars = KChars,
        Tuple8 = KTuple8,
    }
}

pub enum Kind {
    Unit,
    Chars,
    Tuple8,
}

macro_rules! with_idents {($_:tt $len:literal ($($ident:ident)*) ($($identg:ident)*)) => (
    /// private implementation detail
    #[doc(hidden)]
    pub trait __TStrRepr {
        #[doc(hidden)]
        const __KIND: Kind;

        #[doc(hidden)]
        type __Kind: HasTypeWitness<KindWitness<Self::__Kind>>;

        #[doc(hidden)]
        const __CHARS: [char; $len];

        $(
            #[doc(hidden)]
            type $ident: __TStrRepr;
        )*

        #[doc(hidden)]
        type __WithRhs<Rhs: __TStrRepr>: __TStrReprBinary<Lhs = Self, Rhs = Rhs>;

        #[doc(hidden)]
        type __WithLhsArgsWLen<Lhs, S: __TStrRepr, const LEN: usize>
        : __TStrReprBinary<Lhs = Lhs, Rhs = Self>
        where
            Lhs: __TStrRepr + Identity<Type = crate::___<S, LEN>>;

        #[doc(hidden)]
        type __WithLhsArgsUnit<Lhs>
        : __TStrReprBinary<Lhs = Lhs, Rhs = Self>
        where
            Lhs: __TStrRepr + Identity<Type = ()>;

        #[doc(hidden)]
        type __WithLhsArgsChars<Lhs, $(const $identg: char),*>
        : __TStrReprBinary<Lhs = Lhs, Rhs = Self>
        where
            Lhs: __TStrRepr + Identity<Type = crate::__<$($identg,)*>>;

        #[doc(hidden)]
        type __WithLhsArgsTuple8<Lhs, $($identg: __TStrRepr),*>
        : __TStrReprBinary<Lhs = Lhs, Rhs = Self>
        where
            Lhs: __TStrRepr + Identity<Type = ($($identg,)*)>;
    }

    macro_rules! with_reprargs_assoc_types {
        (
            ArgsWLen = $SelfWL:ty,
            ArgsUnit = $SelfU:ty,
            ArgsChars = $SelfC:ty,
            ArgsTuple8 = $SelfT8:ty $_(,)?
        ) => {
            #[doc(hidden)]
            type __WithLhsArgsWLen<Lhs, LS: __TStrRepr, const LLEN: usize> = $SelfWL
            where
                Lhs: __TStrRepr + Identity<Type = crate::___<LS, LLEN>>;

            #[doc(hidden)]
            type __WithLhsArgsUnit<Lhs> = $SelfU
            where
                Lhs: __TStrRepr + Identity<Type = ()>;

            #[doc(hidden)]
            type __WithLhsArgsChars<Lhs, $(const $identg: char),*> = $SelfC
            where
                Lhs: __TStrRepr + Identity<Type = crate::__<$($identg,)*>>;

            #[doc(hidden)]
            type __WithLhsArgsTuple8<Lhs, $($identg: __TStrRepr),*> = $SelfT8
            where
                Lhs: __TStrRepr + Identity<Type = ($($identg,)*)>;

        }
    }


    impl<S, const LEN: usize> __TStrRepr for ___<S, LEN>
    where
        S: __TStrRepr
    {
        #[doc(hidden)]
        const __KIND: Kind = S::__KIND;

        type __Kind = KArgsWLen;

        #[doc(hidden)]
        const __CHARS: [char; $len] = S::__CHARS;

        $(
            #[doc(hidden)]
            type $ident = S::$ident;
        )*

        type __WithRhs<Rhs: __TStrRepr> = Rhs::__WithLhsArgsWLen<Self, S, LEN>;

        with_reprargs_assoc_types!{
            ArgsWLen = EqKindCmp<crate::___<LS, LLEN>, Lhs, Self>,
            ArgsUnit = NeKindCmp<Lhs, Self>,
            ArgsChars = NeKindCmp<Lhs, Self>,
            ArgsTuple8 = NeKindCmp<Lhs, Self>,
        }
    }

    impl __TStrRepr for () {
        const __KIND: Kind = Kind::Unit;

        type __Kind = KUnit;

        const __CHARS: [char; $len] = ['\0'; $len];

        $(type $ident = ();)*

        type __WithRhs<Rhs: __TStrRepr> = Rhs::__WithLhsArgsUnit<Self>;

        with_reprargs_assoc_types!{
            ArgsWLen = NeKindCmp<Lhs, Self>,
            ArgsUnit = EqKindCmp<(), Lhs, Self>,
            ArgsChars = NeKindCmp<Lhs, Self>,
            ArgsTuple8 = NeKindCmp<Lhs, Self>,
        }
    }

    impl<$(const $ident: char,)*> __TStrRepr for crate::__<$($ident,)*> {
        const __KIND: Kind = Kind::Chars;

        type __Kind = KChars;

        const __CHARS: [char; $len] = [$($ident,)*];

        $(type $ident = ();)*

        type __WithRhs<Rhs: __TStrRepr> = Rhs::__WithLhsArgsChars<Self, $($ident,)*>;

        with_reprargs_assoc_types!{
            ArgsWLen = NeKindCmp<Lhs, Self>,
            ArgsUnit = NeKindCmp<Lhs, Self>,
            ArgsChars = EqKindCmp<crate::__<$($identg,)*>, Lhs, Self>,
            ArgsTuple8 = NeKindCmp<Lhs, Self>,
        }
    }

    impl<$($ident: __TStrRepr,)*> __TStrRepr for ($($ident,)*) {
        const __KIND: Kind = Kind::Tuple8;

        type __Kind = KTuple8;

        const __CHARS: [char; $len] = ['\0'; $len];

        $(type $ident = $ident;)*

        type __WithRhs<Rhs: __TStrRepr> = Rhs::__WithLhsArgsTuple8<Self, $($ident,)*>;

        with_reprargs_assoc_types!{
            ArgsWLen = NeKindCmp<Lhs, Self>,
            ArgsUnit = NeKindCmp<Lhs, Self>,
            ArgsChars = NeKindCmp<Lhs, Self>,
            ArgsTuple8 = EqKindCmp<($($identg,)*), Lhs, Self>,
        }
    }
)}

crate::private_macros::with_elem_count_idents2! { with_idents!{$ 8} }
