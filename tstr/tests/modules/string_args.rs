use tstr::*;

// Use this to generate the tuple for a particular string:
/*
fn foo(){
    let string = r"\u{2D}\u{61}A\u{3C}>?{\u{7D}¢¤§©ߨࡕৰ\u{9F0}ⓩ蓭𐂶𣏦";

    print!("(");
    for chunk in string.as_bytes().chunks(8) {
        print!("(");
        for b in chunk {
            print!("__0x{:02X}, ", b);
        }
        print!("), ");
    }
    print!(")");
    println!();
}
*/

// Testing empty strings
test_case!("", (), "");

// Testing single char strings
test_case!("b", __<'b'>, "b");

// Testing two char strings
test_case!("ab", __<'a','b'>, "ab");

#[cfg(not(feature = "str_generics"))]
#[allow(dead_code)]
type AllCharLengths = (
    __<'+', '-', 'a', 'A', '<', '>', '?', '{'>,
    __<'}', '¢', '¤', '§', '©', 'ߨ', 'ࡕ', 'ৰ'>,
    __<'ৰ', 'ⓩ', '蓭', '𐂶', '𣏦'>,
    (),
    (),
    (),
    (),
    (),
);

// Using characters of all utf8 lengths, and no escapes.
str_test_case!("+-aA<>?{}¢¤§©ߨࡕৰৰⓩ蓭𐂶𣏦", AllCharLengths);

str_test_case!(r"+-aA<>?{}¢¤§©ߨࡕৰৰⓩ蓭𐂶𣏦", AllCharLengths);

