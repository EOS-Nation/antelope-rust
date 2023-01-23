use crate::{check, SymbolCode};

use std::cmp::{Ord, PartialEq, PartialOrd};
use std::convert::From;
use std::fmt::{Display, Formatter, Result};
use std::ops::Not;

/// The `Symbol` struct represents a symbol
///
/// Reference: <https://github.com/AntelopeIO/cdt/blob/main/libraries/eosiolib/core/eosio/symbol.hpp>
#[derive(Eq, Copy, Clone, Debug, PartialEq, PartialOrd, Ord, Default)]
pub struct Symbol {
    value: u64,
}

impl Symbol {
    /// Returns the raw value of the symbol
    ///
    /// The raw value is the underlying representation of the symbol
    ///
    /// The raw value is an unsigned 64-bit integer
    ///
    /// # Examples
    ///
    /// ```
    /// use antelope::Symbol;
    ///
    /// let sym = Symbol::from("4,FOO");
    /// assert_eq!(1330595332, sym.raw());
    /// ```
    #[inline]
    #[must_use]
    pub fn raw(&self) -> u64 {
        self.value
    }

    #[inline]
    #[must_use]
    pub fn code(&self) -> SymbolCode {
        SymbolCode::from(self.value >> 8)
    }

    /// Returns true if the symbol is valid
    ///
    /// A symbol code is valid if it is not empty and contains only uppercase letters and has a length of at most 7
    ///
    /// # Examples
    ///
    /// ```
    /// use antelope::Symbol;
    ///
    /// let sym = Symbol::from("4,FOO");
    /// assert_eq!(true, sym.is_valid());
    /// ```
    #[inline]
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.code().is_valid()
    }

    #[inline]
    #[must_use]
    pub fn precision(&self) -> u8 {
        self.value as u8
    }

    /// Returns a new Symbol
    ///
    /// # Examples
    ///
    /// ```
    /// use antelope::Symbol;
    ///
    /// let sym = Symbol::new();
    /// assert_eq!(0, sym.raw());
    /// ```
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self { value: 0 }
    }

    #[inline]
    #[must_use]
    pub fn from_precision(symcode: SymbolCode, precision: u8) -> Self {
        let value = (symcode.raw() << 8) | precision as u64;
        Symbol { value }
    }
}

impl Display for Symbol {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str(format!("{},{}", self.precision(), self.code()).as_str())
    }
}

impl From<&str> for Symbol {
    #[inline]
    #[must_use]
    fn from(str: &str) -> Self {
        let parts = str.split(',').collect::<Vec<&str>>();
        check(parts.len() == 2, "invalid symbol format");
        let precision = parts[0].parse::<u8>();
        check(precision.is_ok(), "invalid symbol precision");
        let symcode = SymbolCode::from(parts[1]);
        Symbol::from_precision(symcode, precision.unwrap())
    }
}

impl From<u64> for Symbol {
    #[inline]
    #[must_use]
    fn from(value: u64) -> Self {
        Symbol { value }
    }
}

impl From<Symbol> for u64 {
    #[inline]
    #[must_use]
    fn from(sym: Symbol) -> Self {
        sym.value
    }
}

impl AsRef<Symbol> for Symbol {
    #[inline]
    #[must_use]
    fn as_ref(&self) -> &Symbol {
        self
    }
}

impl Not for Symbol {
    type Output = bool;

    #[inline]
    #[must_use]
    fn not(self) -> bool {
        self.value == 0
    }
}

