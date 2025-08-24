// only tests the bits relevant to the proc macro

use tstr::alias;

#[allow(non_camel_case_types)]
fn main() {
    
    alias!{ pub _equals = (=); }
    alias!{ pub _dotdot = ..; }
    alias!{ pub _braces = {}; }
    alias!{ pub _parentheses = (); }
    alias!{ pub _brackets = []; }
    alias!{ pub _compile_err1 = compile_error!{"foo"}; }
    alias!{ pub _compile_err2 = foo::bar::compile_error!{"foo"}; }
    alias!{ pub _compile_err3 = crate::bar::compile_error!{"foo"}; }
}




