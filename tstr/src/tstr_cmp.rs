use crate::TStr;

#[cfg(not(feature = "const_generics"))]
mod impl_no_const_generics;

#[cfg_attr(feature = "docsrs", doc(cfg(feature = "cmp_traits")))]
pub trait TStrEq<Rhs> {
    const EQ: bool;
    const NE: bool = !Self::EQ;
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
