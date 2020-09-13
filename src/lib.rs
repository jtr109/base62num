//! A convertor between [`usize`] and [`String`] in [Base62].
//!
//! # Examples
//!
//! ```rust
//! use base62num::{encode, decode, Base62Error};
//!
//! // It can convert an usize number into a string in Base62.
//! assert_eq!(encode(123), "B9");
//!
//! // Vice versa.
//! assert_eq!(decode("B9"), Ok(123));
//!
//! // Any non-alphanumeric character in the string is invalid.
//! assert_eq!(decode("Base*62"), Err(Base62Error::NonAlphanumeric));
//!
//! // The conversion also failed if the result number is overflow.
//! assert_eq!(
//!     decode("AStringTooLongCausesTheOverflowError"),
//!     Err(Base62Error::Overflow)
//! );
//! ```
//! [`usize`]: https://doc.rust-lang.org/std/primitive.usize.html
//! [`String`]: https://doc.rust-lang.org/std/string/struct.String.html
//! [Base62]: https://en.wikipedia.org/wiki/Base62

use std::char;
use std::error::Error;
use std::fmt;

const ALPHANUMERIC: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
const BASE: usize = 62;

/// Conversion errors between numbers and strings in Base62.
#[derive(Debug, PartialEq)]
pub enum Base62Error {
    /// Occurs when decoding a string contains non-alphanumeric characters.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use base62num::{decode, Base62Error};
    ///
    /// assert_eq!(decode("Base*62"), Err(Base62Error::NonAlphanumeric));
    /// ```
    NonAlphanumeric,
    /// Occurs when the result of decoding is overflow.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use base62num::{decode, Base62Error};
    ///
    /// assert_eq!(
    ///     decode("AStringTooLongCausesTheOverflowError"),
    ///     Err(Base62Error::Overflow)
    /// );
    /// ```
    Overflow,
}

impl fmt::Display for Base62Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Base62Error::NonAlphanumeric => write!(f, "Input contains non-alphanumeric."),
            Base62Error::Overflow => write!(f, "Return is overflow."),
        }
    }
}

impl Error for Base62Error {
    fn description(&self) -> &str {
        match *self {
            Base62Error::NonAlphanumeric => "contains non-alphanumeric",
            Base62Error::Overflow => "overflow",
        }
    }
}

fn to_char(num: usize) -> Option<char> {
    ALPHANUMERIC.chars().nth(num)
}

/// Converts a number into a string in Base62.
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// use base62num::encode;
///
/// assert_eq!(encode(7), "H");
/// assert_eq!(encode(123), "B9");
/// ```
pub fn encode(num: usize) -> String {
    let mut digits = vec![];
    let mut n = num;
    while n > 0 {
        let rem = n % BASE;
        n = (n - rem) / BASE;
        match to_char(rem) {
            Some(c) => digits.push(c),
            None => unreachable!(),
        };
    }
    digits.iter().rev().collect()
}

fn to_num(c: char) -> Result<usize, Base62Error> {
    ALPHANUMERIC
        .find(|x| x == c)
        .ok_or(Base62Error::NonAlphanumeric)
}

/// Converts a string in Base62 into an number.
///
/// # Examples
///
/// Basic usage:
/// ```rust
/// use base62num::{decode, Base62Error};
///
/// assert_eq!(decode("H"), Ok(7));
/// assert_eq!(decode("B9"), Ok(123));
/// assert_eq!(decode("Base*62"), Err(Base62Error::NonAlphanumeric));
/// assert_eq!(
///     decode("AStringTooLongCausesTheOverflowError"),
///     Err(Base62Error::Overflow)
/// );
///
pub fn decode(input: &str) -> Result<usize, Base62Error> {
    input.chars().try_fold(0 as usize, |acc, c| {
        to_num(c).and_then(|x| {
            acc.checked_mul(BASE)
                .and_then(|mul| mul.checked_add(x))
                .ok_or(Base62Error::Overflow)
        })
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_pass() {
        assert_eq!(encode(7), "H");
        assert_eq!(encode(123), "B9");
    }

    #[test]
    fn decode_pass() {
        assert_eq!(decode("H"), Ok(7));
        assert_eq!(decode("B9"), Ok(123));
        assert_eq!(decode("Base*62"), Err(Base62Error::NonAlphanumeric));
        assert_eq!(
            decode("AStringTooLongCausesTheOverflowError"),
            Err(Base62Error::Overflow)
        );
    }
}
