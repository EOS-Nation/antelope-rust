#![allow(dead_code, unused)]
use core::str;
use std::cmp::{Ord, Ordering, PartialEq, PartialOrd};
use std::convert::From;

use chrono::{TimeZone, Utc};

use crate::{check, Microseconds, TimePoint};

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Default)]
pub struct TimePointSec {
    pub utc_seconds: u32,
}

impl TimePointSec {
    pub fn new() -> TimePointSec {
        TimePointSec { utc_seconds: 0 }
    }

    pub fn maximum() -> TimePointSec {
        TimePointSec { utc_seconds: 0xffffffff }
    }

    pub fn min() -> TimePointSec {
        TimePointSec { utc_seconds: 0 }
    }

    pub fn sec_since_epoch(&self) -> u32 {
        self.utc_seconds
    }

    pub fn from_iso_string(str: &str) -> Self {
        let dt = Utc.datetime_from_str(str, "%Y-%m-%dT%H:%M:%S").expect("date parsing failed");
        let seconds: u32 = dt.timestamp().try_into().unwrap_or_else(|_| {
            panic!("{} is out of range for TimePointSec", str);
        });
        TimePointSec::from(seconds)
    }
}

impl From<u32> for TimePointSec {
    fn from(seconds: u32) -> Self {
        TimePointSec { utc_seconds: seconds }
    }
}

impl From<TimePoint> for TimePointSec {
    fn from(tp: TimePoint) -> Self {
        TimePointSec::from(tp.sec_since_epoch())
    }
}

impl std::fmt::Display for TimePointSec {
    /**
     * Converts the TimePointSec into string
     *
     * @return String in the form of "%Y-%m-%dT%H:%M:%S" format (e.g. "2018-03-21T13:08:08")
     */
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let ts = crate::TimePoint::from(*self);
        write!(f, "{}", ts)
    }
}

impl std::ops::AddAssign for TimePointSec {
    fn add_assign(&mut self, other: Self) {
        self.utc_seconds += other.utc_seconds;
    }
}

impl std::ops::AddAssign<Microseconds> for TimePointSec {
    fn add_assign(&mut self, other: Microseconds) {
        self.utc_seconds += other.to_seconds() as u32;
    }
}

impl std::ops::AddAssign<u32> for TimePointSec {
    fn add_assign(&mut self, other: u32) {
        self.utc_seconds += other;
    }
}

impl std::ops::SubAssign for TimePointSec {
    fn sub_assign(&mut self, other: Self) {
        self.utc_seconds -= other.utc_seconds;
    }
}

impl std::ops::SubAssign<Microseconds> for TimePointSec {
    fn sub_assign(&mut self, other: Microseconds) {
        self.utc_seconds -= other.to_seconds() as u32;
    }
}

impl std::ops::SubAssign<u32> for TimePointSec {
    fn sub_assign(&mut self, other: u32) {
        self.utc_seconds -= other;
    }
}

impl std::ops::Add<Microseconds> for TimePointSec {
    type Output = Self;
    fn add(self, other: Microseconds) -> Self {
        let mut result = self;
        result += other;
        result
    }
}

impl std::ops::Add<u32> for TimePointSec {
    type Output = Self;
    fn add(self, other: u32) -> Self {
        let mut result = self;
        result += other;
        result
    }
}

impl std::ops::Sub<u32> for TimePointSec {
    type Output = Self;
    fn sub(self, other: u32) -> Self {
        let mut result = self;
        result -= other;
        result
    }
}

impl std::ops::Sub<Microseconds> for TimePointSec {
    type Output = Self;
    fn sub(self, other: Microseconds) -> Self {
        let mut result = self;
        result -= other;
        result
    }
}

impl std::ops::Sub for TimePointSec {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        let mut result = self;
        result -= other;
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::seconds;

    use super::*;

    #[test]
    fn test_new() {
        let time_point = TimePointSec::new();
        assert_eq!(time_point.utc_seconds, 0);
    }

    #[test]
    fn test_maximum() {
        let time_point = TimePointSec::maximum();
        assert_eq!(time_point.utc_seconds, 0xffffffff);
    }

    #[test]
    fn test_min() {
        let time_point = TimePointSec::min();
        assert_eq!(time_point.utc_seconds, 0);
    }

    #[test]
    fn test_partial_eq() {
        let time_point1 = TimePointSec { utc_seconds: 123 };
        let time_point2 = TimePointSec { utc_seconds: 123 };
        let time_point3 = TimePointSec { utc_seconds: 456 };

        assert_eq!(time_point1, time_point2);
        assert_ne!(time_point1, time_point3);
    }

    #[test]
    fn test_partial_ord() {
        let time_point1 = TimePointSec { utc_seconds: 123 };
        let time_point2 = TimePointSec { utc_seconds: 456 };
        let time_point3 = TimePointSec { utc_seconds: 789 };

        assert!(time_point1 < time_point2);
        assert!(time_point2 > time_point1);
        assert!(time_point3 > time_point2);
    }

