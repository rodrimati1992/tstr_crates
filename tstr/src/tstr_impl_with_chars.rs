use crate::{___, __TStrArg, __TStrArgBinary};

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

impl<S1: __TStrRepr, const LEN1: usize, S2: __TStrRepr, const LEN2: usize> __TStrArgBinary
    for (___<S1, LEN1>, ___<S2, LEN2>)
{
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
}

////////////////////////////////////////////////////////////////////////////////

pub enum Kind {
    Unit,
    Chars,
    Tuple8,
}

macro_rules! with_idents {($len:literal ($($ident:ident)*)) => (
    /// private implementation detail
    #[doc(hidden)]
    pub trait __TStrRepr {
        #[doc(hidden)]
        const __KIND: Kind;

        #[doc(hidden)]
        const __CHARS: [char; $len];

        $(
            #[doc(hidden)]
            type $ident: __TStrRepr;
        )*
    }


    impl<S, const LEN: usize> __TStrRepr for ___<S, LEN>
    where
        S: __TStrRepr
    {
        #[doc(hidden)]
        const __KIND: Kind = S::__KIND;

        #[doc(hidden)]
        const __CHARS: [char; $len] = S::__CHARS;

        $(
            #[doc(hidden)]
            type $ident = S::$ident;
        )*
    }

    impl __TStrRepr for () {
        const __KIND: Kind = Kind::Unit;

        const __CHARS: [char; $len] = ['\0'; $len];

        $(type $ident = ();)*
    }

    impl<$(const $ident: char,)*> __TStrRepr for crate::__<$($ident,)*> {
        const __KIND: Kind = Kind::Chars;

        const __CHARS: [char; $len] = [$($ident,)*];

        $(type $ident = ();)*
    }

    impl<$($ident: __TStrRepr,)*> __TStrRepr for ($($ident,)*) {
        const __KIND: Kind = Kind::Tuple8;

        const __CHARS: [char; $len] = ['\0'; $len];

        $(type $ident = $ident;)*
    }
)}

crate::private_macros::with_elem_count_idents! { with_idents!{8} }