impl From<Symbol> for bool {
    #[inline]
    #[must_use]
    fn from(sym: Symbol) -> Self {
        sym.raw() != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn test_cdt_1() {
        //// constexpr uint64_t raw()const
        assert_eq!(0, Symbol::new().raw());
    }

    #[test]
    fn test_cdt_2() {
        //// constexpr explicit symbol(uint64_t)
        assert_eq!(0, Symbol::from(0).raw());
        assert_eq!(1, Symbol::from(1).raw());
        assert_eq!(u64::MAX, Symbol::from(u64::MAX).raw());

        assert_eq!(0 as u64, Symbol::from(0).into());
    }

    #[test]
    fn test_cdt_3() {
        let sc0 = SymbolCode::from("A");
        let sc1 = SymbolCode::from("Z");
        let sc2 = SymbolCode::from("AAAAAAA");
        let sc3 = SymbolCode::from("ZZZZZZZ");

        //// constexpr symbol(string_view, uint8_t)
        // Note:
        // Unless constructed with `initializer_list`, precision does not check for wrap-around
        assert_eq!(16640, Symbol::from_precision(sc0, 0).raw());
        assert_eq!(23040, Symbol::from_precision(sc1, 0).raw());
        assert_eq!(4702111234474983680, Symbol::from_precision(sc2, 0).raw());
        assert_eq!(6510615555426900480, Symbol::from_precision(sc3, 0).raw());

        //// constexpr symbol(symbol_code, uint8_t)
        assert_eq!(16640, Symbol::from_precision(sc0, 0).raw());
        assert_eq!(23040, Symbol::from_precision(sc1, 0).raw());
        assert_eq!(4702111234474983680, Symbol::from_precision(sc2, 0).raw());
        assert_eq!(6510615555426900480, Symbol::from_precision(sc3, 0).raw());
    }

    #[test]
    fn test_cdt_4() {
        // bool is_valid()const
        assert_eq!(true, Symbol::from(16640).is_valid()); // "A", precision: 0
        assert_eq!(true, Symbol::from(23040).is_valid()); // "Z", precision: 0
        assert_eq!(true, Symbol::from(4702111234474983680).is_valid()); // "AAAAAAA", precision: 0
        assert_eq!(true, Symbol::from(6510615555426900480).is_valid()); // "ZZZZZZZ", precision: 0

        assert_eq!(false, Symbol::from(16639).is_valid());
        assert_eq!(false, Symbol::from(6510615555426900736).is_valid());
    }

    #[test]
    fn test_cdt_5() {
        let sc0 = SymbolCode::from("A");
        let sc1 = SymbolCode::from("Z");
        let sc2 = SymbolCode::from("AAAAAAA");
        let sc3 = SymbolCode::from("ZZZZZZZ");

        // uint8_t precision()const
        assert_eq!(0, Symbol::from_precision(sc0, 0).precision());
        assert_eq!(0, Symbol::from_precision(sc1, 0).precision());
        assert_eq!(0, Symbol::from_precision(sc2, 0).precision());
        assert_eq!(0, Symbol::from_precision(sc3, 0).precision());

        assert_eq!(255, Symbol::from_precision(sc0, 255).precision());
        assert_eq!(255, Symbol::from_precision(sc1, 255).precision());
        assert_eq!(255, Symbol::from_precision(sc2, 255).precision());
        assert_eq!(255, Symbol::from_precision(sc3, 255).precision());
    }

    #[test]
    fn test_cdt_6() {
        let sc0 = SymbolCode::from("A");
        let sc1 = SymbolCode::from("Z");
        let sc2 = SymbolCode::from("AAAAAAA");
        let sc3 = SymbolCode::from("ZZZZZZZ");

        // symbol_code code()const
        assert_eq!(sc0, Symbol::from_precision(sc0, 0).code());
        assert_eq!(sc1, Symbol::from_precision(sc1, 0).code());
        assert_eq!(sc2, Symbol::from_precision(sc2, 0).code());
        assert_eq!(sc3, Symbol::from_precision(sc3, 0).code());
    }

    #[test]
    fn test_cdt_7() {
        // constexpr explicit operator bool()const
        assert_eq!(false, Symbol::from(0).into());
        assert_eq!(true, Symbol::from(1).into());
        assert_eq!(true, !Symbol::from(0));
        assert_eq!(false, !Symbol::from(1));

        assert_eq!(false, Symbol::from_precision(SymbolCode::from(""), 0).into());
        assert_eq!(true, Symbol::from_precision(SymbolCode::from("SYMBOLL"), 0).into());
        assert_eq!(true, !Symbol::from_precision(SymbolCode::from(""), 0));
        assert_eq!(false, !Symbol::from_precision(SymbolCode::from("SYMBOLL"), 0));
    }

    #[test]
    fn test_cdt_8() {
        let sc0 = SymbolCode::from("A");
        let sc1 = SymbolCode::from("Z");
        let sc2 = SymbolCode::from("AAAAAAA");
        let sc3 = SymbolCode::from("ZZZZZZZ");

        // friend constexpr bool operator==(const symbol&, const symbol&)
        assert_eq!(true, Symbol::from_precision(sc0, 0) == Symbol::from_precision(sc0, 0));
        assert_eq!(true, Symbol::from_precision(sc1, 0) == Symbol::from_precision(sc1, 0));
        assert_eq!(true, Symbol::from_precision(sc2, 0) == Symbol::from_precision(sc2, 0));
        assert_eq!(true, Symbol::from_precision(sc3, 0) == Symbol::from_precision(sc3, 0));

        // friend constexpr bool operator!=(const symbol&, const symbol&)
        assert_eq!(true, Symbol::from_precision(sc0, 0) != Symbol::new());
        assert_eq!(true, Symbol::from_precision(sc1, 0) != Symbol::new());
        assert_eq!(true, Symbol::from_precision(sc2, 0) != Symbol::new());
        assert_eq!(true, Symbol::from_precision(sc3, 0) != Symbol::new());

        // friend constexpr bool operator<(const symbol&, const symbol&)
        assert_eq!(true, Symbol::new() < Symbol::from_precision(sc0, 0));
        assert_eq!(true, Symbol::new() < Symbol::from_precision(sc1, 0));
        assert_eq!(true, Symbol::new() < Symbol::from_precision(sc2, 0));
        assert_eq!(true, Symbol::new() < Symbol::from_precision(sc3, 0));
    }

    #[test]
    fn test_from_str() {
        assert_eq!(Symbol::from("10,SYM"), Symbol::from_precision(SymbolCode::from("SYM"), 10));
        // CDT allows empty symbol code
        assert_eq!(Symbol::from("0,"), Symbol::from_precision(SymbolCode::from(""), 0));
        assert_eq!(Symbol::from("5,SYM").to_string(), "5,SYM");
        assert_eq!(Symbol::from("50,SYM").to_string(), "50,SYM"); // CDT doesn't check precision, could be > 18
        assert_eq!(Symbol::from("5,SYM").precision(), 5);
        assert_eq!(Symbol::from("5,SYM").code(), SymbolCode::from("SYM"));
    }

    #[test]
    #[allow(unused)]
    #[should_panic(expected = "only uppercase letters allowed in symbol_code string")]
    fn test_from_str_panic_1() {
        Symbol::from("10,a");
    }

    #[test]
    #[allow(unused)]
    #[should_panic(expected = "invalid symbol precision")]
    fn test_from_str_panic_2() {
        Symbol::from("1000,SYM");
    }

    #[test]
    #[allow(unused)]
    #[should_panic(expected = "invalid symbol format")]
    fn test_from_str_panic_3() {
        Symbol::from("10SYM");
    }

    #[test]
    #[allow(unused)]
    #[should_panic(expected = "only uppercase letters allowed in symbol_code string")]
    fn test_from_str_panic_4() {
        SymbolCode::from("SYM,10");
    }

    #[test]
    fn test_from_self() {
        let sym = Symbol::from("4,ABCDEFG");
        assert_eq!(Symbol::from(sym), sym);
    }

    proptest! {
        #[test]
        fn random_symbols(precision in 0..100, symcode in "[[A-Z]]{1,7}") {
            let sym_str = format!("{},{}", precision, symcode);
            let sym = Symbol::from(sym_str.as_str());
            prop_assert_eq!(sym.to_string(), sym_str);
        }
    }
}
