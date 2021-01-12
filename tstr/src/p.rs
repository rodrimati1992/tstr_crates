// This file defines many types that are `#[doc(hidden)] pub`
// and required not to be used by users.

/// This type must not be used by name outside of `structural` macros.
// `TStr` takes this as a type parameter so that
// this library can start using const generics in the future by replacing the
// `T:?Sized` parameter with `const STR:&'static str`.
#[doc(hidden)]
#[cfg(not(feature = "const_generics"))]
pub struct __<T: ?Sized>(core::marker::PhantomData<T>);

// Used inside structural in tests and impls.
#[doc(hidden)]
#[cfg(not(feature = "const_generics"))]
pub(crate) type __TStrPriv<T> = TStr<__<T>>;

// macros can contain arbitrary syntax,
// which allows this to be defined in this file even if Rust stops parsing `const IDENT:Foo`
#[cfg(feature = "const_generics")]
macro_rules! declare_const_items {
    () => {
        #[doc(hidden)]
        pub(crate) type __TStrPriv<const S: &'static str> = TStr<__<S>>;

        // `TStr` takes this as a type parameter so that
        // this library can start using const generics in the future by replacing the
        // `T:?Sized` parameter with `const STR:&'static str`.
        #[doc(hidden)]
        pub struct __<const S: &'static str>;
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
                format!("(__{1},__0x{0:02X}),",b,b as char)
            }else{
                format!("(__0x{0:02X}),",b)
            }
        })
        .collect::<Vec<_>>();
    for chunk in list.chunks(6) {
        for param in chunk {
            print!("{}",param);
        }
        println!();
    }
}


*/

#[cfg(not(feature = "const_generics"))]
macro_rules! create_unit_struct {
    ($( ($struct_:ident $(,$alias:ident)? ) ),* $(,)*) => {
        $(
            #[doc(hidden)]
            pub struct $struct_;

            impl Copy for $struct_{}
            impl Clone for $struct_{
                fn clone(&self) -> $struct_ {
                    $struct_
                }
            }

            $(
                #[doc(hidden)]
                pub type $alias=$struct_;
            )?
        )*
    }
}

#[cfg(not(feature = "const_generics"))]
create_unit_struct! {
    (__0x00),(__0x01),(__0x02),(__0x03),(__0x04),(__0x05),
    (__0x06),(__0x07),(__0x08),(__0x09),(__0x0A),(__0x0B),
    (__0x0C),(__0x0D),(__0x0E),(__0x0F),(__0x10),(__0x11),
    (__0x12),(__0x13),(__0x14),(__0x15),(__0x16),(__0x17),
    (__0x18),(__0x19),(__0x1A),(__0x1B),(__0x1C),(__0x1D),
    (__0x1E),(__0x1F),(__0x20),(__0x21),(__0x22),(__0x23),
    (__0x24),(__0x25),(__0x26),(__0x27),(__0x28),(__0x29),
    (__0x2A),(__0x2B),(__0x2C),(__0x2D),(__0x2E),(__0x2F),
    (__0,__0x30),(__1,__0x31),(__2,__0x32),(__3,__0x33),(__4,__0x34),(__5,__0x35),
    (__6,__0x36),(__7,__0x37),(__8,__0x38),(__9,__0x39),(__0x3A),(__0x3B),
    (__0x3C),(__0x3D),(__0x3E),(__0x3F),(__0x40),(__A,__0x41),
    (__B,__0x42),(__C,__0x43),(__D,__0x44),(__E,__0x45),(__F,__0x46),(__G,__0x47),
    (__H,__0x48),(__I,__0x49),(__J,__0x4A),(__K,__0x4B),(__L,__0x4C),(__M,__0x4D),
    (__N,__0x4E),(__O,__0x4F),(__P,__0x50),(__Q,__0x51),(__R,__0x52),(__S,__0x53),
    (__T,__0x54),(__U,__0x55),(__V,__0x56),(__W,__0x57),(__X,__0x58),(__Y,__0x59),
    (__Z,__0x5A),(__0x5B),(__0x5C),(__0x5D),(__0x5E),(___,__0x5F),
    (__0x60),(__a,__0x61),(__b,__0x62),(__c,__0x63),(__d,__0x64),(__e,__0x65),
    (__f,__0x66),(__g,__0x67),(__h,__0x68),(__i,__0x69),(__j,__0x6A),(__k,__0x6B),
    (__l,__0x6C),(__m,__0x6D),(__n,__0x6E),(__o,__0x6F),(__p,__0x70),(__q,__0x71),
    (__r,__0x72),(__s,__0x73),(__t,__0x74),(__u,__0x75),(__v,__0x76),(__w,__0x77),
    (__x,__0x78),(__y,__0x79),(__z,__0x7A),(__0x7B),(__0x7C),(__0x7D),
    (__0x7E),(__0x7F),(__0x80),(__0x81),(__0x82),(__0x83),
    (__0x84),(__0x85),(__0x86),(__0x87),(__0x88),(__0x89),
    (__0x8A),(__0x8B),(__0x8C),(__0x8D),(__0x8E),(__0x8F),
    (__0x90),(__0x91),(__0x92),(__0x93),(__0x94),(__0x95),
    (__0x96),(__0x97),(__0x98),(__0x99),(__0x9A),(__0x9B),
    (__0x9C),(__0x9D),(__0x9E),(__0x9F),(__0xA0),(__0xA1),
    (__0xA2),(__0xA3),(__0xA4),(__0xA5),(__0xA6),(__0xA7),
    (__0xA8),(__0xA9),(__0xAA),(__0xAB),(__0xAC),(__0xAD),
    (__0xAE),(__0xAF),(__0xB0),(__0xB1),(__0xB2),(__0xB3),
    (__0xB4),(__0xB5),(__0xB6),(__0xB7),(__0xB8),(__0xB9),
    (__0xBA),(__0xBB),(__0xBC),(__0xBD),(__0xBE),(__0xBF),
    (__0xC0),(__0xC1),(__0xC2),(__0xC3),(__0xC4),(__0xC5),
    (__0xC6),(__0xC7),(__0xC8),(__0xC9),(__0xCA),(__0xCB),
    (__0xCC),(__0xCD),(__0xCE),(__0xCF),(__0xD0),(__0xD1),
    (__0xD2),(__0xD3),(__0xD4),(__0xD5),(__0xD6),(__0xD7),
    (__0xD8),(__0xD9),(__0xDA),(__0xDB),(__0xDC),(__0xDD),
    (__0xDE),(__0xDF),(__0xE0),(__0xE1),(__0xE2),(__0xE3),
    (__0xE4),(__0xE5),(__0xE6),(__0xE7),(__0xE8),(__0xE9),
    (__0xEA),(__0xEB),(__0xEC),(__0xED),(__0xEE),(__0xEF),
    (__0xF0),(__0xF1),(__0xF2),(__0xF3),(__0xF4),(__0xF5),
    (__0xF6),(__0xF7),(__0xF8),(__0xF9),(__0xFA),(__0xFB),
    (__0xFC),(__0xFD),(__0xFE),(__0xFF),
}

///////////////////////////////////////////////////////////////////////////////
