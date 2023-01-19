#![allow(dead_code, unused)]
use std::cmp::{Ord, Ordering, PartialEq, PartialOrd};
use std::convert::From;
use std::fmt::{Display, Formatter, Result};

use crate::check;

/// The `SymbolCode` struct represents a symbol code
///
/// Reference: <https://github.com/AntelopeIO/cdt/blob/main/libraries/eosiolib/core/eosio/symbol.hpp>
///
/// A symbol code is a 64-bit unsigned integer that represents a symbol
///
/// The symbol code is used to represent a currency or asset
///
/// # Examples
///
/// ```
/// use antelope::SymbolCode;
///
/// let symcode = SymbolCode::from("FOO");
/// assert_eq!(5197638, symcode.raw());
/// assert_eq!(3, symcode.length());
/// assert_eq!(true, symcode.is_valid());
/// assert_eq!("FOO", symcode.to_string());
/// ```
#[derive(Eq, Copy, Clone, Debug, PartialEq, PartialOrd, Ord, Default)]
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
    /// use antelope::SymbolCode;
    ///
    /// let symcode = SymbolCode::from("FOO");
    /// assert_eq!(5197638, symcode.raw());
    /// ```
    value: u64,
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
    /// use antelope::SymbolCode;
    ///
    /// let symcode = SymbolCode::from("FOO");
    /// assert_eq!(5197638, symcode.raw());
    /// ```
    #[must_use]
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
    /// use antelope::SymbolCode;
    ///
    /// let len = SymbolCode::from("FOO").length();
    /// assert_eq!(3, len);
    /// ```
    #[must_use]
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
    /// use antelope::SymbolCode;
    ///
    /// let symcode = SymbolCode::from("FOO");
    /// assert_eq!(true, symcode.is_valid());
    /// ```
    #[must_use]
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
    /// use antelope::SymbolCode;
    ///
    /// let symcode = SymbolCode::new();
    /// assert_eq!("", symcode.to_string());
    #[must_use]
    pub fn new() -> Self {
        Self { value: 0 }
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

impl From<&str> for SymbolCode {
    fn from(str: &str) -> Self {
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

impl From<u64> for SymbolCode {
    fn from(value: u64) -> Self {
        SymbolCode { value }
    }
}

impl AsRef<SymbolCode> for SymbolCode {
    fn as_ref(&self) -> &SymbolCode {
        self
    }
}

#[cfg(test)]
mod symbol_code_tests {
    use super::*;
    #[test]
    fn test_as_ref() {
        assert_eq!(5197638, SymbolCode::from("FOO").as_ref().value);
        assert_eq!(5390658, SymbolCode::from("BAR").as_ref().value);
    }

    #[test]
    fn test_clone() {
        SymbolCode::from("FOO").clone();
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
    fn test_partial_eq() {
        assert_eq!(true, SymbolCode::from(5197638) == SymbolCode::from(5197638));
        assert_eq!(true, SymbolCode::from(0) != SymbolCode::from(5197638));
        assert_eq!(false, SymbolCode::from(0) == SymbolCode::from(5197638));
        assert_eq!(true, SymbolCode::from(5197637) != SymbolCode::from(5197638));
    }

    #[test]
    fn test_partial_cmp() {
        assert_eq!(true, SymbolCode::from(0) < SymbolCode::from(1));
        assert_eq!(false, SymbolCode::from(3) < SymbolCode::from(2));
    }

    #[test]
    fn test_new() {
        assert_eq!("FOO", SymbolCode::from("FOO").to_string());
        assert_eq!(5197638, SymbolCode::from("FOO").raw());
    }

    #[test]
    fn test_fmt() {
        assert_eq!("FOO", format!("{}", SymbolCode::from("FOO")));
    }

    #[test]
    fn test_println() {
        println!("{}", SymbolCode::from("FOO"));
    }

    #[test]
    fn test_from() {
        assert_eq!(0, SymbolCode::from(0).value);
        assert_eq!(0, SymbolCode::from(0).raw());
        assert_eq!(0, SymbolCode::from("").raw());
        assert_eq!(5197638, SymbolCode::from("FOO").value);
        assert_eq!(5197638, SymbolCode::from(5197638).raw());
    }

    #[test]
    fn test_is_valid() {
        assert_eq!(false, SymbolCode::new().is_valid());
        assert_eq!(false, SymbolCode::from(0).is_valid());
        assert_eq!(false, SymbolCode::from(std::u64::MAX).is_valid());
        assert_eq!(true, SymbolCode::from(5197638).is_valid());
        assert_eq!(true, SymbolCode::from("FOO").is_valid());
    }

    #[test]
    fn test_to_string() {
        assert_eq!("FOO", SymbolCode::from(5197638).to_string());
        assert_eq!("FOO", SymbolCode::from("FOO").to_string());
        assert_eq!("ABCDEFG", SymbolCode::from("ABCDEFG").to_string());
    }

    #[test]
    fn test_to_str() {
        assert_eq!("FOO", SymbolCode::from("FOO").to_string());
    }

    #[test]
    #[should_panic(expected = "string is too long to be a valid symbol_code")]
    fn test_from_string_long_panic_1() {
        SymbolCode::from("ABCDEFGH");
    }

    #[test]
    #[should_panic(expected = "only uppercase letters allowed in symbol_code string")]
    fn test_from_string_letters_panic_1() {
        SymbolCode::from("abc");
    }

    #[test]
    #[should_panic(expected = "only uppercase letters allowed in symbol_code string")]
    fn test_from_string_letters_panic_2() {
        SymbolCode::from("123");
    }

    #[test]
    fn test_symbol_code_ord() {
        let symbol_code1 = SymbolCode::from("ABCDEFG");
        let symbol_code2 = SymbolCode::from("ABCDEFH");
        let symbol_code3 = SymbolCode::from("ABCDEF");
        assert!(symbol_code1 < symbol_code2);
        assert!(symbol_code1 > symbol_code3);
        assert!(symbol_code1 > symbol_code3);
        assert!(symbol_code3 < symbol_code2);
        assert!(symbol_code1 <= symbol_code1);
        assert!(symbol_code1 >= symbol_code1);
    }

    #[test]
    fn test_symbol_code_clone() {
        let symbol_code = SymbolCode::from("ABCDEFG");
        let cloned_symbol_code = symbol_code.clone();
        assert_eq!(symbol_code, cloned_symbol_code);
    }
}
