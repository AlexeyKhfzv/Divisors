//! # Kaadivisors - very fast algorithm for finding prime divisors of huge composite numbers.
//! 
//! `Algorithmic complexity: O(√n)`
//! 
//! Even if we cannot reduce the time to find divisors when the number is prime, we can do it when our number is composite.
//! So, this program finds all prime divisors by iterating throught the numbers up to the <u>root of the greatiest divisor</u>.
//! 
//! * **Firstly**, we check divisibility by 2 and 3, and then iterate in increments of 6, since all prime divisors are of the form 6k ± 1.  
//! * **Secondly**, we iterate through the numbers until the first divisor of our number is found.  
//! In this case, we divide the number by it as much as possible (function: get_power), and then continue iterating through the numbers up to the <u>root of the result</u>.  
//! * **Back to the previous step**
//! 
//! Oh, and this program use generics, so everyone can pass <u>any unsigned integer as an argument</u> (u8, u16, u32, u64, u128)!

use std::fmt;
pub use num_traits::{PrimInt, Unsigned, FromPrimitive};

/// # kaadivisors::Error
/// 
/// * It has only one variant - **PowerError**
/// * It is used when function get_power receives incorrect arguments (see the doc for details)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    PowerError(&'static str),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::PowerError(msg) => write!(f, "power obtaining error: {msg}"),
        }
    }
}

impl std::error::Error for Error {}

/// # This function calculates the power of the number
/// 
/// `Algorithmic complexity: O(log_m(n))`
/// 
/// * T: any type that implements traits PrimInt, Unsigned
/// * Input: number (&mut T), divisor (T)
/// * Output: Result Ok(u8) or Err(kaadivisors::Error::PowerError)
/// 
/// ## Input data type restrictions
/// Any type that implements traits PrimInt, Unsigned
/// 
/// # Returns kaadivisors::Error::PowerError, when
/// * The number is zero
/// * The divisor is equal to zero or one
/// * The number is greater than its divisor
/// 
/// # Example
/// ```
/// use kaadivisors::get_power;
/// 
/// let mut number = 8u8;
/// assert_eq!(
///     Ok(3u8),
///     get_power(&mut number, 2u8),
/// );
/// ```
pub fn get_power<T>(number: &mut T, divisor: T) -> Result<u8, Error>
where
    T: PrimInt + Unsigned,
{
    if number.is_zero() {
        return Err(Error::PowerError("the number cannot be equal to zero"));
    }
    if divisor < T::from(2u8).unwrap() {
        return Err(Error::PowerError("the divisor cannot be equal to zero or one"));
    }
    if *number < divisor {
        return Err(Error::PowerError("the number must be greater than its divisor"));
    }

    let mut power: u8 = 0; // u8 because u128::MAX < 2.pow(u8::MAX)
    loop {
        if (*number % divisor).is_zero() {
            *number = *number / divisor;
            power += 1;
        } else { break; }
    }

    Ok(power)
}

/// # This function calculates all prime divisors of the number.
/// 
/// `Algorithmic complexity: O(log_m(n))`
/// 
/// * T: any type that implements traits PrimInt, Unsigned, FromPrimitive
/// * Input: number (T)
/// * Output: Vec<(T, u8)> // Vec<(prime divisor, power)>
/// 
/// # Example
/// ```
/// use kaadivisors::get_divisors;
/// 
/// assert_eq!(
///     vec![(2u16, 3u8), (5u16, 3u8)],
///     get_divisors(1000u16),
/// );
/// ```
pub fn get_divisors<T>(mut number: T) -> Vec<(T, u8)>
where
    T: PrimInt + Unsigned + FromPrimitive,
{
    let two = T::from(2u8).unwrap();
    let three = T::from(3u8).unwrap();
    let four = T::from(4u8).unwrap();
    let five = T::from(5u8).unwrap();
    let six = T::from(6u8).unwrap();

    if number.is_zero() { return vec![]; }
    if number < four { return vec![(number, 1)]; }

    let mut result = vec![];

    if (number % two).is_zero() {
        result.push((
            two,
            get_power(&mut number, two).unwrap(),
        ));
    }
    if (number % three).is_zero() {
        result.push((
            three,
            get_power(&mut number, three).unwrap(),
        ));
    }

    let mut divisor = five;

    while divisor * divisor <= number {
        if (number % divisor).is_zero() {
            result.push((
                divisor,
                get_power(&mut number, divisor).unwrap(),
            ));
        }
        if (number % (divisor + two)).is_zero() {
            result.push((
                divisor + two,
                get_power(&mut number, divisor + two).unwrap(),
            ));
        }

        divisor = divisor + six;
    }

    if !number.is_one() {
        result.push((number, 1));
    }

    result
}
