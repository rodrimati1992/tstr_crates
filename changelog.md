### 0.2.1

Added `"cmp_traits"` feature to enable TStr comparison traits.

Added `"rust_1_46"` feature to enable const functions to compare `&str` and `&[u8]`.

Added TStrEq trait for equality comparison between type-level strings, enabled by the `"cmp_traits"` feature.

Added a utils module, with `str_eq` and `u8_slice_eq` const functions, enabled bt the `"rust_1_46"` feature.

Added `Assert` type for type-level-string related compile-time assertions, inside `tstr::asserts` module.

Added `EqualityProof` and `InequalityProof` structs, inside `tstr::asserts` module,

Added `tstr_eq`, `tstr_ne`, and `tstr_cmp` macros for comparing type-level strings.

Added `get_two` method to types in `for_examples`, for examples that use `tstr::Assert`.

### 0.2.0

Added support for passing multiple literals/identifiers in macros, outputting a tuple of TStrs.

Added `"min_const_generics"` feature, which changes the representation of type-level strings to use many `char` const parameters.

Added `concat!(...)` and `stringify!(...)` syntaxes to the `TS`/`alias`/`ts` macros,
since those macros are not expanded before being passed to other macros.

Breaking change: removed the `Copy` supertrait of `ToUint`.

Changed `TStr<T>`s implementation of `ToUInt` to use the `T: ToUInt` bound, allowing easier use of `oUint` in generic functions that take a generic `TStr<T>` .

Added `MakeTStr` trait to construct `TStr`s and tuples of them.

Changed the internal representation of type-level strings from singly nested tuples to 
recursive tuples, this is only visible to users of the `tstr` crate in error messages.

### 0.1.1

Crated `tstr` crate, and `tstr_proc_macros` proc macro crates.

Added opt-in `syn` and `proc_macro2` dependencies to `tstr_proc_macros`

Added `tstr_proc_macros` dependency to `tstr`

Added these features to `tstr_proc_macro` crate:
    
    - `"const_generics"`: to use `&'static str` const parameters in the TStr type.
    
    - `"syn_"`: to parse literals with syn, instead of manually parsing them.

Added these features to tstr crate:
    
    - `"const_generics"`: to use `&'static str` const parameters in the TStr type.
    
    - `"nightly_const_generics"`: the same as `"const_generics"`, for use in Rust nightly.
    
    - `"use_syn"`: to parse literals with syn, instead of manually parsing them.
    
    - `"for_examples"`: to enable some types used in documentation examples.

Added TStr type, to represent type level strings

Added TS macro, to get the type of the TStr equivalent of a literal/identifier

Added ts macro, to get the value of the TStr equivalent of a literal/identifier

Added alias macro, to create const and type aliases of type-level strings

Added StrValue trait, to get the `&'static str` value of a TStr, only usable with the `"const_generics"`  feature enabled.

Added ToUint trait, to convert a TStr to an unsigned integer

Added for_examples module, with types used in documentation examples.