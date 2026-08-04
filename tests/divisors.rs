use kaadivisors::get_divisors;

#[test]
fn divisors() {
    assert_eq!(
        vec![(2u16, 3u8), (5u16, 3u8)],
        get_divisors(1000u16),
    );
}

#[test]
#[should_panic]
fn wrong_divisors() {
    assert_eq!(
        vec![(8u16, 1u8), (125u16, 1u8)],
        get_divisors(1000u16),
    );
}
