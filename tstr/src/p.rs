// This file defines many types that are `#[doc(hidden)] pub`
// and required not to be used by users.

#[cfg(all(feature = "min_const_generics", not(feature = "const_generics")))]
macro_rules! declare_min_const {
    (
        [$($chars_structs:ident [$($chars:ident),*],)*]
    )=>{
        $(
            #[doc(hidden)]
            pub struct $chars_structs<$(const $chars: char,)*>;
        )*
    }
}

#[cfg(all(feature = "min_const_generics", not(feature = "const_generics")))]
declare_min_const!{
    [
        __a[A],
        __b[A,B],
        __c[A,B,C],
        __d[A,B,C,D],
        __e[A,B,C,D,E],
        __f[A,B,C,D,E,F],
        __g[A,B,C,D,E,F,G],
        __[A,B,C,D,E,F,G,H],
    ]
}


// macros can contain arbitrary syntax,
// which allows this to be defined in this file even if Rust stops parsing `const IDENT:Foo`
#[cfg(feature = "const_generics")]
macro_rules! declare_const_items {
    () => {
        // `TStr` takes this as a type parameter so that
        // this library can start using const generics in the future by replacing the
        // `T:?Sized` parameter with `const STR:&'static str`.
        #[doc(hidden)]
        pub struct ___<const S: &'static str>;

        impl<const N: &'static str> Copy for ___<N> {}
        impl<const N: &'static str> Clone for ___<N> {
            fn clone(&self) -> Self {
                ___
            }
        }
    };
}

#[cfg(feature = "const_generics")]
declare_const_items! {}

///////////////////////////////////////////////////////////////////////////////
//
//                  Type Level Characters
//
///////////////////////////////////////////////////////////////////////////////

/*
Type-level ascii characters and bytes.

*/

/*

This is code used to generate the macro invocation.

fn main() {
    let mut list=(0..=255u8)
        .map(|b|{
            let c=b as char;
            if (c.is_alphanumeric() || c=='_') && b<128 {
                let for_und = if c=='_' { "_" } else { "" };
                format!("(__{1}{2} = {0},__0x{0:02X}),", b, b as char, for_und)
            }else{
                format!("(__0x{0:02X} = {0}),",b)
            }
        })
        .collect::<Vec<_>>();
    for chunk in list.chunks(4) {
        for param in chunk {
            print!("{}",param);
        }
        println!();
    }
}


*/

#[cfg(not(feature = "min_const_generics"))]
macro_rules! create_unit_struct {
    ($( ($struct_:ident = $value:literal $(,$alias:ident)? ) ),* $(,)*) => {
        $(
            #[doc(hidden)]
            pub struct $struct_;

            #[cfg(feature = "cmp_traits")]
            impl crate::tstr_cmp::U8Repr for $struct_ {
                const REPR: u8 = $value;
            }

            #[cfg(feature = "cmp_traits")]
            impl crate::for_tupled_reprs::classify::Classify for $struct_{
                type IsByte = crate::for_tupled_reprs::True;
                type KindNumber = crate::for_tupled_reprs::classify::Chars1;
            }

            $(
                #[doc(hidden)]
                pub type $alias=$struct_;
            )?
        )*
    }
}

