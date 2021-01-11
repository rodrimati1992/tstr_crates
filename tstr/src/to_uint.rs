mod sealed {
    #[doc(hidden)]
    pub trait Sealed: Sized {}
}
use sealed::Sealed;

/// Converts a [`TStr`] to a usize.
///
/// This trait is sealed, so it can't be implemented outside the `tstr` crate.
///
/// # Example
///
/// ```rust
/// use tstr::{ToUint, TS, ts};
///
/// type Zero = TS!(0);
/// type N8   = TS!(8);
/// type N13  = TS!(13);
/// type N15  = TS!(0xF);
/// type N16  = TS!(0b10000);
///
/// assert_eq!(Zero::USIZE, 0);
/// assert_eq!(N8::USIZE, 8);
/// assert_eq!(N13::USIZE, 13);
/// assert_eq!(N15::USIZE, 15);
/// assert_eq!(N16::USIZE, 16);
///
/// assert_eq!(ts!(0).to_u128(), 0);
/// assert_eq!(ts!(8).to_u128(), 8);
/// assert_eq!(ts!(13).to_u128(), 13);
/// assert_eq!(ts!(0xF).to_u128(), 15);
/// assert_eq!(ts!(0b10000).to_u128(), 16);
///
/// ```
///
/// [`TStr`]: ./struct.TStr.html
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
            let s = s.as_bytes();
            let mut out = 0u128;
            let mut index = 0usize;

            while index < s.len() {
                let digit = s[index];

                // This has the effect of panicking on non to '0' to '9' characters.
                #[allow(clippy::no_effect)]
                ["Expected all characters to be digits"]
                    [!(b'0' <= digit && digit <= b'9') as usize];

                let digit = (digit - b'0') as u128;
                out = out * 10 + digit;

                index += 1;
            }
            out
        }

        impl<const N: &'static str> Sealed for crate::TStr<crate::__<N>> {}

        impl<const N: &'static str> ToUint for crate::TStr<crate::__<N>> {
            const U128: u128 = str_to_u128(N);
            const DIGITS: u32 = N.len() as u32;
        }
    };
}

#[cfg(feature = "const_generics")]
impl_for_const! {}

#[cfg(not(feature = "const_generics"))]
mod impl_no_const_generics;
