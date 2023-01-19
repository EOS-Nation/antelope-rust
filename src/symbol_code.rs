// Reference: https://github.com/AntelopeIO/cdt/blob/main/libraries/eosiolib/core/eosio/symbol.hpp

#![allow(dead_code, unused)]
use std::clone::Clone;
use std::cmp::{Ord, Ordering, PartialEq, PartialOrd};
use std::convert::From;
use std::fmt::{Display, Formatter, Result};
use std::ops::Not;

use crate::check::check;

/// The `SymbolCode` struct represents a symbol code
///
/// A symbol code is a 64-bit unsigned integer that represents a symbol
///
/// The symbol code is used to represent a currency or asset
///
/// # Examples
///
/// ```
/// use antelope::symbol_code::SymbolCode;
///
/// let symcode = SymbolCode::new("FOO");
/// assert_eq!(5197638, symcode.raw());
/// assert_eq!(3, symcode.length());
/// assert_eq!(true, symcode.is_valid());
/// assert_eq!("FOO", symcode.to_string());
/// ```
#[derive(Eq)]
pub struct SymbolCode {
    /// The raw value of the symbol code
    ///
    /// The raw value is the underlying representation of the symbol code
    ///
    /// The raw value is an unsigned 64-bit integer
    ///
    /// # Examples
    ///
    /// ```
    /// use antelope::symbol_code::SymbolCode;
    ///
    /// let symcode = SymbolCode::new("FOO");
    /// assert_eq!(5197638, symcode.value);
    /// ```
    pub value: u64,
}

impl SymbolCode {
    /// Returns the raw value of the symbol code
    ///
    /// The raw value is the underlying representation of the symbol code
    ///
    /// The raw value is an unsigned 64-bit integer
    ///
    /// # Examples
    ///
    /// ```
    /// use antelope::symbol_code::SymbolCode;
    ///
    /// let symcode = SymbolCode::new("FOO");
    /// assert_eq!(5197638, symcode.raw());
    /// ```
    pub fn raw(&self) -> u64 {
        self.value
    }

    /// Returns the length of the symbol code
    ///
    /// The length is the number of characters in the symbol code
    ///
    /// The length is at most 7
    ///
    /// # Examples
    ///
    /// ```
    /// use antelope::symbol_code::SymbolCode;
    ///
    /// let len = SymbolCode::new("FOO").length();
    /// assert_eq!(3, len);
    /// ```
    pub fn length(&self) -> u32 {
        let mut sym: u64 = self.value;
        let mut len: u32 = 0;

        while sym & 0xFF > 0 && len <= 7 {
            len += 1;
            sym >>= 8;
        }
        len
    }

    /// Returns true if the symbol code is valid
    ///
    /// A symbol code is valid if it is not empty and contains only uppercase letters and has a length of at most 7
    ///
    /// # Examples
    ///
    /// ```
    /// use antelope::symbol_code::SymbolCode;
    ///
    /// let symcode = SymbolCode::new("FOO");
    /// assert_eq!(true, symcode.is_valid());
    /// ```
    pub fn is_valid(&self) -> bool {
        let mut sym: u64 = self.value;
        let mut i = 0;
        while i < 7 {
            let c = sym as u8 as char;
            if (!('A'..='Z').contains(&c)) {
                return false;
            }
            sym >>= 8;
            if sym == 0 {
                while i < 7 {
                    sym >>= 8;
                    if (sym & 0xFF) != 0 {
                        return false;
                    }
                    i += 1;
                }
            }
            i += 1;
        }
        true
    }

    /// Returns a new symbol code
    ///
    /// The new symbol code is constructed from the given string
    ///
    /// The string must be at most 7 characters long and contain only uppercase letters
    ///
    /// # Examples
    ///
    /// ```
    /// use antelope::symbol_code::SymbolCode;
    ///
    /// let symcode = SymbolCode::new("FOO");
    /// assert_eq!("FOO", symcode.to_string());
    pub fn new(str: &str) -> SymbolCode {
        SymbolCode::from(str)
    }
}

impl Display for SymbolCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mask = 0x00000000000000FF;
        if self.value == 0 {
            return Result::Ok(());
        }
        let mut begin = "".to_string();
        let mut v = self.value;
        let mut i = 0;

        while i < 7 {
            if v == 0 {
                break;
            }
            let c = (v & mask) as u8 as char;
            begin.push(c);
            v >>= 8;
            i += 1;
        }
        f.write_str(begin.as_str())
    }
}

impl Not for SymbolCode {
    type Output = bool;

    fn not(self) -> bool {
        self.value == 0
    }
}

impl Clone for SymbolCode {
    fn clone(&self) -> Self {
        SymbolCode { value: self.value }
    }
}

impl From<u64> for SymbolCode {
    fn from(value: u64) -> Self {
        SymbolCode { value }
    }
}

impl From<String> for SymbolCode {
    fn from(str: String) -> Self {
        let mut value: u64 = 0;
        check(
            str.len() <= 7,
            "string is too long to be a valid symbol_code",
        );
        for c in str.chars().rev() {
            check(
                ('A'..='Z').contains(&c),
                "only uppercase letters allowed in symbol_code string",
            );
            value <<= 8;
            value |= c as u64;
        }
        SymbolCode { value }
    }
}

