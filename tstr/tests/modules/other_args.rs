use tstr::*;

test_case! {0b0, __<'0'>, "0"}
test_case! {0b1, __<'1'>, "1"}
test_case! {0b10, __<'2'>, "2"}
test_case! {0b100, __<'4'>, "4"}
test_case! {0b1000, __<'8'>, "8"}
test_case! {0b1100, __<'1', '2'>, "12"}
test_case! {0b1_100, __<'1', '2'>, "12"}
test_case! {0b10_00__00_01, __<'1', '2', '9'>, "129"}

test_case! {0o0, __<'0'>, "0"}
test_case! {0o1, __<'1'>, "1"}
test_case! {0o2, __<'2'>, "2"}
test_case! {0o4, __<'4'>, "4"}
test_case! {0o10, __<'8'>, "8"}
test_case! {0o14, __<'1', '2'>, "12"}
test_case! {0o1_4, __<'1', '2'>, "12"}
test_case! {0o201, __<'1', '2', '9'>, "129"}
test_case! {0o2__0_1, __<'1', '2', '9'>, "129"}

test_case! {0x0, __<'0'>, "0"}
test_case! {0x1, __<'1'>, "1"}
test_case! {0x2, __<'2'>, "2"}
test_case! {0x4, __<'4'>, "4"}
test_case! {0x8, __<'8'>, "8"}
test_case! {0xC, __<'1', '2'>, "12"}
test_case! {0x12, __<'1', '8'>, "18"}
test_case! {0x1_2, __<'1', '8'>, "18"}
test_case! {0x41, __<'6', '5'>, "65"}
test_case! {0x4_1, __<'6', '5'>, "65"}
test_case! {0x100, __<'2', '5', '6'>, "256"}
test_case! {0x103, __<'2', '5', '9'>, "259"}
test_case! {0x1__0_3, __<'2', '5', '9'>, "259"}

test_case! {0, __<'0'>, "0"}
test_case! {3, __<'3'>, "3"}
test_case! {10, __<'1', '0'>, "10"}
test_case! {1_0, __<'1', '0'>, "10"}
test_case! {16, __<'1', '6'>, "16"}
test_case! {1_6, __<'1', '6'>, "16"}
test_case! {128, __<'1', '2', '8'>, "128"}
test_case! {1__2_8, __<'1', '2', '8'>, "128"}

test_case! {
    foo_bar_baz,
    (__<'f', 'o', 'o', '_', 'b', 'a', 'r', '_'>, __<'b', 'a', 'z'>, (), (), (), (), (), ()),
    "foo_bar_baz",
}

test_case! {
    r#foo_bar_baz,
    (__<'f', 'o', 'o', '_', 'b', 'a', 'r', '_'>, __<'b', 'a', 'z'>, (), (), (), (), (), ()),
    "foo_bar_baz",
}

test_case! {_0, __<'_', '0'>, "_0"}

test_case! {r#_0, __<'_', '0'>, "_0"}

test_case! {
    r#async,
    __<'a', 's', 'y', 'n', 'c'>,
    "async",
}

// Just making sure that this module is compiled.
#[test]
fn testing_other_args() {}
