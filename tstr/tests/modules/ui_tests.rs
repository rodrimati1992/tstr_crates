// use these commands for ui tests:
//
// clear;clear; env TRYBUILD=overwrite cargo test --no-default-features \
// --features="__ui_tests const_panic" \
// && clear;clear; env TRYBUILD=overwrite cargo test --no-default-features \
// --features="__ui_tests const_panic use_syn" \
// && env TRYBUILD=overwrite cargo +nightly test --no-default-features \
// --features="__ui_tests const_panic nightly_str_generics"
//
// (runs with and without "nightly_str_generics" feature)

#[cfg(feature = "__ui_tests")]
#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    for dir in [
        "assert_ui_tests",
        "strlike_ui_tests",
        "ts_macros_ui_tests",
        "tstr_methods_ui_tests",
    ] {
        #[cfg(not(feature = "str_generics"))]
        t.compile_fail(format!("tests/modules/{}/*-char-err.rs", dir));

        // these don't showcase proc macro errors or errors that show the TStr type
        #[cfg(not(feature = "str_generics"))]
        t.compile_fail(format!("tests/modules/{}/*--err.rs", dir));

        #[cfg(feature = "str_generics")]
        t.compile_fail(format!("tests/modules/{}/*-str-err.rs", dir));

        #[cfg(not(feature = "use_syn"))]
        t.compile_fail(format!("tests/modules/{}/*-unsyn-err.rs", dir));

        #[cfg(feature = "use_syn")]
        t.compile_fail(format!("tests/modules/{}/*-syn-err.rs", dir));

        t.pass(format!("tests/modules/{}/*fine.rs", dir));
    }
}
