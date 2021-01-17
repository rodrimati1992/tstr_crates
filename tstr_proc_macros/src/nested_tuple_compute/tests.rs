use super::find_smaller_power;

#[test]
fn test_find_smaller_power() {
    assert_eq!(find_smaller_power(1), 1);
    assert_eq!(find_smaller_power(5), 1);
    assert_eq!(find_smaller_power(7), 1);
    assert_eq!(find_smaller_power(8), 1);
    assert_eq!(find_smaller_power(9), 8);
    assert_eq!(find_smaller_power(63), 8);
    assert_eq!(find_smaller_power(64), 8);
    assert_eq!(find_smaller_power(65), 64);
    assert_eq!(find_smaller_power(511), 64);
    assert_eq!(find_smaller_power(512), 64);
    assert_eq!(find_smaller_power(513), 512);
    assert_eq!(find_smaller_power(4095), 512);
    assert_eq!(find_smaller_power(4096), 512);
    assert_eq!(find_smaller_power(4097), 4096);
}
