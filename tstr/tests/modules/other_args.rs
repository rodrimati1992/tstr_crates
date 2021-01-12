use tstr::*;

test_case! {0b0, (__0x30, ), "0"}
test_case! {0b1, (__0x31, ), "1"}
test_case! {0b10, (__0x32, ), "2"}
test_case! {0b100, (__0x34, ), "4"}
test_case! {0b1000, (__0x38, ), "8"}
test_case! {0b1100, (__0x31, __0x32), "12"}
test_case! {0b1_100, (__0x31, __0x32), "12"}
test_case! {0b10_00__00_01, (__0x31, __0x32, __0x39), "129"}

test_case! {0o0, (__0x30, ), "0"}
test_case! {0o1, (__0x31, ), "1"}
test_case! {0o2, (__0x32, ), "2"}
test_case! {0o4, (__0x34, ), "4"}
test_case! {0o10, (__0x38, ), "8"}
test_case! {0o14, (__0x31, __0x32), "12"}
test_case! {0o1_4, (__0x31, __0x32), "12"}
test_case! {0o201, (__0x31, __0x32, __0x39), "129"}
test_case! {0o2__0_1, (__0x31, __0x32, __0x39), "129"}

test_case! {0x0, (__0x30, ), "0"}
test_case! {0x1, (__0x31, ), "1"}
test_case! {0x2, (__0x32, ), "2"}
test_case! {0x4, (__0x34, ), "4"}
test_case! {0x8, (__0x38, ), "8"}
test_case! {0xC, (__0x31, __0x32), "12"}
test_case! {0x12, (__0x31, __0x38), "18"}
test_case! {0x1_2, (__0x31, __0x38), "18"}
test_case! {0x41, (__0x36, __0x35), "65"}
test_case! {0x4_1, (__0x36, __0x35), "65"}
test_case! {0x100, (__0x32, __0x35, __0x36), "256"}
test_case! {0x103, (__0x32, __0x35, __0x39), "259"}
test_case! {0x1__0_3, (__0x32, __0x35, __0x39), "259"}

test_case! {0, (__0x30, ), "0"}
test_case! {3, (__0x33, ), "3"}
test_case! {10, (__0x31, __0x30), "10"}
test_case! {1_0, (__0x31, __0x30), "10"}
test_case! {16, (__0x31, __0x36), "16"}
test_case! {1_6, (__0x31, __0x36), "16"}
test_case! {128, (__0x31, __0x32, __0x38), "128"}
test_case! {1__2_8, (__0x31, __0x32, __0x38), "128"}

test_case! {
    foo_bar_baz,
    ((__0x66, __0x6F, __0x6F, __0x5F, __0x62, __0x61, __0x72, __0x5F, ), (__0x62, __0x61, __0x7A, ), ),
    "foo_bar_baz",
}

test_case! {
    r#foo_bar_baz,
    ((__0x66, __0x6F, __0x6F, __0x5F, __0x62, __0x61, __0x72, __0x5F, ), (__0x62, __0x61, __0x7A, ), ),
    "foo_bar_baz",
}

test_case! {_0, (__0x5F, __0x30, ), "_0"}

test_case! {r#_0, (__0x5F, __0x30, ), "_0"}

test_case! {r#async, (__0x61, __0x73, __0x79, __0x6E, __0x63, ), "async"}
