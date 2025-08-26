use tstr::{IsTStr, TS};


const fn eq_fail<S: IsTStr>() {
    tstr::assert_str_eq!(S::VAL, "what", "hello", {?}: "how are you");
}
const fn ne_fail<S: IsTStr>() {
    tstr::assert_str_ne!("no", S::VAL, 10u8, " ", {#X}: 20u16);
}

const _: () = eq_fail::<TS!(no)>();
const _: () = ne_fail::<TS!(no)>();

fn main() {
}
