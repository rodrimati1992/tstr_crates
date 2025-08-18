use crate::{___, __TStrArg};

pub enum Kind {
    Unit,
    Chars,
    Tuple8,
}

macro_rules! with_idents {($len:literal ($($ident:ident)*)) => (
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

    impl<S, const LEN: usize> __TStrArg for ___<S, LEN>
    where
        S: __TStrRepr
    {
        #[doc(hidden)]
        const __LENGTH: usize = LEN;

        #[doc(hidden)]
        const __BYTES: &[u8] = &{
            let mut ret = [0u8; LEN];
            let mut slice: &mut [u8] = &mut ret;
            write_bytes::<S>(&mut slice);
            ret
        };

        #[doc(hidden)]
        const __STR: &str = match core::str::from_utf8(Self::__BYTES) {
            Ok(x) => x,
            Err(_) => unreachable!(),
        };
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

const fn write_char<'a>(this: &mut &'a mut [u8], c: char) {
    if this.is_empty() {
        return;
    }

    let written = c.encode_utf8(this).len();

    let (_, after) = core::mem::replace(this, &mut []).split_at_mut(written);

    *this = after;
}
