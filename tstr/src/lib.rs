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
//!
//!     takes_person(&OtherPerson::new("Bob", "Marley"));
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
//! use other_person::OtherPerson;
//! mod other_person {
//!     use std::ops::Index;
//!
//!     use tstr::TS;
//!     
//!     pub struct OtherPerson {
//!         name: &'static str,
//!         surname: &'static str,
//!     }
//!     
//!     impl OtherPerson {
//!         pub fn new(name: &'static str, surname: &'static str) -> Self {
//!             Self{name, surname}
//!         }
//!     }
//!     
//!     impl Index<TS!(name)> for OtherPerson {
//!         type Output = str;
//!         
//!         fn index(&self, _: TS!(name)) -> &str {
//!             self.name
//!         }
//!     }
//!    
//!     impl Index<TS!(surname)> for OtherPerson {
//!         type Output = str;
//!         
//!         fn index(&self, _: TS!(surname)) -> &str {
//!             self.surname
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

// #[cfg(feature = "for_examples")]
// #[cfg_attr(feature = "docsrs", doc(cfg(feature = "for_examples")))]
// mod for_examples;
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
