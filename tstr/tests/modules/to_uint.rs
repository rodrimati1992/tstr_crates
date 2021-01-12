use tstr::{ts, ToUint};

fn same<T: ToUint>(val: T) -> usize {
    let v128 = val.to_u128();
    let vusize = val.to_usize();
    assert_eq!(vusize as u128, v128);
    vusize
}

#[test]
fn to_uint() {
    assert_eq!(same(ts!(0)), 0);
    assert_eq!(same(ts!(5)), 5);
    assert_eq!(same(ts!(23)), 23);
    assert_eq!(same(ts!(46)), 46);
    assert_eq!(same(ts!(50)), 50);
    assert_eq!(same(ts!(63)), 63);
    assert_eq!(same(ts!(64)), 64);
    assert_eq!(same(ts!(65)), 65);
    assert_eq!(same(ts!(511)), 511);
    assert_eq!(same(ts!(512)), 512);
    assert_eq!(same(ts!(513)), 513);
}

#[test]
#[cfg(target_pointer_width = "128")]
fn to_usize() {
    assert_eq!(
        same(ts!(0xFFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF)),
        usize::MAX
    );
    assert_eq!(
        same(ts!(0xFFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFE)),
        usize::MAX - 1
    );
}

#[test]
#[cfg(target_pointer_width = "64")]
fn to_usize() {
    assert_eq!(same(ts!(0xFFFF_FFFF_FFFF_FFFF)), usize::MAX);
    assert_eq!(same(ts!(0xFFFF_FFFF_FFFF_FFFE)), usize::MAX - 1);

    assert_eq!(ts!(0x1_0000_0000_0000_0000).to_usize(), usize::MAX);
    assert_eq!(ts!(0x1_0000_0000_0000_0001).to_usize(), usize::MAX);
}

#[test]
#[cfg(target_pointer_width = "32")]
fn to_usize() {
    assert_eq!(same(ts!(0xFFFF_FFFF)), usize::MAX);
    assert_eq!(same(ts!(0xFFFF_FFFE)), usize::MAX - 1);

    assert_eq!(ts!(0x1_0000_0000).to_usize(), usize::MAX);
    assert_eq!(ts!(0x1_0000_0001).to_usize(), usize::MAX);
}

#[test]
#[cfg(target_pointer_width = "16")]
fn to_usize() {
    assert_eq!(same(ts!(0xFFFF)), usize::MAX);
    assert_eq!(same(ts!(0xFFFE)), usize::MAX - 1);

    assert_eq!(ts!(0x1_0000).to_usize(), usize::MAX);
    assert_eq!(ts!(0x1_0001).to_usize(), usize::MAX);
}
