mod modules {
    #[macro_use]
    mod utils;

    mod alias_and_tuples;

    mod concat_args;

    mod long_strings;

    mod string_args;

    #[cfg(feature = "cmp_traits")]
    mod string_cmp;

    mod other_args;

    mod to_uint;
}
