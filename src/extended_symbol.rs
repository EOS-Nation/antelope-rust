use crate::{Name, ParseError, Symbol};
use std::cmp::{Ord, PartialEq, PartialOrd};
use std::convert::From;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

/// The `ExtendedSymbol` struct represents an extended symbol
///
/// Reference: <https://github.com/AntelopeIO/cdt/blob/main/libraries/eosiolib/core/eosio/symbol.hpp>
///
/// # Examples
///
/// ```
/// use antelope::ExtendedSymbol;
///
/// let ext_sym = ExtendedSymbol::from("4,FOO@token");
/// assert_eq!("4,FOO", ext_sym.get_symbol().to_string());
/// assert_eq!("token", ext_sym.get_contract().to_string());
/// ```
#[derive(Eq, Copy, Clone, Debug, PartialEq, PartialOrd, Ord, Default)]
pub struct ExtendedSymbol {
    contract: Name,
    sym: Symbol,
}

impl ExtendedSymbol {
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {
            contract: Name::new(),
            sym: Symbol::new(),
        }
    }

    #[inline]
    #[must_use]
    pub fn get_contract(&self) -> Name {
        self.contract
    }

    #[inline]
    #[must_use]
    pub fn get_symbol(&self) -> Symbol {
        self.sym
    }

    #[inline]
    #[must_use]
    pub fn from_extended(sym: Symbol, contract: Name) -> Self {
        ExtendedSymbol { sym, contract }
    }

    #[inline]
    #[must_use]
    pub fn from_symbol(sym: Symbol) -> Self {
        ExtendedSymbol {
            sym,
            contract: Name::new(),
        }
    }
}

impl Display for ExtendedSymbol {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{}@{}", self.sym, self.contract).as_str())
    }
}

impl FromStr for ExtendedSymbol {
    type Err = ParseError;

    /**
     * Parse ExtendedSymbol from string formatted as "4,SYM@contract"
     *
     */
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('@').collect();
        if parts.len() != 2 {
            return Err(ParseError::BadFormat);
        }

        let symbol = match Symbol::from_str(parts[0]) {
            Ok(asset) => asset,
            Err(_) => return Err(ParseError::BadSymbol(parts[0].to_string())),
        };
        let contract = match Name::from_str(parts[1]) {
            Ok(name) => name,
            Err(_) => return Err(ParseError::BadName(parts[1].to_string())),
        };

        Ok(ExtendedSymbol::from_extended(symbol, contract))
    }
}

impl From<&str> for ExtendedSymbol {
    fn from(s: &str) -> Self {
        Self::from_str(s).unwrap_or_else(|e| panic!("failed to parse extended symbol: {}", e))
    }
}

impl AsRef<ExtendedSymbol> for ExtendedSymbol {
    #[inline]
    #[must_use]
    fn as_ref(&self) -> &ExtendedSymbol {
        self
    }
}

