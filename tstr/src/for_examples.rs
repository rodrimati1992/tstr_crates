//! Types used by documentation examples.

use core::ops::{Index, IndexMut};

#[cfg(feature = "cmp_traits")]
use crate::{asserts::InequalityProof, TStr, TStrEq};

use crate::TS;

macro_rules! impl_index_indexmut {
    (
        impl $impl_params:tt $self:ty {
            $($fields:tt)*
        }
    ) => (
        $(
            impl_index_indexmut!{
                @rep
                impl $impl_params $self { $fields }
            }
        )*

        impl_index_indexmut!{
            @inner
            impl $impl_params $self
        }


    );
    (@inner impl[$($impl:tt)*] $self:ty ) => {
        #[cfg(feature = "cmp_traits")]
        #[cfg_attr(feature = "docsrs", doc(cfg(feature = "cmp_traits")))]
        impl<$($impl)*>  $self {
            pub fn get_two<A, B>(
                &self,
                field_a: TStr<A>,
                field_b: TStr<B>,
                _proof: InequalityProof<TStr<A>, TStr<B>>,
            ) -> (&<Self as Index<TStr<A>>>::Output, &<Self as Index<TStr<B>>>::Output)
            where
                Self: Index<TStr<A>> + Index<TStr<B>>,
                A: TStrEq<B>,
            {
                (&self[field_a], &self[field_b])
            }
        }
    };
    (@rep
        impl[$($impl:tt)*] $self:ty {
            ($field_name:ident : $type:ty)
        }
    ) => {
        const _: () = {
            type $field_name = TS!($field_name);

            impl<$($impl)*> Index<$field_name> for $self {
                type Output = $type;

                fn index(&self, _: $field_name) -> &$type {
                    &self.$field_name
                }
            }

            impl<$($impl)*> IndexMut<$field_name> for $self {
                fn index_mut(&mut self, _: $field_name) -> &mut $type {
                    &mut self.$field_name
                }
            }
        };
    };
}

/// For examples
#[derive(Debug)]
pub struct Foo {
    bar: u32,
    baz: u64,
    qux: &'static str,
}

impl Foo {
    /// A simple contructor
    pub fn new(bar: u32, baz: u64, qux: &'static str) -> Self {
        Self { bar, baz, qux }
    }
}

impl_index_indexmut! {
    impl[] Foo {
        (bar: u32)
        (baz: u64)
        (qux: &'static str)
    }
}

/// For examples
#[derive(Debug)]
pub struct Bar {
    bar: u32,
    baz: bool,
    boom: Option<char>,
}

impl Bar {
    /// A simple contructor
    pub fn new(bar: u32, baz: bool, boom: Option<char>) -> Self {
        Self { bar, baz, boom }
    }
}

impl_index_indexmut! {
    impl[] Bar {
        (bar: u32)
        (baz: bool)
        (boom: Option<char>)
    }
}
