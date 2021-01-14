use crate::TStr;

/// A type that is trivially constructible
pub trait MakeTStr {
    const MAKE: Self;
}

impl<T> MakeTStr for TStr<T> {
    const MAKE: Self = TStr::NEW;
}

macro_rules! tuple_impl {
    ($($ty:ident)*) => (
        impl<$($ty),*> MakeTStr for ($($ty,)*)
        where
            $($ty: MakeTStr,)*
        {
            const MAKE: Self = (
                $($ty::MAKE,)*
            );
        }
    )
}
tuple_impl! {}
tuple_impl! {A }
tuple_impl! {A B}
tuple_impl! {A B C}
tuple_impl! {A B C D}
tuple_impl! {A B C D E}
tuple_impl! {A B C D E F}
tuple_impl! {A B C D E F G}
tuple_impl! {A B C D E F G H}
