// This file defines many types that are `#[doc(hidden)] pub`
// and required not to be used by users.

#[cfg(not(feature = "str_generics"))]
macro_rules! declare_min_const {
    (($($chars:ident)*))=>{
        #[doc(hidden)]
        /// implementation detail of tstr
        pub struct __<$(const $chars: char = '\0',)*>;
        
        #[doc(hidden)]
        /// implementation detail of tstr
        pub struct ___<S, const LEN: usize>(core::marker::PhantomData<fn() -> S>);
    }
}

#[cfg(not(feature = "str_generics"))]
crate::private_macros::with_elem_count_idents!{ declare_min_const!{} }


#[cfg(feature = "str_generics")]
macro_rules! declare_const_items {
    () => {
        #[doc(hidden)]
        /// implementation detail of tstr
        pub struct ___<const S: &'static str>;
    };
}

#[cfg(feature = "str_generics")]
declare_const_items! {}