impl From<&str> for SymbolCode {
    fn from(str: &str) -> Self {
        SymbolCode::from(str.to_string())
    }
}

impl PartialEq for SymbolCode {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl PartialOrd for SymbolCode {
    fn partial_cmp(&self, other: &SymbolCode) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SymbolCode {
    fn cmp(&self, other: &SymbolCode) -> Ordering {
        self.value.cmp(&other.value)
    }
}

impl From<SymbolCode> for bool {
    fn from(_other: SymbolCode) -> bool {
        _other.is_valid()
    }
}

trait AsBool {
    fn as_bool(&self) -> bool;
}

impl AsBool for SymbolCode {
    fn as_bool(&self) -> bool {
        self.is_valid()
    }
}

impl AsRef<SymbolCode> for SymbolCode {
    fn as_ref(&self) -> &SymbolCode {
        self
    }
}

#[test]
fn test_as_ref() {
    assert_eq!(5197638, SymbolCode::from("FOO").as_ref().value);
    assert_eq!(5390658, SymbolCode::from("BAR").as_ref().value);
}

#[test]
fn test_clone() {
    SymbolCode::new("FOO").clone();
}

#[test]
fn test_some_unwrap() {
    let symcode = Some(SymbolCode::new("FOO"));
}

#[test]
fn test_length() {
    assert_eq!(1, SymbolCode::from("A").length());
    assert_eq!(2, SymbolCode::from("AB").length());
    assert_eq!(3, SymbolCode::from("ABC").length());
    assert_eq!(4, SymbolCode::from("ABCD").length());
    assert_eq!(5, SymbolCode::from("ABCDE").length());
    assert_eq!(6, SymbolCode::from("ABCDEF").length());
    assert_eq!(7, SymbolCode::from("ABCDEFG").length());
}

#[test]
fn test_not() {
    assert_eq!(true, SymbolCode::from(0).not());
    assert_eq!(false, SymbolCode::from(5197638).not());
}

#[test]
fn test_partial_eq() {
    assert_eq!(true, SymbolCode::from(5197638) == SymbolCode::from(5197638));
    assert_eq!(true, SymbolCode::from(0) != SymbolCode::from(5197638));
    assert_eq!(false, SymbolCode::from(0) == SymbolCode::from(5197638));
}

#[test]
fn test_partial_cmp() {
    assert_eq!(true, SymbolCode::from(0) < SymbolCode::from(1));
    assert_eq!(false, SymbolCode::from(3) < SymbolCode::from(2));
}

#[test]
fn test_new() {
    assert_eq!("FOO", SymbolCode::new("FOO").to_string());
    assert_eq!(5197638, SymbolCode::new("FOO").raw());
}

#[test]
fn test_fmt() {
    assert_eq!("FOO", format!("{}", SymbolCode::new("FOO")));
}

#[test]
fn test_println() {
    println!("{}", SymbolCode::new("FOO"));
}

#[test]
fn test_as_bool() {
    assert_eq!(true, SymbolCode::from(5197638).as_bool());
}

#[test]
fn test_into_bool() {
    assert_eq!(true, SymbolCode::from(5197638).into());
    assert_eq!(false, SymbolCode::from(0).into());
    if SymbolCode::from(0).into() {
        panic!("SymbolCode::from( 0 ) should be false");
    }
}

#[test]
fn test_if_bool() {
    if SymbolCode::from(0).into() {
        panic!("SymbolCode::from( 0 ) should be false");
    }
}

#[test]
#[should_panic]
fn test_if_bool_panic() {
    if SymbolCode::from(5197638).into() {
        panic!("SymbolCode::from( 5197638 ) should be true");
    }
}

#[test]
fn test_from() {
    assert_eq!(0, SymbolCode::from(0).value);
    assert_eq!(0, SymbolCode::from(0).raw());
    assert_eq!(0, SymbolCode::from("".to_string()).raw());
    assert_eq!(5197638, SymbolCode::from("FOO".to_string()).value);
    assert_eq!(5197638, SymbolCode::from(5197638).raw());
}

#[test]
fn test_is_valid() {
    assert_eq!(false, SymbolCode::from(0).is_valid());
    assert_eq!(true, SymbolCode::from("FOO".to_string()).is_valid());
}

#[test]
fn test_to_string() {
    assert_eq!("FOO", SymbolCode::from(5197638).to_string());
    assert_eq!("FOO", SymbolCode::from("FOO".to_string()).to_string());
    assert_eq!(
        "ABCDEFG",
        SymbolCode::from("ABCDEFG".to_string()).to_string()
    );
}

#[test]
fn test_to_str() {
    assert_eq!("FOO", SymbolCode::from("FOO").to_string());
}

#[test]
#[should_panic(expected = "string is too long to be a valid symbol_code")]
fn test_from_string_long_panic_1() {
    SymbolCode::from("ABCDEFGH".to_string());
}

#[test]
#[should_panic(expected = "only uppercase letters allowed in symbol_code string")]
fn test_from_string_letters_panic_1() {
    SymbolCode::from("abc".to_string());
}

#[test]
#[should_panic(expected = "only uppercase letters allowed in symbol_code string")]
fn test_from_string_letters_panic_2() {
    SymbolCode::from("123".to_string());
}
