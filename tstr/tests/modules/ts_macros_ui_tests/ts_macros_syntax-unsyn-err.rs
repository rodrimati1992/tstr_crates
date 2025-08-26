// only tests the bits relevant to the proc macro

macro_rules! case {
    (($($tt1:tt)*) ($($tt2:tt)*)) => (
        let _: tstr::TS!($($tt1)*) = tstr::ts!($($tt2)*);
    )
}

#[allow(non_camel_case_types)]
fn main() {
    
    case!{ ((=)) ((=)) }
    case!{ (..) (..) }
    case!{ ({}) ({}) }
    case!{ (()) (()) }
    case!{ ([]) ([]) }
    case!{ (compile_error!{"foo"}) (compile_error!{"foo"}) }
    case!{ (foo::bar::compile_error!{"foo"}) (foo::bar::compile_error!{"foo"}) }
    case!{ (crate::bar::compile_error!{"foo"}) (crate::bar::compile_error!{"foo"}) }
}




