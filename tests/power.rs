use kaadivisors::{get_power, Error::PowerError};

#[test]
fn power() {
    let mut number = 8u8;
    assert_eq!(
        Ok(3u8),
        get_power(&mut number, 2u8),
    );
}

#[test]
#[should_panic]
fn incorrect_power() {
    let mut number = 1024u16;
    assert_eq!(
        Ok(11u8),
        get_power(&mut number, 2u16),
    );
}

#[test]
fn incorrect_args() {
    let mut incorrect_number = 0u8; // number = 0
    assert_eq!(
        Err(PowerError("the number cannot be equal to zero")),
        get_power(&mut incorrect_number, 0u8),
    );

    let mut number = 10u8;
    assert_eq!(
        Err(PowerError("the divisor cannot be equal to zero or one")),
        get_power(&mut number, 0u8), // divisor = 0
    );
    assert_eq!(
        Err(PowerError("the divisor cannot be equal to zero or one")),
        get_power(&mut number, 1u8), // divisor = 1
    );

    let (mut incorrect_number, incorrect_divisor) = (10u8, 20u8); // number < divisor
    assert_eq!(
        Err(PowerError("the number must be greater than its divisor")),
        get_power(&mut incorrect_number, incorrect_divisor),
    );
}
