// only tests the bits relevant to the proc macro

use tstr::alias;

alias!{ pub A = A; }

mod sub {
    tstr::alias!{ S_B_PRIV = B; }
    tstr::alias!{ pub S_C = C; }
    tstr::alias!{ pub(super) S_D = D; }
    tstr::alias!{ pub(crate) S_E = E; }
    
    mod subsub {
        tstr::alias!{ SS_B_PRIV = B; }
        tstr::alias!{ pub SS_C = C; }
        tstr::alias!{ pub(super) SS_D = D; }
        tstr::alias!{ pub(crate) SS_E = E; }
    }

    const _: () = {
        assert!(matches!(tstr::to_bytes(subsub::SS_B_PRIV), b"B"));
        assert!(matches!(tstr::to_bytes(subsub::SS_C), b"C"));
        assert!(matches!(tstr::to_bytes(subsub::SS_D), b"D"));
        assert!(matches!(tstr::to_bytes(subsub::SS_E), b"E"));
    };
}

const _: () = {
    assert!(matches!(tstr::to_bytes(sub::S_B_PRIV), b"B"));
    assert!(matches!(tstr::to_bytes(sub::S_C), b"C"));
    assert!(matches!(tstr::to_bytes(sub::S_D), b"D"));
    assert!(matches!(tstr::to_bytes(sub::S_E), b"E"));

    assert!(matches!(tstr::to_bytes(sub::subsub::SS_B_PRIV), b"B"));
    assert!(matches!(tstr::to_bytes(sub::subsub::SS_C), b"C"));
    assert!(matches!(tstr::to_bytes(sub::subsub::SS_D), b"D"));
    assert!(matches!(tstr::to_bytes(sub::subsub::SS_E), b"E"));
};
fn main() {}




