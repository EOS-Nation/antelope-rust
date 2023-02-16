use crate::check;
use std::cmp::{Ord, PartialEq, PartialOrd};
use std::convert::From;
use std::fmt::{Display, Formatter, Result};

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
    #[inline]
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
    #[inline]
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
    #[inline]
    #[must_use]
    pub fn is_valid(&self) -> bool {
        let mut sym: u64 = self.value;
        let mut i = 0;
        while i < 7 {
            let c = sym as u8 as char;
            if !('A'..='Z').contains(&c) {
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
    /// The new symbol code is empty
    ///
    /// The new symbol code has a raw value of 0
    ///
    /// # Examples
    ///
    /// ```
    /// use antelope::SymbolCode;
    ///
    /// let symcode = SymbolCode::new();
    /// assert_eq!(0, symcode.raw());
    /// ```
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self { value: 0 }
    }
}

impl Display for SymbolCode {
    #[inline]
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
    #[inline]
    #[must_use]
    fn from(str: &str) -> Self {
        let mut value: u64 = 0;
        check(str.len() <= 7, "string is too long to be a valid symbol_code");
        for c in str.chars().rev() {
            check(('A'..='Z').contains(&c), "only uppercase letters allowed in symbol_code string");
            value <<= 8;
            value |= c as u64;
        }
        SymbolCode { value }
    }
}

impl From<u64> for SymbolCode {
    #[inline]
    #[must_use]
    fn from(value: u64) -> Self {
        SymbolCode { value }
    }
}

impl From<SymbolCode> for u64 {
    #[inline]
    #[must_use]
    fn from(symcode: SymbolCode) -> Self {
        symcode.value
    }
}

impl AsRef<SymbolCode> for SymbolCode {
    #[inline]
    #[must_use]
    fn as_ref(&self) -> &SymbolCode {
        self
    }
}

impl From<SymbolCode> for bool {
    #[inline]
    #[must_use]
    fn from(symcode: SymbolCode) -> Self {
        symcode.raw() != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn test_cdt_1() {
        //// constexpr symbol_code()
        // constexpr uint64_t raw()const
        assert_eq!(0, SymbolCode::new().raw());
        assert_eq!(0 as u64, SymbolCode::new().into());
    }

    #[test]
    fn test_cdt_2() {
        //// constexpr explicit symbol_code(uint64_t raw)
        assert_eq!(0, SymbolCode::from(0).raw());
        assert_eq!(1, SymbolCode::from(1).raw());
        assert_eq!(u64::MAX, SymbolCode::from(u64::MAX).raw());
    }

    #[test]
    fn test_cdt_3() {
        //// constexpr explicit symbol_code(string_view str)
        assert_eq!(65, SymbolCode::from("A").raw());
        assert_eq!(90, SymbolCode::from("Z").raw());
        assert_eq!(18367622009667905, SymbolCode::from("AAAAAAA").raw());
        assert_eq!(25432092013386330, SymbolCode::from("ZZZZZZZ").raw());
    }

    #[test]
    fn test_cdt_4() {
        //// constexpr bool is_valid()const
        assert_eq!(true, SymbolCode::from(65).is_valid()); // "A"
        assert_eq!(true, SymbolCode::from(90).is_valid()); // "Z"
        assert_eq!(true, SymbolCode::from(18367622009667905).is_valid()); // "AAAAAAA"
        assert_eq!(true, SymbolCode::from(25432092013386330).is_valid()); // "ZZZZZZZ"

        assert_eq!(false, SymbolCode::from(64).is_valid());
        assert_eq!(false, SymbolCode::from(25432092013386331).is_valid());
    }

    #[test]
    fn test_cdt_5() {
        // constexpr uint32_t length()const
        assert_eq!(0, SymbolCode::from("").length());
        assert_eq!(1, SymbolCode::from("S").length());
        assert_eq!(2, SymbolCode::from("SY").length());
        assert_eq!(3, SymbolCode::from("SYM").length());
        assert_eq!(4, SymbolCode::from("SYMB").length());
        assert_eq!(5, SymbolCode::from("SYMBO").length());
        assert_eq!(6, SymbolCode::from("SYMBOL").length());
        assert_eq!(7, SymbolCode::from("SYMBOLL").length());
    }

    #[test]
    fn test_cdt_6() {
        // constexpr explicit operator bool()const
        assert_eq!(false, SymbolCode::from(0).into());
        assert_eq!(true, SymbolCode::from(1).into());

        assert_eq!(false, SymbolCode::from("").into());
        assert_eq!(true, SymbolCode::from("SYMBOL").into());
    }

    #[test]
    fn test_cdt_7() {
        // string to_string()const
        assert_eq!("A", SymbolCode::from("A").to_string());
        assert_eq!("Z", SymbolCode::from("Z").to_string());
        assert_eq!("AAAAAAA", SymbolCode::from("AAAAAAA").to_string());
        assert_eq!("ZZZZZZZ", SymbolCode::from("ZZZZZZZ").to_string());
    }

    #[test]
    fn test_cdt_8() {
        // friend bool operator==(const symbol_code&, const symbol_code&)
        assert_eq!(true, SymbolCode::from("A") == SymbolCode::from("A"));
        assert_eq!(true, SymbolCode::from("Z") == SymbolCode::from("Z"));
        assert_eq!(true, SymbolCode::from("AAAAAAA") == SymbolCode::from("AAAAAAA"));
        assert_eq!(true, SymbolCode::from("ZZZZZZZ") == SymbolCode::from("ZZZZZZZ"));
    }

    #[test]
    fn test_cdt_9() {
        // friend bool operator!=(const symbol_code&, const symbol_code&)
        assert_eq!(true, SymbolCode::from("A") != SymbolCode::new());
        assert_eq!(true, SymbolCode::from("Z") != SymbolCode::new());
        assert_eq!(true, SymbolCode::from("AAAAAAA") != SymbolCode::new());
        assert_eq!(true, SymbolCode::from("ZZZZZZZ") != SymbolCode::new());
    }

    #[test]
    fn test_cdt_10() {
        // friend bool operator<(const symbol_code&, const symbol_code&)
        assert_eq!(true, SymbolCode::new() < SymbolCode::from("A"));
        assert_eq!(true, SymbolCode::new() < SymbolCode::from("Z"));
        assert_eq!(true, SymbolCode::new() < SymbolCode::from("AAAAAAA"));
        assert_eq!(true, SymbolCode::new() < SymbolCode::from("ZZZZZZZ"));
    }

    #[test]
    #[allow(unused)]
    #[should_panic(expected = "string is too long to be a valid symbol_code")]
    fn test_cdt_panic_1() {
        SymbolCode::from("ABCDEFGH");
    }

    #[test]
    #[allow(unused)]
    #[should_panic(expected = "only uppercase letters allowed in symbol_code string")]
    fn test_cdt_panic_2a() {
        SymbolCode::from("a");
    }

    #[test]
    #[allow(unused)]
    #[should_panic(expected = "only uppercase letters allowed in symbol_code string")]
    fn test_cdt_panic_2b() {
        SymbolCode::from("@");
    }

    #[test]
    fn test_as_ref() {
        assert_eq!(5197638, SymbolCode::from("FOO").as_ref().value);
        assert_eq!(5390658, SymbolCode::from("BAR").as_ref().value);
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
        assert_eq!("", format!("{}", SymbolCode::new()));
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
    #[allow(unused)]
    #[should_panic(expected = "string is too long to be a valid symbol_code")]
    fn test_from_string_long_panic_1() {
        SymbolCode::from("ABCDEFGH");
    }

    #[test]
    #[allow(unused)]
    #[should_panic(expected = "only uppercase letters allowed in symbol_code string")]
    fn test_from_string_letters_panic_1() {
        SymbolCode::from("abc");
    }

    #[test]
    #[allow(unused)]
    #[should_panic(expected = "only uppercase letters allowed in symbol_code string")]
    fn test_from_string_letters_panic_2() {
        SymbolCode::from("123");
    }

    #[test]
    fn test_ord() {
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
    fn test_clone() {
        let symbol_code = SymbolCode::from("ABCDEFG");
        let cloned_symbol_code = symbol_code.clone();
        assert_eq!(symbol_code, cloned_symbol_code);
        assert_eq!(5197638, SymbolCode::from("FOO").clone().value);
    }

    #[test]
    fn test_from_self() {
        let symcode = SymbolCode::from("ABCDEFG");
        assert_eq!(SymbolCode::from(symcode), symcode);
    }

    #[test]
    fn test_to_bool() {
        assert_eq!(true, SymbolCode::from("ABCDEFG").into());
        assert_eq!(false, SymbolCode::default().into());
        assert_eq!(false, SymbolCode::from("").into());
    }

    proptest! {
        #[test]
        fn random_sym_codes(input in "[[A-Z]]{1,7}") {
            let symcode = SymbolCode::from(input.as_str());
            prop_assert_eq!(symcode.to_string(), input);
        }
    }
}
