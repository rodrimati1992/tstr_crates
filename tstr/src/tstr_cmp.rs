use crate::TStr;

#[cfg(not(feature = "const_generics"))]
mod impl_no_const_generics;

/// For equality comparison between type-level strings.
///
/// # Examples
///
/// # Basic
///
/// ```rust
/// use tstr::{TS, TStrEq, ts};
///
/// use std::cmp::Ordering;
///
/// assert!(<TS!("foo") as TStrEq<TS!("foo")>>::EQ); // EQ for equal
///
/// assert!(<TS!("foo") as TStrEq<TS!("bar")>>::NE); // NE for not equal
///
/// // You can also compare TStrs using the `tstr_eq` and `tstr_ne` methods.
/// assert!(ts!("foo").tstr_eq(&ts!("foo")));
///
/// assert!(ts!("foo").tstr_ne(&ts!("bar")));
///
/// ```
///
///
/// # Advanced
///
/// This uses types from the `for_examples` module,
/// which can be seen in the docs with the "for_examples" feature.
///
/// ```rust
/// use tstr::for_examples::Foo;
/// use tstr::{TStr, TStrEq, TS, ts};
///
/// use std::ops::Index;
///
/// let this = Foo::new(3, 5, "8");
/// assert_eq!(get_two(&this, ts!(bar), ts!(qux)), (&3, &"8"));
///
/// // Doesn't compile
/// // assert_eq!(get_two(&this, ts!(bar), ts!(bar)), (&3, &3));
///
/// fn get_two<T, A, B>(
///     this: &T,
///     field_a: TStr<A>,
///     field_b: TStr<B>,
/// ) -> (&IndexOut<T, TStr<A>>, &IndexOut<T, TStr<B>>)
/// where
///     T: Index<TStr<A>> + Index<TStr<B>>,
///     A: TStrEq<B>,
/// {
///     tstr::Assert::<A, B>::NOT_EQUAL;
///     (&this[field_a], &this[field_b])
/// }
///
///
/// type IndexOut<T, I> = <T as Index<I>>::Output;
///
/// ```
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "cmp_traits")))]
pub trait TStrEq<Rhs>: Sized {
    /// Whether `Self` equals `Rhs`
    const EQ: bool;

    /// Whether `Self` is not equal to `Rhs`
    const NE: bool = !Self::EQ;

    /// Returns whether `self` is equal to `other`.
    #[inline(always)]
    fn tstr_eq(&self, _other: &Rhs) -> bool {
        Self::EQ
    }

    /// Returns whether `self` is not equal to `other`.
    #[inline(always)]
    fn tstr_ne(&self, _other: &Rhs) -> bool {
        Self::NE
    }
}

#[cfg(not(feature = "min_const_generics"))]
pub trait U8Repr {
    const REPR: u8;
}

impl<T, U> TStrEq<TStr<U>> for TStr<T>
where
    T: TStrEq<U>,
{
    const EQ: bool = T::EQ;
}

/// For comparison between two type-level strings,
/// getting the `Ordering` of `Self` relative to `Rhs`.
///
/// This is only available with the `"const_generics"` feature,
/// since it's only implemented for the `&'static str`-const-parameter-based representation.
///
/// # Example
///
/// ```rust
/// use tstr::{TS, TStrOrd, ts};
///
/// use std::cmp::Ordering;
///
/// const FOO_CMP_FOO: Ordering = <TS!("foo") as TStrOrd<TS!("foo")>>::CMP;
/// assert_eq!(FOO_CMP_FOO, Ordering::Equal);
///
/// const FOO_CMP_FOOOOO: Ordering = <TS!("foo") as TStrOrd<TS!("fooooo")>>::CMP;
/// assert_eq!(FOO_CMP_FOOOOO, Ordering::Less);
///
/// // You can also compare TStrs using the `tstr_cmp` method.
/// assert_eq!(ts!("foo").tstr_cmp(&ts!("bar")), Ordering::Greater);
///
/// // short strings can be greater than longer strings
/// assert_eq!(ts!("foo").tstr_cmp(&ts!("aaaaaa")), Ordering::Greater);
///
/// ```
///
#[cfg(feature = "const_generics")]
#[cfg_attr(
    feature = "docsrs",
    doc(cfg(all(feature = "const_generics", feature = "cmp_traits")))
)]
pub trait TStrOrd<Rhs>: Sized {
    /// The `Ordering` of `Self` relative to `Rhs`.
    const CMP: core::cmp::Ordering;

    /// Compares `self` and `other` for ordering.
    #[inline(always)]
    fn tstr_cmp(&self, _other: &Rhs) -> core::cmp::Ordering {
        Self::CMP
    }
}

#[cfg(feature = "const_generics")]
impl<T, U> TStrOrd<TStr<U>> for TStr<T>
where
    T: TStrOrd<U>,
{
    const CMP: core::cmp::Ordering = T::CMP;
}

#[cfg(feature = "const_generics")]
macro_rules! impl_const_generics {
    () => {
        impl<const S: &'static str, const Z: &'static str> TStrEq<crate::___<Z>> for crate::___<S> {
            const EQ: bool = crate::utils::str_eq(S, Z);
        }

        impl<const S: &'static str, const Z: &'static str> TStrOrd<crate::___<Z>>
            for crate::___<S>
        {
            const CMP: core::cmp::Ordering = crate::utils::str_cmp(S, Z);
        }
    };
}

#[cfg(feature = "const_generics")]
impl_const_generics! {}
