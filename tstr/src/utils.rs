//! Utility functions

/// A const equivalent of `&str` equality comparison.
///
/// # Example
///
/// ```rust
/// use tstr::utils::str_eq;
///
/// const FOO: &str = "foo";
/// const BAR: &str = "fooooo";
/// const BAZ: &str = "bar";
///
///
/// const FOO_EQ_FOO: bool = str_eq(FOO, FOO);
/// assert!( FOO_EQ_FOO );
///
/// const FOO_EQ_BAR: bool = str_eq(FOO, BAR);
/// assert!( !FOO_EQ_BAR );
///
/// const FOO_EQ_BAZ: bool = str_eq(FOO, BAZ);
/// assert!( !FOO_EQ_BAZ );
///
/// ```
///
#[cfg(feature = "rust_1_46")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "rust_1_46")))]
#[inline]
pub const fn str_eq(left: &str, right: &str) -> bool {
    u8_slice_eq(left.as_bytes(), right.as_bytes())
}

/// A const equivalent of `&[u8]` equality comparison.
///
/// # Example
///
/// ```rust
/// use tstr::utils::u8_slice_eq;
///
/// const FOO: &[u8] = &[10, 20];
/// const BAR: &[u8] = &[10, 20, 30, 40];
/// const BAZ: &[u8] = &[3, 5, 8, 13];
///
/// const FOO_EQ_FOO: bool = u8_slice_eq(FOO, FOO);
/// assert!( FOO_EQ_FOO );
///
/// const FOO_EQ_BAR: bool = u8_slice_eq(FOO, BAR);
/// assert!( !FOO_EQ_BAR );
///
/// const FOO_EQ_BAZ: bool = u8_slice_eq(FOO, BAZ);
/// assert!( !FOO_EQ_BAZ );
///
///
/// ```
///
#[cfg(feature = "rust_1_46")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "rust_1_46")))]
#[inline]
pub const fn u8_slice_eq(left: &[u8], right: &[u8]) -> bool {
    if left.len() != right.len() {
        return false;
    }

    let mut i = 0;
    while i != left.len() {
        if left[i] != right[i] {
            return false;
        }
        i += 1;
    }

    true
}

#[cfg(feature = "rust_1_46")]
pub use slice_cmp::{str_cmp, u8_slice_cmp};

#[cfg(feature = "rust_1_46")]
mod slice_cmp {
    use core::cmp::Ordering;

    const LESS: u8 = 0;
    const GREATER: u8 = 1;
    const EQUAL: u8 = 2;

    macro_rules! ret_if_ne {
        ($left:expr, $right:expr) => {{
            let l = $left;
            let r = $right;
            if l != r {
                return (l > r) as u8;
            }
        }};
    }

    const fn to_ordering(n: u8) -> Ordering {
        match n {
            LESS => Ordering::Less,
            GREATER => Ordering::Greater,
            _ => Ordering::Equal,
        }
    }

    /// A const equivalent of `str::cmp`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use tstr::utils::str_cmp;
    ///
    /// use std::cmp::Ordering;
    ///
    /// const FOO: &str = "foo";
    /// const BAR: &str = "fooooo";
    /// const BAZ: &str = "bar";
    ///
    ///
    /// const FOO_CMP_FOO: Ordering = str_cmp(FOO, FOO);
    /// assert_eq!(FOO_CMP_FOO, Ordering::Equal);
    ///
    /// const FOO_CMP_BAR: Ordering = str_cmp(FOO, BAR);
    /// assert_eq!(FOO_CMP_BAR, Ordering::Less);
    ///
    /// const FOO_CMP_BAZ: Ordering = str_cmp(FOO, BAZ);
    /// assert_eq!(FOO_CMP_BAZ, Ordering::Greater);
    ///
    /// ```
    ///
    #[cfg_attr(feature = "docsrs", doc(cfg(feature = "rust_1_46")))]
    #[inline]
    pub const fn str_cmp(left: &str, right: &str) -> Ordering {
        const fn str_cmp_inner(left: &[u8], right: &[u8]) -> u8 {
            let left_len = left.len();
            let right_len = right.len();
            let (min_len, on_ne) = if left_len < right_len {
                (left_len, LESS)
            } else {
                (right_len, GREATER)
            };

            let mut i = 0;
            while i < min_len {
                ret_if_ne! {left[i], right[i]}
                i += 1;
            }

            if left_len == right_len {
                EQUAL
            } else {
                on_ne
            }
        }

        to_ordering(str_cmp_inner(left.as_bytes(), right.as_bytes()))
    }

