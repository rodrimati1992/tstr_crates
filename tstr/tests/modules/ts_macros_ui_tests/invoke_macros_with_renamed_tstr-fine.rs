extern crate tstr as  tat;
extern crate std as  tstr;

// making sure that tstr actually refers to the standard library.
const _: () = tstr::assert!(true);
const _: () = ::tstr::assert!(true);
const _: Option<()> = tstr::option::Option::None;
const _: Option<()> = ::tstr::option::Option::None;

const _: () = assert!(matches!(tat::strlike::as_str!("heh").as_bytes(), b"heh"));
const _: () = tat::assert_str_eq!("heh", "heh");
const _: () = tat::assert_str_ne!("heh", "no");

const _: () = {
    tat::alias!{
        pub Foo = "mario";
    }

    assert!(matches!(tat::to_bytes(Foo), b"mario"));
};

const _: () = {
    let ts: tat::TStr<_> = tat::ts!(20);
    assert!(matches!(tat::to_bytes(ts), b"20"));
};

const _: () = {
    let ts: tat::TStr<_> = <tat::TS!(wow)>::new();
    assert!(matches!(tat::to_bytes(ts), b"wow"));
};



fn main(){}





