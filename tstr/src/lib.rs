//! This crate provides an encoding of type-level strings as types.
//!
//! # Example
//!
//! ```
//! use std::ops::Index;
//!
//! use tstr::{TS, ts};
//!
//! fn main(){
//!     takes_person(&Person::new("Bob".into(), "Marley".into()));
//! }
//!
//! fn takes_person<P>(
//!     pers: &P
//! ) where
//!     P: Index<TS!(name), Output = str> + Index<TS!(surname), Output = str>
//! {
//!     assert_eq!(&pers[ts!(name)], "Bob");
//!     assert_eq!(&pers[ts!(surname)], "Marley");
//! }
//!
//!
//! use person::Person;
//! mod person {
//!     use std::ops::Index;
//!
//!     use tstr::TS;
//!     
//!     pub struct Person {
//!         name: String,
//!         surname: String,
//!     }
//!     
//!     impl Person {
//!         pub fn new(name: String, surname: String) -> Self {
//!             Self{name, surname}
//!         }
//!     }
//!     
//!     impl Index<TS!(name)> for Person {
//!         type Output = str;
//!         
//!         fn index(&self, _: TS!(name)) -> &str {
//!             &self.name
//!         }
//!     }
//!    
//!     impl Index<TS!(surname)> for Person {
//!         type Output = str;
//!         
//!         fn index(&self, _: TS!(surname)) -> &str {
//!             &self.surname
//!         }
//!     }
//! }
//!
//! ```
//!
#![no_std]
#![cfg_attr(feature = "nightly_const_generics", feature(const_generics))]
#![cfg_attr(feature = "docsrs", feature(doc_cfg))]
#![allow(non_camel_case_types)]
#![cfg_attr(feature = "nightly_const_generics", allow(incomplete_features))]

mod macros;
mod to_uint;
mod tstr;

#[doc(hidden)]
pub use tstr_proc_macros::__ts_impl;

pub use crate::{to_uint::ToUint, tstr::TStr};

#[cfg_attr(feature = "docsrs", doc(cfg(feature = "const_generics")))]
#[cfg(feature = "const_generics")]
pub use crate::tstr::StrValue;

include! {"./p.rs"}