    /// A const equivalent of `<[u8]>::cmp`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use tstr::utils::u8_slice_cmp;
    ///
    /// use std::cmp::Ordering;
    ///
    /// const FOO: &[u8] = &[10, 20];
    /// const BAR: &[u8] = &[10, 20, 30, 40];
    /// const BAZ: &[u8] = &[3, 5];
    ///
    /// const FOO_CMP_FOO: Ordering = u8_slice_cmp(FOO, FOO);
    /// assert_eq!(FOO_CMP_FOO, Ordering::Equal);
    ///
    /// const FOO_CMP_BAR: Ordering = u8_slice_cmp(FOO, BAR);
    /// assert_eq!(FOO_CMP_BAR, Ordering::Less);
    ///
    /// const FOO_CMP_BAZ: Ordering = u8_slice_cmp(FOO, BAZ);
    /// assert_eq!(FOO_CMP_BAZ, Ordering::Greater);
    ///
    /// ```
    ///
    #[cfg_attr(feature = "docsrs", doc(cfg(feature = "rust_1_46")))]
    #[inline]
    pub const fn u8_slice_cmp(left: &[u8], right: &[u8]) -> Ordering {
        const fn u8_slice_cmp_inner(left: &[u8], right: &[u8]) -> u8 {
            let left_len = left.len();

            ret_if_ne! {left_len, right.len()}

            let mut i = 0;
            while i < left_len {
                ret_if_ne! {left[i], right[i]}
                i += 1;
            }

            EQUAL
        }

        to_ordering(u8_slice_cmp_inner(left, right))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "rust_1_46")]
    fn slice_eq_test() {
        assert!(u8_slice_eq(&[], &[]));
        assert!(!u8_slice_eq(&[], &[0]));
        assert!(!u8_slice_eq(&[0], &[]));
        assert!(u8_slice_eq(&[0], &[0]));
        assert!(!u8_slice_eq(&[0], &[1]));
        assert!(!u8_slice_eq(&[1], &[0]));
        assert!(!u8_slice_eq(&[0], &[0, 1]));
        assert!(!u8_slice_eq(&[0, 1], &[0]));
        assert!(u8_slice_eq(&[0, 1], &[0, 1]));
        assert!(!u8_slice_eq(&[0, 1], &[0, 2]));
    }

    #[test]
    #[cfg(feature = "rust_1_46")]
    fn str_eq_test() {
        assert!(str_eq("", ""));
        assert!(!str_eq("", "0"));
        assert!(!str_eq("0", ""));
        assert!(str_eq("0", "0"));
        assert!(!str_eq("0", "1"));
        assert!(!str_eq("1", "0"));
        assert!(!str_eq("0", "0, 1"));
        assert!(!str_eq("0, 1", "0"));
        assert!(!str_eq("0, 1", "1"));
        assert!(str_eq("0, 1", "0, 1"));
        assert!(!str_eq("0, 1", "0, 2"));
    }

    #[test]
    #[cfg(feature = "rust_1_46")]
    fn slice_cmp_test() {
        use core::cmp::{
            Ord,
            Ordering::{Equal, Greater, Less},
        };

        macro_rules! assert_s_cmp {
            ($left:expr, $right:expr, $expected:expr) => {
                assert_eq!(u8_slice_cmp($left, $right), $expected);
                assert_eq!(<[u8]>::cmp($left, $right), $expected);

                assert_eq!(u8_slice_cmp($right, $left), $expected.reverse());
                assert_eq!(<[u8]>::cmp($right, $left), $expected.reverse());
            };
        }

        assert_s_cmp!(&[], &[], Equal);
        assert_s_cmp!(&[], &[0], Less);
        assert_s_cmp!(&[0], &[], Greater);
        assert_s_cmp!(&[0], &[0], Equal);
        assert_s_cmp!(&[0], &[1], Less);
        assert_s_cmp!(&[0], &[0, 1], Less);
        assert_s_cmp!(&[0, 1], &[0, 1], Equal);
        assert_s_cmp!(&[0, 1], &[0, 2], Less);
    }

    #[test]
    #[cfg(feature = "rust_1_46")]
    fn str_cmp_test() {
        use core::cmp::{
            Ord,
            Ordering::{Equal, Greater, Less},
        };

        macro_rules! assert_s_cmp {
            ($left:expr, $right:expr, $expected:expr) => {
                assert_eq!(str_cmp($left, $right), $expected, "A");
                assert_eq!($left.cmp($right), $expected, "B");

                assert_eq!(str_cmp($left, $left), Equal);
                assert_eq!(str_cmp($right, $right), Equal);

                assert_eq!(str_cmp($right, $left), $expected.reverse(), "cmp");
                assert_eq!($right.cmp($left), $expected.reverse(), "cmp");
            };
        }

        assert_s_cmp!("0", "", Greater);
        assert_s_cmp!("0", "1", Less);
        assert_s_cmp!("0", "01", Less);
        assert_s_cmp!("1", "01", Greater);
        assert_s_cmp!("099999", "12", Less);
        assert_s_cmp!("111111", "12", Less);
        assert_s_cmp!("120", "12", Greater);
        assert_s_cmp!("199999", "12", Greater);
        assert_s_cmp!("299999", "12", Greater);
        assert_s_cmp!("01", "02", Less);
    }
}
