mod modules {
    #[macro_use]
    mod utils;

    mod alias_and_tuples;

    mod concat_args;

    #[cfg(feature = "const_panic")]
    mod const_panic_tests;

    mod long_strings;

    mod string_args;

    mod string_cmp;

    mod strlike_tests;

    mod other_args;

    mod trait_fn_equiv_tests;

    mod tstr_methods_tests;
}
