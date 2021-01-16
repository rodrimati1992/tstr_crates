use tstr::*;

macro_rules! str_test_case {
    ($string:tt, $tuple:ty $(,)*) => {
        test_case!($string, $tuple, $tuple, $string);
    };
}

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
test_case!("", (), (), "");

// Testing single char strings
test_case!("b", (__b,), __a<'b'>, "b");

// Testing two char strings
test_case!("ab", (__a, __b), __b<'a','b'>, "ab");

#[cfg(not(feature = "min_const_generics"))]
#[allow(dead_code)]
type AllCharLengths = (
    (
        __0x2B,
        __0x2D,
        __0x61,
        __0x41,
        __0x3C,
        __0x3E,
        __0x3F,
        __0x7B,
    ),
    (
        __0x7D,
        __0xC2,
        __0xA2,
        __0xC2,
        __0xA4,
        __0xC2,
        __0xA7,
        __0xC2,
    ),
    (
        __0xA9,
        __0xDF,
        __0xA8,
        __0xE0,
        __0xA1,
        __0x95,
        __0xE0,
        __0xA7,
    ),
    (
        __0xB0,
        __0xE0,
        __0xA7,
        __0xB0,
        __0xE2,
        __0x93,
        __0xA9,
        __0xE8,
    ),
    (
        __0x93,
        __0xAD,
        __0xF0,
        __0x90,
        __0x82,
        __0xB6,
        __0xF0,
        __0xA3,
    ),
    (__0x8F, __0xA6),
);

