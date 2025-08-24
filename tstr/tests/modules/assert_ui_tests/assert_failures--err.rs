use tstr::ts;


const _: () = tstr::assert_str_eq!(ts!(uh), "what", "hello", {?}: "how are you");

const _: () = tstr::assert_str_ne!("no", ts!(no), 10u8, " ", {#X}: 20u16);


fn main() {}
