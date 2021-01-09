//! This crate provides the `TStr` type for emulating const parameters of `&'static str` type.
//!
//!
#![no_std]
#![cfg_attr(feature = "nightly_const_generics", feature(const_generics))]
#![allow(non_camel_case_types)]

mod macros;
mod tstr;

#[doc(hidden)]
pub use tstr_proc_macros::__ts_impl;

pub use crate::tstr::TStr;

include! {"./p.rs"}
