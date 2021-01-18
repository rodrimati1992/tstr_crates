use crate::TStr;

#[cfg(not(feature = "const_generics"))]
mod impl_no_const_generics;

/// For equality comparison between type-level strings.
///
/// # Example
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
pub trait TStrEq<Rhs> {
    const EQ: bool;

    const NE: bool = !Self::EQ;

    /// Returns whether `self` is equal to `other`.
    fn tstr_eq(&self, _other: &Rhs) -> bool {
        Self::EQ
    }

    /// Returns whether `self` is not equal to `other`.
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

#[cfg(feature = "const_generics")]
macro_rules! impl_const_generics {
    () => {
        impl<const S: &'static str, const Z: &'static str> TStrEq<crate::___<Z>> for crate::___<S> {
            const EQ: bool = crate::utils::str_eq(S, Z);
        }
    };
}

#[cfg(feature = "const_generics")]
impl_const_generics! {}