str_test_case!(r#"+-aA<>?{}¢¤§©ߨࡕৰৰⓩ蓭𐂶𣏦"#, AllCharLengths);

str_test_case!(r##"+-aA<>?{}¢¤§©ߨࡕৰৰⓩ蓭𐂶𣏦"##, AllCharLengths);

// Using unicode escapes
str_test_case!(
    "+\u{2D}\u{61}A\u{3C}\u{3E}?\u{7B}\u{7D}¢\u{A4}\u{A7}©\u{7E8}\u{855}ৰ\u{9F0}\u{24E9}蓭\u{100B6}\u{233E6}",
    AllCharLengths
);

////////////////////////////////////////////////////////////////////////////////////

#[cfg(not(feature = "str_generics"))]
#[allow(dead_code)]
type IntermittentUnicodeEscapes = (
    __<'-', 'a', 'A', '<', '>', '?', '{', '}'>,
    __<'¢', '¤', '§', '©', 'ߨ', 'ࡕ', 'ৰ', 'ৰ'>,
    __<'ⓩ', '蓭', '𐂶', '𣏦'>,
    (),
    (),
    (),
    (),
    (),
);

#[cfg(not(feature = "str_generics"))]
#[allow(dead_code)]
type IntermittentUnicodeEscapesRaw = (
    __<'\\', 'u', '{', '2', 'D', '}', '\\', 'u'>,
    __<'{', '6', '1', '}', 'A', '\\', 'u', '{'>,
    __<'3', 'C', '}', '>', '?', '{', '\\', 'u'>,
    __<'{', '7', 'D', '}', '¢', '¤', '§', '©'>,
    __<'ߨ', 'ࡕ', 'ৰ', '\\', 'u', '{', '9', 'F'>,
    __<'0', '}', 'ⓩ', '蓭', '𐂶', '𣏦'>,
    (),
    (),
);

// Using characters of all utf8 lengths, and with some intermittent escapes.
str_test_case!(
    "\u{2D}\u{61}A\u{3C}>?{\u{7D}¢¤§©ߨࡕৰ\u{9F0}ⓩ蓭𐂶𣏦",
    IntermittentUnicodeEscapes
);

str_test_case!(
    r"\u{2D}\u{61}A\u{3C}>?{\u{7D}¢¤§©ߨࡕৰ\u{9F0}ⓩ蓭𐂶𣏦",
    IntermittentUnicodeEscapesRaw
);

str_test_case!(
    r#"\u{2D}\u{61}A\u{3C}>?{\u{7D}¢¤§©ߨࡕৰ\u{9F0}ⓩ蓭𐂶𣏦"#,
    IntermittentUnicodeEscapesRaw
);

str_test_case!(
    r##"\u{2D}\u{61}A\u{3C}>?{\u{7D}¢¤§©ߨࡕৰ\u{9F0}ⓩ蓭𐂶𣏦"##,
    IntermittentUnicodeEscapesRaw,
);

////////////////////////////////////////////////////////////////////////////////////

#[cfg(not(feature = "str_generics"))]
#[allow(dead_code)]
type AsciiEscapes = (
    __<'A', '\u{0}', 'B', ' ', 'C', '1', 'D', 'B'>,
    __<'E', 'S', 'F', 'z', 'G', '\u{7f}', 'H'>,
    (),
    (),
    (),
    (),
    (),
    (),
);

#[cfg(not(feature = "str_generics"))]
#[allow(dead_code)]
type AsciiEscapesRaw = (
    __<'A', '\\', 'x', '0', '0', 'B', '\\', 'x'>,
    __<'2', '0', 'C', '\\', 'x', '3', '1', 'D'>,
    __<'\\', 'x', '4', '2', 'E', '\\', 'x', '5'>,
    __<'3', 'F', '\\', 'x', '7', 'a', 'G', '\\'>,
    __<'x', '7', 'F', 'H'>,
    (),
    (),
    (),
);

// Testing the ascii escapes
str_test_case! {"A\x00B\x20C\x31D\x42E\x53F\x7aG\x7FH", AsciiEscapes}

str_test_case! {r"A\x00B\x20C\x31D\x42E\x53F\x7aG\x7FH", AsciiEscapesRaw}
str_test_case! {r#"A\x00B\x20C\x31D\x42E\x53F\x7aG\x7FH"#, AsciiEscapesRaw}
str_test_case! {r##"A\x00B\x20C\x31D\x42E\x53F\x7aG\x7FH"##, AsciiEscapesRaw}

////////////////////////////////////////////////////////////////////////////////////

#[cfg(not(feature = "str_generics"))]
#[allow(dead_code)]
type SingleCharEscapes = (
    __<'A', '\n', 'B', '\r', 'C', '\t', 'D', '\\'>,
    __<'E', '\u{0}', 'F', 'F', '\'', 'G', 'G', '\"'>,
    __<'H', 'H', '\n'>,
    (),
    (),
    (),
    (),
    (),
);

#[cfg(not(feature = "str_generics"))]
#[allow(dead_code)]
type SingleCharEscapesRaw = (
    __<'A', '\\', 'n', 'B', '\\', 'r', 'C', '\\'>,
    __<'t', 'D', '\\', '\\', 'E', '\\', '0', 'F'>,
    __<'F', '\\', '\'', 'G', 'G', '\\', '\"', 'H'>,
    __<'H', '\n'>,
    (),
    (),
    (),
    (),
);

// Testing single char escapes
str_test_case! {
    "A\nB\rC\tD\\E\0FF\'GG\"HH
",
    SingleCharEscapes,
}

str_test_case! {
    r#"A\nB\rC\tD\\E\0FF\'GG\"HH
"#,
    SingleCharEscapesRaw,
}

str_test_case! {
    r##"A\nB\rC\tD\\E\0FF\'GG\"HH
"##,
    SingleCharEscapesRaw,
}

////////////////////////////////////////////////////////////////////////////////////

#[cfg(not(feature = "str_generics"))]
#[allow(dead_code)]
type BackSlashNewline = __<'f', 'o', 'o', 'b', 'a', 'r'>;

#[cfg(not(feature = "str_generics"))]
#[allow(dead_code)]
type BackSlashNewlineRaw = (
    __<'f', 'o', 'o', '\\', '\n', ' ', ' ', ' '>,
    __<' ', 'b', 'a', 'r'>,
    (),
    (),
    (),
    (),
    (),
    (),
);

str_test_case! {
    "foo\
    bar",
    BackSlashNewline,
}

str_test_case! {
    r"foo\
    bar",
    BackSlashNewlineRaw,
}

////////////////////////////////////////////////////////////////////////////////////

#[cfg(not(feature = "str_generics"))]
#[allow(dead_code)]
type Quotes = __<'"', '"'>;

#[cfg(not(feature = "str_generics"))]
#[allow(dead_code)]
type QuoteHash = __<'"', '#'>;

str_test_case! {r#""""#, Quotes}
str_test_case! {r##""""##   , Quotes}

str_test_case! {r##""#"##, QuoteHash}
str_test_case! {r###""#"###, QuoteHash}

// Just making sure that this module is compiled.
#[test]
fn testing_string_args() {}
