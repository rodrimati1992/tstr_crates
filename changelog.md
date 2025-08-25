# 0.3

### 0.3.0

Removed these items:
- `asserts` module: replaced by the `assert_str_*` macros and `tstr::type_eq` function.
- `for_examples` module
- `StrValue` trait: replaced by the `IsTStr` trait
- `ToUint` trait: integers have const fns for parsing from a `&str`. The `TStr` docs show how this can be used to parse an integer.
- `tstr_cmp` macro: replaced by the `tstr::cmp` function
- `tstr_eq` macro: replaced by the `tstr::eq` function
- `tstr_ne` macro: replaced by the `tstr::ne` function
- `TStrEq` trait: replaced by the `tstr::eq` function
- `TStrOrd` trait: replaced by the `tstr::cmp` function
- `MakeTStr` trait: replaced by the `IsTStr` trait

Removed these features:
- `"rust_1_46"`: because the Minimum Supported Rust Version is now higher than 1.46
- `"cmp_traits"`: because comparison functions are enabled by default
- `"for_examples"`: because there's no `for_examples` module anymore
- `"min_const_generics"`: because the const-`char`-parameter-based representation is now the default.

Renamed these features:
- `"const_generics"` to `"str_generics"`
- `"nightly_const_generics"` to `"nightly_str_generics"`

Changed Minimum Supported Rust Version to 1.88.0

Fixed `tstr::utils::u8_slice_cmp`: it used to consider all shorter slices less than all longer slices. Now it behaves the same as in std, where `short_slice.cmp(&long_slice)` can also return `Greater` depending on their elements.

Fixed support for Rust keywords in `alias`/`ts`/`TS` macros when `"use_syn"` feature is enabled

Fixed `&'static str`-repr support (needed fixing because nightly changed how `&'static str` const parameters are enabled)

Removed the units-struct-based representation for strings, making the `char` const-parameter-based representation the default.

Added `const_panic` feature and dependency (enabled by the feature, the feature is enabled by default)

Added `typewit` 1.13 dependency, with only the `"rust_1_61"` feature enabled by default.

Added re-exports of `typewit` crate at `tstr::typewit`.

Changed `"str_generics"` feature enable `"typewit/adt_const_marker"` feature, to use its `Str` const marker.

Added these `const fn`s at the root module (they all take `IsTStr` generically, and have an equivalent  `IsTStr` method):
- `tstr::cmp`: compares two `IsTStr`s for ordering
- `tstr::eq`: compares two `IsTStr`s for equality
- `tstr::len`: gets the utf-8 length of an `IsTStr`
- `tstr::ne`: compares two `IsTStr`s for inequality
- `tstr::to_bytes`: converts an `IsTStr` to a `&'static str` 
- `tstr::to_str`: converts an `IsTStr` to a `&'static str` 
- `tstr::type_eq`: compares two `IsTStr`s and returns a proof of their equality/inequality
(these functions are always enabled and work on stable)

Added `IsTStr` trait, implemented for `TStr` with:
- methods equivalents of the root module functions
- methods equivalents of `TStr` associated functions for converting to/from an `IsTStr` type parameter
- supertraits of all the traits that `TStr` impls (those that can be expressed, some are too generic t o write as a supertrait)
- `VAL` constant for constructing an `impl IsTStr` value
- constants for length, the `&[u8]` and `&str` values of the `TStr`

Added `TStrArg` trait, for bounding `TStr`'s type parameter.

Added `TStr::{from_gen, to_gen}` associated functions

Changed `TStr` impls, now it bounds its type parameter with `TStrArg` for every impl except for `Copy`, `Clone`, and `Default`.

Added these impls for `TStr`:
- `Display`: display formats the string returned by `to_str`
- `Hash`

Changed `PartialEq` and `PartialOrd` impls for `TStr` to take any `IsTStr` as the rhs argument.

Changed `Debug` impl for `TStr` to debug format the string returned by `to_str`.

Added these conditional on the `"const_panic"` feature:
- `const_panic::PanicFmt` impl for `TStr`
- reexport of `const_panic` itself: at `tstr::const_panic`
- reexport of `const_panic::unwrap_ok`: at `tstr::unwrap`

Added these macros conditional on `"const_panic"` feature:
- `tstr::assert_str_eq`: asserts that two `&str`/`impl ÌsTStr`s are equal, with formatted assertion error message.
- `tstr::assert_str_ne`: asserts that two `&str`/`impl ÌsTStr`s are unequal, with formatted assertion error message.

Changed `tstr::{TS, ts}` macros to accept additional parentheses around their arguments,

Changed `tstr::{alias, TS, ts}` macros to not produce a tuple of `TStr`s when multiple strings are passed in, those arguments are simply disallowed.

Improved error reporting for `tstr::{alias, TS, ts}` macros.

Added `tstr::strlike` module with these items:
- `StrLike` trait: for coercing `&str`/`impl ÌsTStr` to a `&str`
- `as_str` macro: coerces its `StrLike` argument to a `&str`

# 0.2

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