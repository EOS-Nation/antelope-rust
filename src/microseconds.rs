#![allow(dead_code, unused)]
use core::str;
use std::cmp::{Ord, Ordering, PartialEq, PartialOrd};
use std::convert::From;

use crate::check;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Default)]
pub struct Microseconds {
    count: i64,
}

impl Microseconds {
    pub fn new() -> Microseconds {
        Microseconds { count: 0 }
    }

    pub fn maximum() -> Microseconds {
        Microseconds::from(0x7fffffffffffffff)
    }

    pub fn count(&self) -> i64 {
        self.count
    }

    pub fn to_seconds(&self) -> i64 {
        self.count / 1000000
    }
}

impl From<i64> for Microseconds {
    fn from(count: i64) -> Self {
        Microseconds { count }
    }
}

impl From<Microseconds> for i64 {
    fn from(microseconds: Microseconds) -> i64 {
        microseconds.count
    }
}

impl std::ops::Add for Microseconds {
    type Output = Microseconds;
    fn add(self, other: Microseconds) -> Microseconds {
        Microseconds::from(self.count + other.count)
    }
}

impl std::ops::Sub for Microseconds {
    type Output = Microseconds;
    fn sub(self, other: Microseconds) -> Microseconds {
        Microseconds::from(self.count - other.count)
    }
}

impl std::ops::AddAssign for Microseconds {
    fn add_assign(&mut self, other: Microseconds) {
        self.count += other.count;
    }
}

impl std::ops::SubAssign for Microseconds {
    fn sub_assign(&mut self, other: Microseconds) {
        self.count -= other.count;
    }
}

pub fn milliseconds(ms: i64) -> Microseconds {
    Microseconds::from(ms * 1000)
}

pub fn seconds(s: i64) -> Microseconds {
    milliseconds(s * 1000)
}

pub fn minutes(m: i64) -> Microseconds {
    seconds(60 * m)
}

pub fn hours(h: i64) -> Microseconds {
    minutes(60 * h)
}

pub fn days(d: i64) -> Microseconds {
    hours(24 * d)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_microseconds_creation() {
        let micro = Microseconds::from(100);
        assert_eq!(micro.count(), 100);

        let micro = Microseconds::from(-100);
        assert_eq!(micro.count(), -100);
    }

    #[test]
    fn test_microseconds_equality() {
        let micro1 = Microseconds::from(100);
        let micro2 = Microseconds::from(100);
        assert_eq!(micro1, micro2);
    }

    #[test]
    fn test_microseconds_inequality() {
        let micro1 = Microseconds::from(100);
        let micro2 = Microseconds::from(200);
        assert_ne!(micro1, micro2);

        let micro1 = Microseconds::from(-100);
        let micro2 = Microseconds::from(100);
        assert_ne!(micro1, micro2);
    }

    #[test]
    fn test_microseconds_addition() {
        let micro1 = Microseconds::from(100);
        let micro2 = Microseconds::from(200);
        let result = micro1 + micro2;
        assert_eq!(result.count(), 300);
    }

    #[test]
    fn test_microseconds_subtraction() {
        let micro1 = Microseconds::from(100);
        let micro2 = Microseconds::from(200);
        let result = micro2 - micro1;
        assert_eq!(result.count(), 100);
    }

    #[test]
    fn test_microseconds_comparison() {
        let micro1 = Microseconds::from(100);
        let micro2 = Microseconds::from(200);
        assert!(micro1 < micro2);
        assert!(micro2 > micro1);
        assert!(micro1 <= micro2);
        assert!(micro2 >= micro1);
    }

    #[test]
    fn test_microseconds_addition_assignment() {
        let mut micro1 = Microseconds::from(100);
        let micro2 = Microseconds::from(200);
        micro1 += micro2;
        assert_eq!(micro1.count(), 300);
    }

    #[test]
    fn test_microseconds_subtraction_assignment() {
        let micro1 = Microseconds::from(100);
        let mut micro2 = Microseconds::from(200);
        micro2 -= micro1;
        assert_eq!(micro2.count(), 100);
    }

    #[test]
    fn test_microseconds_to_seconds() {
        let micro = Microseconds::from(1000000);
        assert_eq!(micro.to_seconds(), 1);
    }

    #[test]
    fn test_microseconds_time_functions() {
        let days_micro = days(1);
        let hours_micro = hours(1);
        let minutes_micro = minutes(1);
        let seconds_micro = seconds(1);

        assert_eq!(days_micro.count(), 86400000000);
        assert_eq!(hours_micro.count(), 3600000000);
        assert_eq!(minutes_micro.count(), 60000000);
        assert_eq!(seconds_micro.count(), 1000000);
    }
}
