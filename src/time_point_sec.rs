#![allow(dead_code, unused)]
use core::str;
use std::cmp::{Ord, Ordering, PartialEq, PartialOrd};
use std::convert::From;

use crate::check;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Default)]
pub struct TimePointSec {
    utc_seconds: u32,
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
}

impl From<u32> for TimePointSec {
    fn from(seconds: u32) -> Self {
        TimePointSec { utc_seconds: seconds }
    }
}

#[cfg(test)]
mod tests {
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
}
