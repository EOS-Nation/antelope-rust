#![allow(dead_code, unused)]
use core::str;
use std::cmp::{Ord, Ordering, PartialEq, PartialOrd};
use std::convert::From;
use std::fmt;

use crate::check;

pub const NAME_CHARS: [u8; 32] = *b".12345abcdefghijklmnopqrstuvwxyz";

/// The maximum character length of an Antelope name.
pub const NAME_MAX_LEN: usize = 13;

/// The `Name` struct represents an Antelope name
///
/// Reference: <https://github.com/AntelopeIO/cdt/blob/main/libraries/eosiolib/core/eosio/name.hpp>
///
/// Wraps a %uint64_t to ensure it is only passed to methods that expect a %name.
/// Ensures value is only passed to methods that expect a %name and that no mathematical
/// operations occur.  Also enables specialization of print
///
/// # Examples
///
/// ```
/// use antelope::Name;
///
/// let account = Name::from("pinax");
/// assert_eq!(12368694922654515200, account.value);
/// assert_eq!("pinax", account.to_string());
/// ```
#[derive(Eq, Copy, Clone, Debug, PartialEq, PartialOrd, Ord, Default)]
pub struct Name {
    /// The raw value of the name
    ///
    /// The raw value is the underlying representation of the name
    ///
    /// The raw value is an unsigned 64-bit integer
    pub value: u64,
}

impl Name {
    /// Creates a new name
    #[must_use]
    pub fn new() -> Self {
        Self { value: 0 }
    }
}

#[must_use]
pub fn name_to_bytes(value: u64) -> [u8; NAME_MAX_LEN] {
    let mut chars = [b'.'; NAME_MAX_LEN];
    if value == 0 {
        return chars;
    }

    let mask = 0xF800_0000_0000_0000;
    let mut v = value;
    for (i, c) in chars.iter_mut().enumerate() {
        let index = (v & mask) >> (if i == 12 { 60 } else { 59 });
        let index = usize::try_from(index).unwrap_or_default();
        if let Some(v) = NAME_CHARS.get(index) {
            *c = *v;
        }
        v <<= 5;
    }
    chars
}

impl fmt::Display for Name {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let bytes = name_to_bytes(self.value);
        let value = str::from_utf8(&bytes)
            .map(|s| s.trim_end_matches('.'))
            .map_err(|_| fmt::Error)?;
        write!(f, "{}", value)
    }
}

/// Converts a character to a symbol.
fn char_to_value(c: u8) -> Option<u8> {
    if c == b'.' {
        Some(0)
    } else if (b'1'..=b'5').contains(&c) {
        Some(c - b'1' + 1)
    } else if (b'a'..=b'z').contains(&c) {
        Some(c - b'a' + 6)
    } else {
        None
    }
}

impl From<&str> for Name {
    fn from(str: &str) -> Self {
        let mut value = 0_u64;
        let mut len = 0_u64;
        let mut iter = str.bytes();

        // Loop through up to 12 characters
        for c in iter.by_ref() {
            let v = char_to_value(c);
            check(v.is_some(), "name contains invalid character");
            value <<= 5;
            value |= u64::from(v.unwrap());
            len += 1;

            if len == 12 {
                break;
            }
        }

        if len == 0 {
            return Self { value: 0 };
        }

        value <<= 4 + 5 * (12 - len);

        // Check if we have a 13th character
        if let Some(c) = iter.next() {
            let v = char_to_value(c);
            check(v.is_some(), "name contains invalid character");
            let v = v.unwrap();
            // The 13th character can only be 4 bits, it has to be between letters
            // 'a' to 'j'
            check(v <= 0x0F, "name contains invalid character");

            value |= u64::from(v);

            // Check if we have more than 13 characters
            check(iter.next().is_none(), "name is too long");
        }

        Self { value }
    }
}

impl From<u64> for Name {
    fn from(value: u64) -> Self {
        Name { value }
    }
}

impl AsRef<Name> for Name {
    fn as_ref(&self) -> &Name {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name_new() {
        let name = Name::new();
        assert_eq!(name.value, 0);
    }

    #[test]
    fn test_name_from_str() {
        let name = Name::from("pinax");
        assert_eq!(name.value, 12368694922654515200);
        assert_eq!(name.to_string(), "pinax");
    }

    #[test]
    fn test_name_from_str_13_chars() {
        let name = Name::from("aaaaaaaaaaaaj");
        assert_eq!(name.to_string(), "aaaaaaaaaaaaj");
    }

    #[test]
    #[should_panic(expected = "name contains invalid character")]
    fn test_name_from_str_invalid_char() {
        let _name = Name::from("pinax!");
    }

    #[test]
    #[should_panic(expected = "name is too long")]
    fn test_name_from_str_too_long() {
        let _name = Name::from("aaaaaaaaaaaaaa");
    }

    #[test]
    #[should_panic(expected = "name contains invalid character")]
    fn test_name_from_str_invalid_char_2() {
        let _name = Name::from("aaaaaaaaaaaak");
    }

    #[test]
    fn test_name_to_string() {
        assert_eq!(Name::from("pinax").to_string(), "pinax");
    }

    #[test]
    fn test_name_copy() {
        let name = Name::from("pinax");
        let copied_name = name;
        assert_eq!(name, copied_name);
    }

    #[test]
    fn test_name_clone() {
        let name = Name::from("pinax");
        let cloned_name = name.clone();
        assert_eq!(name, cloned_name);
    }

    #[test]
    fn test_name_ord() {
        let name1 = Name::from("pinax");
        let name2 = Name::from("antelope");
        let name3 = Name::from("eos");
        let name4 = Name::from("pinax");
        assert!(name1 > name2);
        assert!(name3 < name1);
        assert!(name2 < name3);
        assert!(name1 <= name4);
        assert!(name1 >= name4);
        assert!(name1 == name4);
        assert!(name1 != name2);
    }

    #[test]
    fn test_name_default() {
        let default_name = Name::default();
        assert_eq!(default_name.value, 0);
    }

    #[test]
    fn test_name_as_ref_str() {
        let name = Name::from("pinax");
        let name_ref = name.as_ref();
        assert_eq!(name_ref.to_string(), "pinax");
    }
}
