#![allow(dead_code, unused)]
use core::str;
use std::cmp::{Ord, Ordering, PartialEq, PartialOrd};
use std::convert::From;
use std::fmt;
use std::ops::Not;

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

    #[must_use]
    pub fn raw(&self) -> u64 {
        self.value
    }

    #[must_use]
    pub fn length(&self) -> u8 {
        let mask: u64 = 0xF800000000000000;

        if (self.value == 0) {
            return 0;
        }
        let mut l: u8 = 0;
        let mut i: u8 = 0;

        let mut v = self.value;
        while i < 13 {
            if ((v & mask) > 0) {
                l = i;
            }
            v <<= 5;
            i += 1;
        }
        l + 1
    }

    /// Converts a character to a symbol.
    #[must_use]
    fn char_to_value(c: char) -> Option<u8> {
        if c == '.' {
            Some(0)
        } else if ('1'..='5').contains(&c) {
            Some(c as u8 - b'1' + 1)
        } else if ('a'..='z').contains(&c) {
            Some(c as u8 - b'a' + 6)
        } else {
            None
        }
    }

    #[must_use]
    fn prefix(&self) -> Name {
        let mut result: u64 = self.value;
        let mut not_dot_character_seen: bool = false;
        let mut mask: u64 = 0xF;

        // Get characters one-by-one in name in order from right to left
        let mut offset: i32 = 0;
        while (offset <= 59) {
            let c = (self.value >> offset) & mask;

            if (c == 0) {
                // if this character is a dot
                if (not_dot_character_seen) {
                    // we found the rightmost dot character
                    result = (self.value >> offset) << offset;
                    break;
                }
            } else {
                not_dot_character_seen = true;
            }

            if (offset == 0) {
                offset += 4;
                mask = 0x1F;
            } else {
                offset += 5;
            }
        }

        Name::from(result)
    }

    fn suffix(&self) -> Name {
        let mut remaining_bits_after_last_actual_dot: u32 = 0;
        let mut tmp: u32 = 0;

        let mut remaining_bits = 59;
        while (remaining_bits >= 4) {
            // Note: remaining_bits must remain signed integer
            // Get characters one-by-one in name in order from left to right (not including the 13th character)
            let c = (self.value >> remaining_bits) & 0x1F;
            if (c == 0) {
                // if this character is a dot
                tmp = remaining_bits as u32;
            } else {
                // if this character is not a dot
                remaining_bits_after_last_actual_dot = tmp;
            }
            remaining_bits -= 5;
        }

        let mut thirteenth_character: u64 = self.value & 0x0F;
        if thirteenth_character != 0 {
            // if 13th character is not a dot
            remaining_bits_after_last_actual_dot = tmp;
        }

        if (remaining_bits_after_last_actual_dot == 0) {
            // there is no actual dot in the %name other than potentially leading dots
            return Name::from(self.value);
        }

        // At this point remaining_bits_after_last_actual_dot has to be within the range of 4 to 59 (and restricted to increments of 5).

        // Mask for remaining bits corresponding to characters after last actual dot, except for 4 least significant bits (corresponds to 13th character).
        let mask: u64 = (1 << remaining_bits_after_last_actual_dot) - 16;
        let shift: u32 = 64 - remaining_bits_after_last_actual_dot;

        Name::from(((self.value & mask) << shift) + (thirteenth_character << (shift - 1)))
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

impl From<&str> for Name {
    fn from(str: &str) -> Self {
        let mut value = 0_u64;
        let mut len = 0_u64;
        let mut iter = str.bytes();

        // Loop through up to 12 characters
        for c in iter.by_ref() {
            let v = Name::char_to_value(c as char);
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
            let v = Name::char_to_value(c as char);
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
    #[inline]
    #[must_use]
    fn from(value: u64) -> Self {
        Name { value }
    }
}

impl From<Name> for u64 {
    #[inline]
    #[must_use]
    fn from(name: Name) -> Self {
        name.value
    }
}

impl AsRef<Name> for Name {
    fn as_ref(&self) -> &Name {
        self
    }
}

impl Not for Name {
    type Output = bool;

    #[inline]
    #[must_use]
    fn not(self) -> bool {
        self.value == 0
    }
}

impl From<Name> for bool {
    #[inline]
    #[must_use]
    fn from(name: Name) -> Self {
        name.raw() != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn test_cdt_1() {
        //// constexpr name()
        assert_eq!(Name::new().value, 0);

        //// constexpr explicit name(uint64_t)
        assert_eq!(Name::from(0).value, 0);
        assert_eq!(Name::from(1).value, 1);
        assert_eq!(Name::from(u64::MAX).value, u64::MAX);

        //// constexpr explicit name(name::raw)
        assert_eq!(Name::from(0).raw(), 0);
        assert_eq!(Name::from(1).raw(), 1);
        assert_eq!(Name::from(u64::MAX).raw(), u64::MAX);

        // test that constexpr constructor is evaluated at compile time
        assert_eq!(Name::from(0).value == 0, true);
        assert_eq!(Name::from(Name::from(1)).value == 1, true);
    }

    #[test]
    fn test_cdt_2() {
        //// constexpr explicit name(string_view)
        // Note:
        // These are the exact `uint64_t` value representations of the given string
        assert_eq!(Name::from("1").value, 576460752303423488);
        assert_eq!(Name::from("5").value, 2882303761517117440);
        assert_eq!(Name::from("a").value, 3458764513820540928);
        assert_eq!(Name::from("z").value, 17870283321406128128);

        assert_eq!(Name::from("abc").value, 3589368903014285312);
        assert_eq!(Name::from("123").value, 614178399182651392);

        assert_eq!(Name::from(".abc").value, 112167778219196416);
        assert_eq!(Name::from(".........abc").value, 102016);
        assert_eq!(Name::from("123.").value, 614178399182651392);
        assert_eq!(Name::from("123.........").value, 614178399182651392);
        assert_eq!(Name::from(".a.b.c.1.2.3.").value, 108209673814966320);
        assert_eq!(Name::from("abc.123").value, 3589369488740450304);
        assert_eq!(Name::from("123.abc").value, 614181822271586304);

        assert_eq!(Name::from("12345abcdefgj").value, 614251623682315983);
        assert_eq!(Name::from("hijklmnopqrsj").value, 7754926748989239183);
        assert_eq!(Name::from("tuvwxyz.1234j").value, 14895601873741973071);

        assert_eq!(Name::from("111111111111j").value, 595056260442243615);
        assert_eq!(Name::from("555555555555j").value, 2975281302211218015);
        assert_eq!(Name::from("aaaaaaaaaaaaj").value, 3570337562653461615);
        assert_eq!(Name::from("zzzzzzzzzzzzj").value, u64::MAX);

        // test that constexpr constructor is evaluated at compile time
        assert_eq!(Name::from("1").value == 576460752303423488, true);
    }

    #[test]
    fn test_cdt_3() {
        // constexpr uint8_t length()cosnt
        assert_eq!(Name::from("").length(), 0);
        assert_eq!(Name::from("e").length(), 1);
        assert_eq!(Name::from("eo").length(), 2);
        assert_eq!(Name::from("eos").length(), 3);
        assert_eq!(Name::from("eosi").length(), 4);
        assert_eq!(Name::from("eosio").length(), 5);
        assert_eq!(Name::from("eosioa").length(), 6);
        assert_eq!(Name::from("eosioac").length(), 7);
        assert_eq!(Name::from("eosioacc").length(), 8);
        assert_eq!(Name::from("eosioacco").length(), 9);
        assert_eq!(Name::from("eosioaccou").length(), 10);
        assert_eq!(Name::from("eosioaccoun").length(), 11);
        assert_eq!(Name::from("eosioaccount").length(), 12);
        assert_eq!(Name::from("eosioaccountj").length(), 13);
    }

    #[test]
    fn test_cdt_4() {
        // constexpr name suffix()const
        assert_eq!(
            Name::from(".eosioaccounj").suffix(),
            Name::from("eosioaccounj")
        );
        assert_eq!(
            Name::from("e.osioaccounj").suffix(),
            Name::from("osioaccounj")
        );
        assert_eq!(
            Name::from("eo.sioaccounj").suffix(),
            Name::from("sioaccounj")
        );
        assert_eq!(
            Name::from("eos.ioaccounj").suffix(),
            Name::from("ioaccounj")
        );
        assert_eq!(Name::from("eosi.oaccounj").suffix(), Name::from("oaccounj"));
        assert_eq!(Name::from("eosio.accounj").suffix(), Name::from("accounj"));
        assert_eq!(Name::from("eosioa.ccounj").suffix(), Name::from("ccounj"));
        assert_eq!(Name::from("eosioac.counj").suffix(), Name::from("counj"));
        assert_eq!(Name::from("eosioacc.ounj").suffix(), Name::from("ounj"));
        assert_eq!(Name::from("eosioacco.unj").suffix(), Name::from("unj"));
        assert_eq!(Name::from("eosioaccou.nj").suffix(), Name::from("nj"));
        assert_eq!(Name::from("eosioaccoun.j").suffix(), Name::from("j"));

        assert_eq!(Name::from("e.o.s.i.o.a.c").suffix(), Name::from("c"));
        assert_eq!(Name::from("eos.ioa.cco").suffix(), Name::from("cco"));
    }

    #[test]
    fn test_cdt_5() {
        // constexpr name prefix()const
        assert_eq!(Name::from(".eosioaccounj").prefix(), Name::new());
        assert_eq!(Name::from("e.osioaccounj").prefix(), Name::from("e"));
        assert_eq!(Name::from("eo.sioaccounj").prefix(), Name::from("eo"));
        assert_eq!(Name::from("eos.ioaccounj").prefix(), Name::from("eos"));
        assert_eq!(Name::from("eosi.oaccounj").prefix(), Name::from("eosi"));
        assert_eq!(Name::from("eosio.accounj").prefix(), Name::from("eosio"));
        assert_eq!(Name::from("eosioa.ccounj").prefix(), Name::from("eosioa"));
        assert_eq!(Name::from("eosioac.counj").prefix(), Name::from("eosioac"));
        assert_eq!(Name::from("eosioacc.ounj").prefix(), Name::from("eosioacc"));
        assert_eq!(
            Name::from("eosioacco.unj").prefix(),
            Name::from("eosioacco")
        );
        assert_eq!(
            Name::from("eosioaccou.nj").prefix(),
            Name::from("eosioaccou")
        );
        assert_eq!(
            Name::from("eosioaccoun.j").prefix(),
            Name::from("eosioaccoun")
        );
        assert_eq!(
            Name::from("eosioaccounj.").prefix(),
            Name::from("eosioaccounj")
        );
        assert_eq!(
            Name::from("eosioaccountj").prefix(),
            Name::from("eosioaccountj")
        );

        assert_eq!(
            Name::from("e.o.s.i.o.a.c").prefix(),
            Name::from("e.o.s.i.o.a")
        );
        assert_eq!(Name::from("eos.ioa.cco").prefix(), Name::from("eos.ioa"));

        assert_eq!(Name::from("a.my.account").prefix(), Name::from("a.my"));
        assert_eq!(
            Name::from("a.my.account").prefix().prefix(),
            Name::from("a")
        );

        assert_eq!(
            Name::from("e.osioaccounj").prefix() == Name::from("e"),
            true
        );
    }

    #[test]
    fn test_cdt_6() {
        // constexpr explicit operator bool()const
        // Note that I must be explicit about calling the operator because it is defined as `explicit`
        assert_eq!(false, Name::from(0).into());
        assert_eq!(true, Name::from(1).into());
        assert_eq!(true, !Name::from(0));
        assert_eq!(false, !Name::from(1));

        assert_eq!(false, Name::from("").into());
        assert_eq!(true, Name::from("1").into());
        assert_eq!(true, !Name::from(""));
        assert_eq!(false, !Name::from("1"));

        assert_eq!(true, false == Name::from(0).into());
    }

    #[test]
    fn test_cdt_7() {
        // EOSIO_TEST_BEGIN(name_type_test_op_bool)
        // // ---------------------------------------

        // EOSIO_TEST_END

        // EOSIO_TEST_BEGIN(name_type_test_memcmp)
        // // ----------------------------------------
        // // char* write_as_string(char*, char*)const
        // constexpr uint8_t buffer_size{32);
        // char buffer[buffer_size]{);

        // string str::from("1");
        // Name::from(str).write_as_string( buffer, buffer + sizeof(buffer) );
        // assert_eq!(memcmp(str.c_str(), buffer, strlen(str.c_str())), 0 );
        // Name::from(str = "5").write_as_string( buffer, buffer + sizeof(buffer) );
        // assert_eq!(memcmp(str.c_str(), buffer, strlen(str.c_str())), 0 );
        // Name::from(str = "a").write_as_string( buffer, buffer + sizeof(buffer) );
        // assert_eq!(memcmp(str.c_str(), buffer, strlen(str.c_str())), 0 );
        // Name::from(str = "z").write_as_string( buffer, buffer + sizeof(buffer) );
        // assert_eq!(memcmp(str.c_str(), buffer, strlen(str.c_str())), 0 );

        // Name::from(str = "abc").write_as_string( buffer, buffer + sizeof(buffer) );
        // assert_eq!(memcmp(str.c_str(), buffer, strlen(str.c_str())), 0 );
        // Name::from(str = "123").write_as_string( buffer, buffer + sizeof(buffer) );
        // assert_eq!(memcmp(str.c_str(), buffer, strlen(str.c_str())), 0 );

        // // Note:
        // // Any '.' characters at the end of a name are ignored
        // Name::from(str = ".abc").write_as_string( buffer, buffer + sizeof(buffer) );
        // assert_eq!(memcmp(str.c_str(), buffer, strlen(str.c_str())), 0 );
        // Name::from(str = ".........abc").write_as_string( buffer, buffer + sizeof(buffer) );
        // assert_eq!(memcmp(str.c_str(), buffer, strlen(str.c_str())), 0 );
        // Name::from(str = "123.").write_as_string( buffer, buffer + sizeof(buffer) );
        // assert_eq!(memcmp("123", buffer, 3), 0 );
        // Name::from(str = "123.........").write_as_string( buffer, buffer + sizeof(buffer) );
        // assert_eq!(memcmp("123", buffer, 3), 0 );
        // Name::from(str = ".a.b.c.1.2.3.").write_as_string( buffer, buffer + sizeof(buffer) );
        // assert_eq!(memcmp(".a.b.c.1.2.3", buffer, 12), 0 );

        // Name::from(str = "abc.123").write_as_string( buffer, buffer + sizeof(buffer) );
        // assert_eq!(memcmp(str.c_str(), buffer, strlen(str.c_str())), 0 );
        // Name::from(str = "123.abc").write_as_string( buffer, buffer + sizeof(buffer) );
        // assert_eq!(memcmp(str.c_str(), buffer, strlen(str.c_str())), 0 );

        // Name::from(str = "12345abcdefgj").write_as_string( buffer, buffer + sizeof(buffer) );
        // assert_eq!(memcmp(str.c_str(), buffer, strlen(str.c_str())), 0 );
        // Name::from(str = "hijklmnopqrsj").write_as_string( buffer, buffer + sizeof(buffer) );
        // assert_eq!(memcmp(str.c_str(), buffer, strlen(str.c_str())), 0 );
        // Name::from(str = "tuvwxyz.1234j").write_as_string( buffer, buffer + sizeof(buffer) );
        // assert_eq!(memcmp(str.c_str(), buffer, strlen(str.c_str())), 0 );

        // Name::from(str = "111111111111j").write_as_string( buffer, buffer + sizeof(buffer) );
        // assert_eq!(memcmp(str.c_str(), buffer, strlen(str.c_str())), 0 );
        // Name::from(str = "555555555555j").write_as_string( buffer, buffer + sizeof(buffer) );
        // assert_eq!(memcmp(str.c_str(), buffer, strlen(str.c_str())), 0 );
        // Name::from(str = "aaaaaaaaaaaaj").write_as_string( buffer, buffer + sizeof(buffer) );
        // assert_eq!(memcmp(str.c_str(), buffer, strlen(str.c_str())), 0 );
        // Name::from(str = "zzzzzzzzzzzzj").write_as_string( buffer, buffer + sizeof(buffer) );
        // assert_eq!(memcmp(str.c_str(), buffer, strlen(str.c_str())), 0 );
        // EOSIO_TEST_END
    }

    #[test]
    fn test_cdt_8() {
        // string to_string()const
        assert_eq!(Name::from("1").to_string(), "1");
        assert_eq!(Name::from("5").to_string(), "5");
        assert_eq!(Name::from("a").to_string(), "a");
        assert_eq!(Name::from("z").to_string(), "z");

        assert_eq!(Name::from("abc").to_string(), "abc");
        assert_eq!(Name::from("123").to_string(), "123");

        assert_eq!(Name::from(".abc").to_string(), ".abc");
        assert_eq!(Name::from(".........abc").to_string(), ".........abc");
        assert_eq!(Name::from("123.").to_string(), "123");
        assert_eq!(Name::from("123.........").to_string(), "123");
        assert_eq!(Name::from(".a.b.c.1.2.3.").to_string(), ".a.b.c.1.2.3");

        assert_eq!(Name::from("abc.123").to_string(), "abc.123");
        assert_eq!(Name::from("123.abc").to_string(), "123.abc");

        assert_eq!(Name::from("12345abcdefgj").to_string(), "12345abcdefgj");
        assert_eq!(Name::from("hijklmnopqrsj").to_string(), "hijklmnopqrsj");
        assert_eq!(Name::from("tuvwxyz.1234j").to_string(), "tuvwxyz.1234j");

        assert_eq!(Name::from("111111111111j").to_string(), "111111111111j");
        assert_eq!(Name::from("555555555555j").to_string(), "555555555555j");
        assert_eq!(Name::from("aaaaaaaaaaaaj").to_string(), "aaaaaaaaaaaaj");
        assert_eq!(Name::from("zzzzzzzzzzzzj").to_string(), "zzzzzzzzzzzzj");

        assert_eq!(Name::from("1") == Name::from("1"), true);
    }

    #[test]
    fn test_cdt_9() {
        // friend constexpr bool operator==(const name&, const name&)
        assert_eq!(Name::from("1") == Name::from("1"), true);
        assert_eq!(Name::from("5") == Name::from("5"), true);
        assert_eq!(Name::from("a") == Name::from("a"), true);
        assert_eq!(Name::from("z") == Name::from("z"), true);

        assert_eq!(Name::from("abc") == Name::from("abc"), true);
        assert_eq!(Name::from("123") == Name::from("123"), true);

        assert_eq!(Name::from(".abc") == Name::from(".abc"), true);
        assert_eq!(
            Name::from(".........abc") == Name::from(".........abc"),
            true
        );
        assert_eq!(Name::from("123.") == Name::from("123"), true);
        assert_eq!(Name::from("123.........") == Name::from("123"), true);
        assert_eq!(
            Name::from(".a.b.c.1.2.3.") == Name::from(".a.b.c.1.2.3"),
            true
        );

        assert_eq!(Name::from("abc.123") == Name::from("abc.123"), true);
        assert_eq!(Name::from("123.abc") == Name::from("123.abc"), true);

        assert_eq!(
            Name::from("12345abcdefgj") == Name::from("12345abcdefgj"),
            true
        );
        assert_eq!(
            Name::from("hijklmnopqrsj") == Name::from("hijklmnopqrsj"),
            true
        );
        assert_eq!(
            Name::from("tuvwxyz.1234j") == Name::from("tuvwxyz.1234j"),
            true
        );

        assert_eq!(
            Name::from("111111111111j") == Name::from("111111111111j"),
            true
        );
        assert_eq!(
            Name::from("555555555555j") == Name::from("555555555555j"),
            true
        );
        assert_eq!(
            Name::from("aaaaaaaaaaaaj") == Name::from("aaaaaaaaaaaaj"),
            true
        );
        assert_eq!(
            Name::from("zzzzzzzzzzzzj") == Name::from("zzzzzzzzzzzzj"),
            true
        );

        // test constexpr
        assert_eq!(Name::from("1") == Name::from("1"), true);
    }

    #[test]
    fn test_cdt_10() {
        // friend constexpr bool operator!=(const name&, const name&)
        assert_eq!(Name::from("1") != Name::new(), true);
        assert_eq!(Name::from("5") != Name::new(), true);
        assert_eq!(Name::from("a") != Name::new(), true);
        assert_eq!(Name::from("z") != Name::new(), true);

        assert_eq!(Name::from("abc") != Name::new(), true);
        assert_eq!(Name::from("123") != Name::new(), true);

        assert_eq!(Name::from(".abc") != Name::new(), true);
        assert_eq!(Name::from(".........abc") != Name::new(), true);
        assert_eq!(Name::from("123.") != Name::new(), true);
        assert_eq!(Name::from("123.........") != Name::new(), true);
        assert_eq!(Name::from(".a.b.c.1.2.3.") != Name::new(), true);

        assert_eq!(Name::from("abc.123") != Name::new(), true);
        assert_eq!(Name::from("123.abc") != Name::new(), true);

        assert_eq!(Name::from("12345abcdefgj") != Name::new(), true);
        assert_eq!(Name::from("hijklmnopqrsj") != Name::new(), true);
        assert_eq!(Name::from("tuvwxyz.1234j") != Name::new(), true);

        assert_eq!(Name::from("111111111111j") != Name::new(), true);
        assert_eq!(Name::from("555555555555j") != Name::new(), true);
        assert_eq!(Name::from("aaaaaaaaaaaaj") != Name::new(), true);
        assert_eq!(Name::from("zzzzzzzzzzzzj") != Name::new(), true);

        // test constexpr
        assert_eq!(Name::from("1") != Name::from("2"), true);
    }

    #[test]
    fn test_cdt_11() {
        // friend constexpr bool operator<(const name&, const name&)
        assert_eq!(Name::new() < Name::from("1"), true);
        assert_eq!(Name::new() < Name::from("5"), true);
        assert_eq!(Name::new() < Name::from("a"), true);
        assert_eq!(Name::new() < Name::from("z"), true);

        assert_eq!(Name::new() < Name::from("abc"), true);
        assert_eq!(Name::new() < Name::from("123"), true);

        assert_eq!(Name::new() < Name::from(".abc"), true);
        assert_eq!(Name::new() < Name::from(".........abc"), true);
        assert_eq!(Name::new() < Name::from("123."), true);
        assert_eq!(Name::new() < Name::from("123........."), true);
        assert_eq!(Name::new() < Name::from(".a.b.c.1.2.3."), true);

        assert_eq!(Name::new() < Name::from("abc.123"), true);
        assert_eq!(Name::new() < Name::from("123.abc"), true);

        assert_eq!(Name::new() < Name::from("12345abcdefgj"), true);
        assert_eq!(Name::new() < Name::from("hijklmnopqrsj"), true);
        assert_eq!(Name::new() < Name::from("tuvwxyz.1234j"), true);

        assert_eq!(Name::new() < Name::from("111111111111j"), true);
        assert_eq!(Name::new() < Name::from("555555555555j"), true);
        assert_eq!(Name::new() < Name::from("aaaaaaaaaaaaj"), true);
        assert_eq!(Name::new() < Name::from("zzzzzzzzzzzzj"), true);

        // test constexpr
        assert_eq!(Name::new() < Name::from("1"), true);
    }

    #[test]
    #[ignore]
    #[allow(unused)]
    #[should_panic(expected = "character is not in allowed character set for names")]
    fn test_cdt_panic_1() {
        Name::from("-1");
        Name::from("0");
        Name::from("6");
    }

    #[test]
    #[ignore]
    #[allow(unused)]
    #[should_panic(expected = "thirteenth character in name cannot be a letter that comes after j")]
    fn test_cdt_panic_2() {
        Name::from("111111111111k");
        Name::from("zzzzzzzzzzzzk");
    }

    #[test]
    #[ignore]
    #[allow(unused)]
    #[should_panic(expected = "string is too long to be a valid name")]
    fn test_cdt_panic_3() {
        Name::from("12345abcdefghj");
    }

    #[test]
    #[ignore]
    #[allow(unused)]
    #[should_panic(expected = "character is not in allowed character set for names")]
    fn test_cdt_panic_4() {
        Name::char_to_value('-');
        Name::char_to_value('/');
        Name::char_to_value('6');
        Name::char_to_value('A');
        Name::char_to_value('Z');
        Name::char_to_value('`');
        Name::char_to_value('{');
    }

    #[test]
    #[ignore]
    #[allow(unused)]
    #[should_panic(expected = "string is too long to be a valid name")]
    fn test_cdt_panic_5() {
        Name::from("12345abcdefghj").length();
    }

    #[test]
    fn test_copy() {
        let name = Name::from("aaaaaaaaaaaa");
        let copied_name = name;
        assert_eq!(name, copied_name);
    }

    #[test]
    fn test_clone() {
        let name = Name::from("aaaaaaaaaaaa");
        let cloned_name = name.clone();
        assert_eq!(name, cloned_name);
    }

    #[test]
    fn test_default() {
        let default_name = Name::default();
        assert_eq!(default_name.value, 0);
    }

    #[test]
    fn test_as_ref() {
        let name = Name::from("aaaaaaaaaaaa");
        let name_ref = name.as_ref();
        assert_eq!(name_ref.to_string(), "aaaaaaaaaaaa");
    }

    proptest! {
        #[test]
        fn random_names(input in "[[1-5][a-z]]{0,12}[a-j]{0,1}") {
            let name = Name::from(input.as_str());
            prop_assert_eq!(name.to_string(), input);
        }
        #[test]
        fn random_names_with_dot(input in "[[1-5][a-z]]{1,5}[.]{0,1}[1-5][a-z]{1,5}") {
            let name = Name::from(input.as_str());
            prop_assert_eq!(name.to_string(), input);
        }
    }
}
