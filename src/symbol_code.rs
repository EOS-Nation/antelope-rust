#![allow(dead_code, unused)]
use std::cmp::Eq;
use std::cmp::PartialEq;
use std::convert::From;
use std::fmt;
use std::ops::Not;

#[derive(Debug)]
struct SymbolCode {
    value: u64,
}

impl SymbolCode {
    fn raw(&self) -> u64 {
        self.value
    }

    fn origin() -> SymbolCode {
        SymbolCode { value: 0 }
    }

    fn new(str: &str) -> SymbolCode {
        SymbolCode::from(str)
    }

    fn length(&self) -> u32 {
        let mut sym: u64 = self.value;
        let mut len: u32 = 0;

        while sym & 0xFF > 0 && len <= 7 {
            len += 1;
            sym >>= 8;
        }
        len
    }

    fn is_valid(&self) -> bool {
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
}

impl fmt::Display for SymbolCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mask = 0x00000000000000FF;
        if self.value == 0 {
            return fmt::Result::Ok(());
        }
        let mut begin = "".to_string();
        let mut v = self.value;
        let mut i = 0;

        while (i < 7) {
            if (v == 0) {
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
        !self.is_valid()
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
        if str.len() > 7 {
            panic!("string is too long to be a valid symbol_code")
        }
        for c in str.chars().rev() {
            if !('A'..='Z').contains(&c) {
                panic!("only uppercase letters allowed in symbol_code string")
            }
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

impl From<SymbolCode> for bool {
    fn from(_other: SymbolCode) -> bool {
        _other.is_valid()
    }
}

impl AsRef<SymbolCode> for SymbolCode {
    fn as_ref(&self) -> &SymbolCode {
        &self
    }
}

#[test]
fn test_as_ref() {
    assert_eq!(5459781, SymbolCode::from("EOS").as_ref().value);
}

#[test]
fn test_length() {
    assert_eq!(3, SymbolCode::from(5459781).length());
    assert_eq!(3, SymbolCode::from("EOS").length());
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
    assert_eq!(false, SymbolCode::from(5459781).not());
}

#[test]
fn test_partial_eq() {
    assert_eq!(true, SymbolCode::from(5459781) == SymbolCode::from(5459781));
    assert_eq!(true, SymbolCode::from(0) != SymbolCode::from(5459781));
    assert_eq!(false, SymbolCode::from(0) == SymbolCode::from(5459781));
}

#[test]
fn test_new() {
    assert_eq!("EOS", SymbolCode::new("EOS").to_string());
    assert_eq!(5459781, SymbolCode::new("EOS").raw());
}

#[test]
fn test_fmt() {
    assert_eq!("EOS", format!("{}", SymbolCode::new("EOS")));
}

#[test]
fn test_println() {
    println!("{}", SymbolCode::new("EOS"));
}

#[test]
fn test_origin() {
    assert_eq!(0, SymbolCode::origin().value);
}

#[test]
fn test_into_bool() {
    let true_boolean: bool = SymbolCode::from(5459781).into();
    let false_boolean: bool = SymbolCode::from(0).into();
    assert_eq!(true, true_boolean);
    assert_eq!(false, false_boolean);
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
    if SymbolCode::from(5459781).into() {
        panic!("SymbolCode::from( 5459781 ) should be true");
    }
}

#[test]
fn test_from() {
    assert_eq!(0, SymbolCode::from(0).value);
    assert_eq!(0, SymbolCode::from(0).raw());
    assert_eq!(0, SymbolCode::from("".to_string()).raw());
    assert_eq!(5459781, SymbolCode::from("EOS".to_string()).value);
    assert_eq!(5459781, SymbolCode::from(5459781).raw());
}

#[test]
fn test_is_valid() {
    assert_eq!(false, SymbolCode::from(0).is_valid());
    assert_eq!(true, SymbolCode::from("EOS".to_string()).is_valid());
}

#[test]
fn test_to_string() {
    assert_eq!("EOS", SymbolCode::from(5459781).to_string());
    assert_eq!("EOS", SymbolCode::from("EOS".to_string()).to_string());
    assert_eq!(
        "ABCDEFG",
        SymbolCode::from("ABCDEFG".to_string()).to_string()
    );
}


#[test]
fn test_to_str() {
    assert_eq!("EOS", SymbolCode::from("EOS").to_string());
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