impl From<ExtendedSymbol> for bool {
    #[inline]
    #[must_use]
    fn from(ext_sym: ExtendedSymbol) -> Self {
        ext_sym.contract.raw() != 0 && ext_sym.sym.raw() != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SymbolCode;

    #[test]
    fn test_cdt_1() {
        //// constexpr ExtendedSymbol::from_extended()
        // constexpr name get_symbol()
        // constexpr name get_contract()
        assert_eq!(ExtendedSymbol::new().get_symbol().raw(), 0);
        assert_eq!(ExtendedSymbol::new().get_contract().value, 0);
    }

    #[test]
    fn test_cdt_2() {
        let n0 = Name::from("1");
        let n1 = Name::from("5");
        let n2 = Name::from("a");
        let n3 = Name::from("z");
        let n4 = Name::from("111111111111j");
        let n5 = Name::from("555555555555j");
        let n6 = Name::from("aaaaaaaaaaaaj");
        let n7 = Name::from("zzzzzzzzzzzzj");

        let s0 = Symbol::from_precision(SymbolCode::from("A"), 0);
        let s1 = Symbol::from_precision(SymbolCode::from("Z"), 0);
        let s2 = Symbol::from_precision(SymbolCode::from("AAAAAAA"), 255);
        let s3 = Symbol::from_precision(SymbolCode::from("ZZZZZZZ"), 255);

        //// constexpr ExtendedSymbol::from_extended(symbol, name)
        assert_eq!(ExtendedSymbol::from_extended(s0, n0).get_symbol().raw(), 16640);
        assert_eq!(ExtendedSymbol::from_extended(s0, n1).get_symbol().code().raw(), 65);
        assert_eq!(ExtendedSymbol::from_extended(s1, n2).get_symbol().raw(), 23040);
        assert_eq!(ExtendedSymbol::from_extended(s1, n3).get_symbol().code().raw(), 90);
        assert_eq!(ExtendedSymbol::from_extended(s0, n0).get_contract().value, 576460752303423488);
        assert_eq!(ExtendedSymbol::from_extended(s0, n1).get_contract().value, 2882303761517117440);
        assert_eq!(ExtendedSymbol::from_extended(s1, n2).get_contract().value, 3458764513820540928);
        assert_eq!(ExtendedSymbol::from_extended(s1, n3).get_contract().value, 17870283321406128128);
        assert_eq!(ExtendedSymbol::from_extended(s2, n4).get_symbol().raw(), 4702111234474983935);
        assert_eq!(ExtendedSymbol::from_extended(s2, n5).get_symbol().code().raw(), 18367622009667905);
        assert_eq!(ExtendedSymbol::from_extended(s3, n6).get_symbol().raw(), 6510615555426900735);
        assert_eq!(ExtendedSymbol::from_extended(s3, n7).get_symbol().code().raw(), 25432092013386330);
        assert_eq!(ExtendedSymbol::from_extended(s2, n4).get_contract().value, 595056260442243615);
        assert_eq!(ExtendedSymbol::from_extended(s2, n5).get_contract().value, 2975281302211218015);
        assert_eq!(ExtendedSymbol::from_extended(s3, n6).get_contract().value, 3570337562653461615);
        assert_eq!(ExtendedSymbol::from_extended(s3, n7).get_contract().value, u64::MAX);
    }

    #[test]
    fn test_cdt_3() {
        let n0 = Name::from("1");
        let n3 = Name::from("z");
        let n4 = Name::from("111111111111j");
        let n7 = Name::from("zzzzzzzzzzzzj");

        let s0 = Symbol::from_precision(SymbolCode::from("A"), 0);
        let s1 = Symbol::from_precision(SymbolCode::from("Z"), 0);
        let s2 = Symbol::from_precision(SymbolCode::from("AAAAAAA"), 255);
        let s3 = Symbol::from_precision(SymbolCode::from("ZZZZZZZ"), 255);

        // -------------------------------------------------------------------------------
        // friend constexpr bool operator==(const ExtendedSymbol::from_extended(, const ExtendedSymbol::from_extended()
        assert_eq!(ExtendedSymbol::from_extended(s0, n0) == ExtendedSymbol::from_extended(s0, n0), true);
        assert_eq!(ExtendedSymbol::from_extended(s1, n3) == ExtendedSymbol::from_extended(s1, n3), true);
        assert_eq!(ExtendedSymbol::from_extended(s2, n4) == ExtendedSymbol::from_extended(s2, n4), true);
        assert_eq!(ExtendedSymbol::from_extended(s3, n7) == ExtendedSymbol::from_extended(s3, n7), true);
    }

    #[test]
    fn test_cdt_4() {
        let s0 = Symbol::from_precision(SymbolCode::from("A"), 0);
        let s1 = Symbol::from_precision(SymbolCode::from("Z"), 0);
        let s2 = Symbol::from_precision(SymbolCode::from("AAAAAAA"), 255);
        let s3 = Symbol::from_precision(SymbolCode::from("ZZZZZZZ"), 255);

        // friend constexpr bool operator!=(const ExtendedSymbol::from_extended(, const ExtendedSymbol::from_extended();
        assert_eq!(ExtendedSymbol::from_symbol(Symbol::new()) != ExtendedSymbol::from_symbol(s0), true);
        assert_eq!(ExtendedSymbol::from_symbol(s0) != ExtendedSymbol::from_symbol(s1), true);
        assert_eq!(ExtendedSymbol::from_symbol(s1) != ExtendedSymbol::from_symbol(s2), true);
        assert_eq!(ExtendedSymbol::from_symbol(s2) != ExtendedSymbol::from_symbol(s3), true);
    }

    #[test]
    fn test_cdt_5() {
        let s0 = Symbol::from_precision(SymbolCode::from("A"), 0);
        let s1 = Symbol::from_precision(SymbolCode::from("Z"), 0);
        let s2 = Symbol::from_precision(SymbolCode::from("AAAAAAA"), 255);
        let s3 = Symbol::from_precision(SymbolCode::from("ZZZZZZZ"), 255);

        // friend constexpr bool operator<(const ExtendedSymbol::from_extended(, const ExtendedSymbol::from_extended();
        assert_eq!(ExtendedSymbol::new() < ExtendedSymbol::from_symbol(s0), true);
        assert_eq!(ExtendedSymbol::new() < ExtendedSymbol::from_symbol(s1), true);
        assert_eq!(ExtendedSymbol::new() < ExtendedSymbol::from_symbol(s2), true);
        assert_eq!(ExtendedSymbol::new() < ExtendedSymbol::from_symbol(s3), true);
    }
}
