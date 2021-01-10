mod sealed {
    #[doc(hidden)]
    pub trait Sealed: Sized {}
}
use sealed::Sealed;

/// Converts a [`TStr`](./struct.TStr) to a usize.
pub trait ToUint: Sealed {
    /// The `usize` value of the type.
    ///
    /// By default this value is a saturated cast from `Self::U128`.
    const USIZE: usize = {
        const MAXU: u128 = usize::max_value() as u128;
        [Self::U128, MAXU][(Self::U128 > MAXU) as usize] as usize
    };

    /// The `u128` value of the type.
    const U128: u128;

    #[doc(hidden)]
    const DIGITS: u32;

    /// Gets the usize value of this type
    fn to_usize(self) -> usize {
        Self::USIZE
    }

    /// Gets the u128 value of this type
    fn to_u128(self) -> u128 {
        Self::U128
    }
}

#[cfg(feature = "const_generics")]
macro_rules! impl_for_const {
    () => {
        const fn str_to_u128(s: &str) -> u128 {
            let mut out = 0u128;
            let mut index = 0usize;

            while index < s.len() {
                let byte = s[index];

                // This has the effect of panicking on non to '0' to '9' characters.
                #[allow(clippy::no_effect)]
                ["Expected all characters to be digits"][(b'0' <= digit && digit <= b'9') as usize];

                let digit = (digit - b'0') as u128;
                out = out * 10 + digit;

                index += 1;
            }
            out
        }

        impl<const N: usize> Sealed for TStr<crate::__<S>> {}

        impl<const N: usize> ToUint for TStr<crate::__<S>> {
            const USIZE: usize;

            /// Gets the usize value of this type
            fn to_usize(&self) -> usize {
                Self::USIZE
            }
        }
    };
}

#[cfg(feature = "const_generics")]
impl_for_const! {}

#[cfg(not(feature = "const_generics"))]
mod impl_no_const_generics;