#[cfg(not(feature = "min_const_generics"))]
create_unit_struct! {
    (__0x00 = 0),(__0x01 = 1),(__0x02 = 2),(__0x03 = 3),
    (__0x04 = 4),(__0x05 = 5),(__0x06 = 6),(__0x07 = 7),
    (__0x08 = 8),(__0x09 = 9),(__0x0A = 10),(__0x0B = 11),
    (__0x0C = 12),(__0x0D = 13),(__0x0E = 14),(__0x0F = 15),
    (__0x10 = 16),(__0x11 = 17),(__0x12 = 18),(__0x13 = 19),
    (__0x14 = 20),(__0x15 = 21),(__0x16 = 22),(__0x17 = 23),
    (__0x18 = 24),(__0x19 = 25),(__0x1A = 26),(__0x1B = 27),
    (__0x1C = 28),(__0x1D = 29),(__0x1E = 30),(__0x1F = 31),
    (__0x20 = 32),(__0x21 = 33),(__0x22 = 34),(__0x23 = 35),
    (__0x24 = 36),(__0x25 = 37),(__0x26 = 38),(__0x27 = 39),
    (__0x28 = 40),(__0x29 = 41),(__0x2A = 42),(__0x2B = 43),
    (__0x2C = 44),(__0x2D = 45),(__0x2E = 46),(__0x2F = 47),
    (__0 = 48,__0x30),(__1 = 49,__0x31),(__2 = 50,__0x32),(__3 = 51,__0x33),
    (__4 = 52,__0x34),(__5 = 53,__0x35),(__6 = 54,__0x36),(__7 = 55,__0x37),
    (__8 = 56,__0x38),(__9 = 57,__0x39),(__0x3A = 58),(__0x3B = 59),
    (__0x3C = 60),(__0x3D = 61),(__0x3E = 62),(__0x3F = 63),
    (__0x40 = 64),(__A = 65,__0x41),(__B = 66,__0x42),(__C = 67,__0x43),
    (__D = 68,__0x44),(__E = 69,__0x45),(__F = 70,__0x46),(__G = 71,__0x47),
    (__H = 72,__0x48),(__I = 73,__0x49),(__J = 74,__0x4A),(__K = 75,__0x4B),
    (__L = 76,__0x4C),(__M = 77,__0x4D),(__N = 78,__0x4E),(__O = 79,__0x4F),
    (__P = 80,__0x50),(__Q = 81,__0x51),(__R = 82,__0x52),(__S = 83,__0x53),
    (__T = 84,__0x54),(__U = 85,__0x55),(__V = 86,__0x56),(__W = 87,__0x57),
    (__X = 88,__0x58),(__Y = 89,__0x59),(__Z = 90,__0x5A),(__0x5B = 91),
    (__0x5C = 92),(__0x5D = 93),(__0x5E = 94),(____ = 95,__0x5F),
    (__0x60 = 96),(__a = 97,__0x61),(__b = 98,__0x62),(__c = 99,__0x63),
    (__d = 100,__0x64),(__e = 101,__0x65),(__f = 102,__0x66),(__g = 103,__0x67),
    (__h = 104,__0x68),(__i = 105,__0x69),(__j = 106,__0x6A),(__k = 107,__0x6B),
    (__l = 108,__0x6C),(__m = 109,__0x6D),(__n = 110,__0x6E),(__o = 111,__0x6F),
    (__p = 112,__0x70),(__q = 113,__0x71),(__r = 114,__0x72),(__s = 115,__0x73),
    (__t = 116,__0x74),(__u = 117,__0x75),(__v = 118,__0x76),(__w = 119,__0x77),
    (__x = 120,__0x78),(__y = 121,__0x79),(__z = 122,__0x7A),(__0x7B = 123),
    (__0x7C = 124),(__0x7D = 125),(__0x7E = 126),(__0x7F = 127),
    (__0x80 = 128),(__0x81 = 129),(__0x82 = 130),(__0x83 = 131),
    (__0x84 = 132),(__0x85 = 133),(__0x86 = 134),(__0x87 = 135),
    (__0x88 = 136),(__0x89 = 137),(__0x8A = 138),(__0x8B = 139),
    (__0x8C = 140),(__0x8D = 141),(__0x8E = 142),(__0x8F = 143),
    (__0x90 = 144),(__0x91 = 145),(__0x92 = 146),(__0x93 = 147),
    (__0x94 = 148),(__0x95 = 149),(__0x96 = 150),(__0x97 = 151),
    (__0x98 = 152),(__0x99 = 153),(__0x9A = 154),(__0x9B = 155),
    (__0x9C = 156),(__0x9D = 157),(__0x9E = 158),(__0x9F = 159),
    (__0xA0 = 160),(__0xA1 = 161),(__0xA2 = 162),(__0xA3 = 163),
    (__0xA4 = 164),(__0xA5 = 165),(__0xA6 = 166),(__0xA7 = 167),
    (__0xA8 = 168),(__0xA9 = 169),(__0xAA = 170),(__0xAB = 171),
    (__0xAC = 172),(__0xAD = 173),(__0xAE = 174),(__0xAF = 175),
    (__0xB0 = 176),(__0xB1 = 177),(__0xB2 = 178),(__0xB3 = 179),
    (__0xB4 = 180),(__0xB5 = 181),(__0xB6 = 182),(__0xB7 = 183),
    (__0xB8 = 184),(__0xB9 = 185),(__0xBA = 186),(__0xBB = 187),
    (__0xBC = 188),(__0xBD = 189),(__0xBE = 190),(__0xBF = 191),
    (__0xC0 = 192),(__0xC1 = 193),(__0xC2 = 194),(__0xC3 = 195),
    (__0xC4 = 196),(__0xC5 = 197),(__0xC6 = 198),(__0xC7 = 199),
    (__0xC8 = 200),(__0xC9 = 201),(__0xCA = 202),(__0xCB = 203),
    (__0xCC = 204),(__0xCD = 205),(__0xCE = 206),(__0xCF = 207),
    (__0xD0 = 208),(__0xD1 = 209),(__0xD2 = 210),(__0xD3 = 211),
    (__0xD4 = 212),(__0xD5 = 213),(__0xD6 = 214),(__0xD7 = 215),
    (__0xD8 = 216),(__0xD9 = 217),(__0xDA = 218),(__0xDB = 219),
    (__0xDC = 220),(__0xDD = 221),(__0xDE = 222),(__0xDF = 223),
    (__0xE0 = 224),(__0xE1 = 225),(__0xE2 = 226),(__0xE3 = 227),
    (__0xE4 = 228),(__0xE5 = 229),(__0xE6 = 230),(__0xE7 = 231),
    (__0xE8 = 232),(__0xE9 = 233),(__0xEA = 234),(__0xEB = 235),
    (__0xEC = 236),(__0xED = 237),(__0xEE = 238),(__0xEF = 239),
    (__0xF0 = 240),(__0xF1 = 241),(__0xF2 = 242),(__0xF3 = 243),
    (__0xF4 = 244),(__0xF5 = 245),(__0xF6 = 246),(__0xF7 = 247),
    (__0xF8 = 248),(__0xF9 = 249),(__0xFA = 250),(__0xFB = 251),
    (__0xFC = 252),(__0xFD = 253),(__0xFE = 254),(__0xFF = 255),
}

///////////////////////////////////////////////////////////////////////////////
