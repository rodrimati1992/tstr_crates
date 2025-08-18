use crate::TStr;

/// For constructing [`TStr`]s or collections of them.
///
/// [`TStr`]: crate::TStr
pub trait Make: Copy {
    /// Gets a value of this type
    const MAKE: Self;
}

impl<T> Make for TStr<T> {
    const MAKE: Self = TStr::new();
}

macro_rules! impl_make {
    ($($ty:ident)*) => (
        impl<$($ty),*> Make for ($($ty,)*)
        where
            $($ty: Make,)*
        {
            const MAKE: Self = (
                $($ty::MAKE,)*
            );
        }
    )
}

macro_rules! with_idents {
    ( ( $($first:ident $($ty:ident)*)? ) ) => (
        impl_make!{$($first $($ty)*)?}

        $( with_idents!{ ($($ty)*) } )?
    )
}

crate::private_macros::with_elem_count_idents! { with_idents!{} }