#[cfg(all(feature = "min_const_generics", not(feature = "const_generics")))]
#[allow(dead_code)]
type AllCharLengths = (
    __<'+', '-', 'a', 'A', '<', '>', '?', '{'>,
    __<'}', '¢', '¤', '§', '©', 'ߨ', 'ࡕ', 'ৰ'>,
    __e<'ৰ', 'ⓩ', '蓭', '𐂶', '𣏦'>,
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

#[cfg(not(feature = "min_const_generics"))]
#[allow(dead_code)]
type IntermittentUnicodeEscapes = (
    (
        __0x2D,
        __0x61,
        __0x41,
        __0x3C,
        __0x3E,
        __0x3F,
        __0x7B,
        __0x7D,
    ),
    (
        __0xC2,
        __0xA2,
        __0xC2,
        __0xA4,
        __0xC2,
        __0xA7,
        __0xC2,
        __0xA9,
    ),
    (
        __0xDF,
        __0xA8,
        __0xE0,
        __0xA1,
        __0x95,
        __0xE0,
        __0xA7,
        __0xB0,
    ),
    (
        __0xE0,
        __0xA7,
        __0xB0,
        __0xE2,
        __0x93,
        __0xA9,
        __0xE8,
        __0x93,
    ),
    (
        __0xAD,
        __0xF0,
        __0x90,
        __0x82,
        __0xB6,
        __0xF0,
        __0xA3,
        __0x8F,
    ),
    (__0xA6,),
);

#[cfg(all(feature = "min_const_generics", not(feature = "const_generics")))]
#[allow(dead_code)]
type IntermittentUnicodeEscapes = (
    __<'-', 'a', 'A', '<', '>', '?', '{', '}'>,
    __<'¢', '¤', '§', '©', 'ߨ', 'ࡕ', 'ৰ', 'ৰ'>,
    __d<'ⓩ', '蓭', '𐂶', '𣏦'>,
);

#[cfg(not(feature = "min_const_generics"))]
#[allow(dead_code)]
type IntermittentUnicodeEscapesRaw = (
    (
        __0x5C,
        __0x75,
        __0x7B,
        __0x32,
        __0x44,
        __0x7D,
        __0x5C,
        __0x75,
    ),
    (
        __0x7B,
        __0x36,
        __0x31,
        __0x7D,
        __0x41,
        __0x5C,
        __0x75,
        __0x7B,
    ),
    (
        __0x33,
        __0x43,
        __0x7D,
        __0x3E,
        __0x3F,
        __0x7B,
        __0x5C,
        __0x75,
    ),
    (
        __0x7B,
        __0x37,
        __0x44,
        __0x7D,
        __0xC2,
        __0xA2,
        __0xC2,
        __0xA4,
    ),
    (
        __0xC2,
        __0xA7,
        __0xC2,
        __0xA9,
        __0xDF,
        __0xA8,
        __0xE0,
        __0xA1,
    ),
    (
        __0x95,
        __0xE0,
        __0xA7,
        __0xB0,
        __0x5C,
        __0x75,
        __0x7B,
        __0x39,
    ),
    (
        __0x46,
        __0x30,
        __0x7D,
        __0xE2,
        __0x93,
        __0xA9,
        __0xE8,
        __0x93,
    ),
    (
        __0xAD,
        __0xF0,
        __0x90,
        __0x82,
        __0xB6,
        __0xF0,
        __0xA3,
        __0x8F,
    ),
    (__0xA6,),
);

#[cfg(all(feature = "min_const_generics", not(feature = "const_generics")))]
#[allow(dead_code)]
type IntermittentUnicodeEscapesRaw = (
    __<'\\', 'u', '{', '2', 'D', '}', '\\', 'u'>,
    __<'{', '6', '1', '}', 'A', '\\', 'u', '{'>,
    __<'3', 'C', '}', '>', '?', '{', '\\', 'u'>,
    __<'{', '7', 'D', '}', '¢', '¤', '§', '©'>,
    __<'ߨ', 'ࡕ', 'ৰ', '\\', 'u', '{', '9', 'F'>,
    __f<'0', '}', 'ⓩ', '蓭', '𐂶', '𣏦'>,
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
    IntermittentUnicodeEscapesRaw
);

////////////////////////////////////////////////////////////////////////////////////

#[cfg(not(feature = "min_const_generics"))]
#[allow(dead_code)]
type AsciiEscapes = (
    (
        __0x41,
        __0x00,
        __0x42,
        __0x20,
        __0x43,
        __0x31,
        __0x44,
        __0x42,
    ),
    (__0x45, __0x53, __0x46, __0x7A, __0x47, __0x7F, __0x48),
);

#[cfg(all(feature = "min_const_generics", not(feature = "const_generics")))]
#[allow(dead_code)]
type AsciiEscapes = (
    __<'A', '\u{0}', 'B', ' ', 'C', '1', 'D', 'B'>,
    __g<'E', 'S', 'F', 'z', 'G', '\u{7f}', 'H'>,
);

#[cfg(not(feature = "min_const_generics"))]
#[allow(dead_code)]
type AsciiEscapesRaw = (
    (
        __0x41,
        __0x5C,
        __0x78,
        __0x30,
        __0x30,
        __0x42,
        __0x5C,
        __0x78,
    ),
    (
        __0x32,
        __0x30,
        __0x43,
        __0x5C,
        __0x78,
        __0x33,
        __0x31,
        __0x44,
    ),
    (
        __0x5C,
        __0x78,
        __0x34,
        __0x32,
        __0x45,
        __0x5C,
        __0x78,
        __0x35,
    ),
    (
        __0x33,
        __0x46,
        __0x5C,
        __0x78,
        __0x37,
        __0x61,
        __0x47,
        __0x5C,
    ),
    (__0x78, __0x37, __0x46, __0x48),
);

#[cfg(all(feature = "min_const_generics", not(feature = "const_generics")))]
#[allow(dead_code)]
type AsciiEscapesRaw = (
    __<'A', '\\', 'x', '0', '0', 'B', '\\', 'x'>,
    __<'2', '0', 'C', '\\', 'x', '3', '1', 'D'>,
    __<'\\', 'x', '4', '2', 'E', '\\', 'x', '5'>,
    __<'3', 'F', '\\', 'x', '7', 'a', 'G', '\\'>,
    __d<'x', '7', 'F', 'H'>,
);

// Testing the ascii escapes
str_test_case! {"A\x00B\x20C\x31D\x42E\x53F\x7aG\x7FH", AsciiEscapes}

str_test_case! {r"A\x00B\x20C\x31D\x42E\x53F\x7aG\x7FH", AsciiEscapesRaw}
str_test_case! {r#"A\x00B\x20C\x31D\x42E\x53F\x7aG\x7FH"#, AsciiEscapesRaw}
str_test_case! {r##"A\x00B\x20C\x31D\x42E\x53F\x7aG\x7FH"##, AsciiEscapesRaw}

////////////////////////////////////////////////////////////////////////////////////

#[cfg(not(feature = "min_const_generics"))]
#[allow(dead_code)]
type SingleCharEscapes = (
    (
        __0x41,
        __0x0A,
        __0x42,
        __0x0D,
        __0x43,
        __0x09,
        __0x44,
        __0x5C,
    ),
    (
        __0x45,
        __0x00,
        __0x46,
        __0x46,
        __0x27,
        __0x47,
        __0x47,
        __0x22,
    ),
    (__0x48, __0x48, __0x0A),
);

#[cfg(all(feature = "min_const_generics", not(feature = "const_generics")))]
#[allow(dead_code)]
type SingleCharEscapes = (
    __<'A', '\n', 'B', '\r', 'C', '\t', 'D', '\\'>,
    __<'E', '\u{0}', 'F', 'F', '\'', 'G', 'G', '\"'>,
    __c<'H', 'H', '\n'>,
);

#[cfg(not(feature = "min_const_generics"))]
#[allow(dead_code)]
type SingleCharEscapesRaw = (
    (
        __0x41,
        __0x5C,
        __0x6E,
        __0x42,
        __0x5C,
        __0x72,
        __0x43,
        __0x5C,
    ),
    (
        __0x74,
        __0x44,
        __0x5C,
        __0x5C,
        __0x45,
        __0x5C,
        __0x30,
        __0x46,
    ),
    (
        __0x46,
        __0x5C,
        __0x27,
        __0x47,
        __0x47,
        __0x5C,
        __0x22,
        __0x48,
    ),
    (__0x48, __0x0A),
);

#[cfg(all(feature = "min_const_generics", not(feature = "const_generics")))]
#[allow(dead_code)]
type SingleCharEscapesRaw = (
    __<'A', '\\', 'n', 'B', '\\', 'r', 'C', '\\'>,
    __<'t', 'D', '\\', '\\', 'E', '\\', '0', 'F'>,
    __<'F', '\\', '\'', 'G', 'G', '\\', '\"', 'H'>,
    __b<'H', '\n'>,
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

#[cfg(not(feature = "min_const_generics"))]
#[allow(dead_code)]
type BackSlashNewline = (__f, __o, __o, __b, __a, __r);

#[cfg(all(feature = "min_const_generics", not(feature = "const_generics")))]
#[allow(dead_code)]
type BackSlashNewline = __f<'f', 'o', 'o', 'b', 'a', 'r'>;

#[cfg(not(feature = "min_const_generics"))]
#[allow(dead_code)]
type BackSlashNewlineRaw = (
    (__f, __o, __o, __0x5C, __0x0A, __0x20, __0x20, __0x20),
    (__0x20, __b, __a, __r),
);

#[cfg(all(feature = "min_const_generics", not(feature = "const_generics")))]
#[allow(dead_code)]
type BackSlashNewlineRaw = (
    __<'f', 'o', 'o', '\\', '\n', ' ', ' ', ' '>,
    __d<' ', 'b', 'a', 'r'>,
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

#[cfg(not(feature = "min_const_generics"))]
#[allow(dead_code)]
type Quotes = (__0x22, __0x22);

#[cfg(all(feature = "min_const_generics", not(feature = "const_generics")))]
#[allow(dead_code)]
type Quotes = __b<'"', '"'>;

#[cfg(not(feature = "min_const_generics"))]
#[allow(dead_code)]
type QuoteHash = (__0x22, __0x23);

#[cfg(all(feature = "min_const_generics", not(feature = "const_generics")))]
#[allow(dead_code)]
type QuoteHash = __b<'"', '#'>;

str_test_case! {r#""""#, Quotes}
str_test_case! {r##""""##   , Quotes}

str_test_case! {r##""#"##, QuoteHash}
str_test_case! {r###""#"###, QuoteHash}