    #[test]
    fn test_eq() {
        let time_point1 = TimePointSec { utc_seconds: 123 };
        let time_point2 = TimePointSec { utc_seconds: 123 };
        let time_point3 = TimePointSec { utc_seconds: 456 };

        assert_eq!(time_point1.eq(&time_point2), true);
        assert_eq!(time_point1.eq(&time_point3), false);
    }

    #[test]
    fn test_cmp() {
        let time_point1 = TimePointSec { utc_seconds: 123 };
        let time_point2 = TimePointSec { utc_seconds: 456 };
        let time_point3 = TimePointSec { utc_seconds: 789 };

        assert_eq!(time_point1.cmp(&time_point2), Ordering::Less);
        assert_eq!(time_point2.cmp(&time_point1), Ordering::Greater);
        assert_eq!(time_point2.cmp(&time_point3), Ordering::Less);
    }

    #[test]
    fn test_iso_string() {
        assert_eq!(TimePointSec::from_iso_string("1970-01-01T00:00:00").sec_since_epoch(), 0);
        assert_eq!(TimePointSec::from_iso_string("1998-06-15T08:13:12").sec_since_epoch(), 897898392);
        assert_eq!(TimePointSec::from_iso_string("2020-01-01T00:00:00").sec_since_epoch(), 1577836800);
        assert_eq!(TimePointSec::from_iso_string("2038-01-19T03:14:07").sec_since_epoch(), 2147483647);
        assert_eq!(
            TimePointSec::from_iso_string("1998-06-15T08:13:12").to_string(),
            "1998-06-15T08:13:12"
        );
        assert_eq!(
            TimePointSec::from_iso_string("2038-01-19T03:14:07").to_string(),
            "2038-01-19T03:14:07"
        );
    }

    #[test]
    #[should_panic(expected = "date parsing failed")]
    fn test_iso_string_panic() {
        TimePointSec::from_iso_string("invalid_string");
    }

    #[test]
    #[should_panic(expected = "date parsing failed")]
    fn test_iso_string_panic2() {
        TimePointSec::from_iso_string("2010-13-81T00:00:00");
    }

    #[test]
    fn test_add_assign_self() {
        let mut tp1 = TimePointSec::from(100u32);
        let tp2 = TimePointSec::from(50u32);
        tp1 += tp2;
        assert_eq!(tp1.sec_since_epoch(), 150);
    }

    #[test]
    fn test_add_assign_ms() {
        let mut tp = TimePointSec::from(100);
        let elapsed = Microseconds::from(50_000_000);
        tp += elapsed;
        assert_eq!(tp.sec_since_epoch(), 150);
    }

    #[test]
    fn test_add_assign_u32() {
        let mut tp1 = TimePointSec::from(100);
        tp1 += 50u32;
        assert_eq!(tp1.sec_since_epoch(), 150);
    }

    #[test]
    fn test_sub_assign_self() {
        let mut tp1 = TimePointSec::from(100u32);
        let tp2 = TimePointSec::from(50u32);
        tp1 -= tp2;
        assert_eq!(tp1.sec_since_epoch(), 50);
    }

    #[test]
    fn test_sub_assign_ms() {
        let mut tp = TimePointSec::from(100);
        let elapsed = Microseconds::from(50_000_000);
        tp -= elapsed;
        assert_eq!(tp.sec_since_epoch(), 50);
    }

    #[test]
    fn test_sub_assign_u32() {
        let mut tp1 = TimePointSec::from(100);
        tp1 -= 50u32;
        assert_eq!(tp1.sec_since_epoch(), 50);
    }

    #[test]
    fn test_add_s() {
        let tp1 = TimePointSec::from(100);
        let tp2 = tp1 + 50;
        assert_eq!(tp2.sec_since_epoch(), 150);
    }

    #[test]
    fn test_add_ms() {
        let tp1 = TimePointSec::from(100);
        let tp2 = tp1 + Microseconds::from(50_000_000);
        assert_eq!(tp2.sec_since_epoch(), 150);
    }

    #[test]
    fn test_sub_self() {
        let tp1 = TimePointSec::from(100);
        let tp2 = TimePointSec::from(50);
        assert_eq!((tp1 - tp2).sec_since_epoch(), 50);
    }

    #[test]
    fn test_sub_s() {
        let tp1 = TimePointSec::from(100);
        assert_eq!((tp1 - 50).sec_since_epoch(), 50);
    }

    #[test]
    fn test_sub_ms() {
        let tp1 = TimePointSec::from(100);
        assert_eq!((tp1 - Microseconds::from(50_000_000)).sec_since_epoch(), 50);
    }
}
