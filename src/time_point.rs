// use chrono::{TimeZone, Utc};

use crate::{Microseconds, TimePointSec};

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Default)]
pub struct TimePoint {
    elapsed: Microseconds,
}

impl TimePoint {
    pub fn new() -> TimePoint {
        TimePoint {
            elapsed: Microseconds::new(),
        }
    }

    pub fn time_since_epoch(&self) -> Microseconds {
        self.elapsed
    }

    pub fn sec_since_epoch(&self) -> u32 {
        self.elapsed.to_seconds() as u32
    }

    // pub fn from_iso_string(str: &str) -> Self {
    //     let dt = Utc.datetime_from_str(str, "%Y-%m-%dT%H:%M:%S").expect("date parsing failed");

    //     TimePoint::from(crate::seconds(dt.timestamp()))
    // }
}

impl From<Microseconds> for TimePoint {
    #[inline]
    #[must_use]
    fn from(elapsed: Microseconds) -> Self {
        TimePoint { elapsed }
    }
}

impl From<TimePointSec> for TimePoint {
    fn from(tps: TimePointSec) -> Self {
        TimePoint::from(crate::seconds(tps.sec_since_epoch() as i64))
    }
}

impl AsRef<TimePoint> for TimePoint {
    #[inline]
    #[must_use]
    fn as_ref(&self) -> &TimePoint {
        self
    }
}

// impl std::fmt::Display for TimePoint {
//     /**
//      * Converts the TimePoint into string
//      *
//      * @return String in the form of "%Y-%m-%dT%H:%M:%S" format (e.g. "2018-03-21T13:08:08")
//      */
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         let dt = Utc.timestamp_opt(self.sec_since_epoch() as i64, 0).unwrap();

//         write!(f, "{}", dt.format("%Y-%m-%dT%H:%M:%S"))
//     }
// }

impl std::ops::AddAssign for TimePoint {
    fn add_assign(&mut self, other: Self) {
        self.elapsed += other.elapsed;
    }
}

impl std::ops::AddAssign<Microseconds> for TimePoint {
    fn add_assign(&mut self, other: Microseconds) {
        self.elapsed += other;
    }
}

impl std::ops::SubAssign for TimePoint {
    fn sub_assign(&mut self, other: Self) {
        self.elapsed -= other.elapsed;
    }
}

impl std::ops::SubAssign<Microseconds> for TimePoint {
    fn sub_assign(&mut self, other: Microseconds) {
        self.elapsed -= other;
    }
}

impl std::ops::Add for TimePoint {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let mut result = self;
        result += other;
        result
    }
}

impl std::ops::Sub for TimePoint {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        let mut result = self;
        result -= other;
        result
    }
}

impl std::ops::Sub<Microseconds> for TimePoint {
    type Output = Self;
    fn sub(self, other: Microseconds) -> Self {
        TimePoint::from(self.elapsed - other)
    }
}

impl std::ops::Add<Microseconds> for TimePoint {
    type Output = Self;
    fn add(self, other: Microseconds) -> Self {
        TimePoint::from(self.elapsed + other)
    }
}

impl std::ops::Sub<TimePointSec> for TimePoint {
    type Output = Self;
    fn sub(self, other: TimePointSec) -> Self {
        self - TimePoint::from(other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from() {
        let ms = Microseconds::from(1234567);
        let tp = TimePoint::from(ms);
        assert_eq!(tp.elapsed, ms);
    }

    #[test]
    fn test_time_since_epoch() {
        let ms = Microseconds::from(1234567);
        let tp = TimePoint::from(ms);
        assert_eq!(tp.time_since_epoch(), ms);
    }

    #[test]
    fn test_sec_since_epoch() {
        let ms = Microseconds::from(1234567890123);
        let tp = TimePoint::from(ms);
        assert_eq!(tp.sec_since_epoch(), 1234567);
    }

    // #[test]
    // fn test_display() {
    //     assert_eq!(TimePoint::from(Microseconds::new()).to_string(), "1970-01-01T00:00:00");
    //     assert_eq!(
    //         TimePoint::from(Microseconds::from(897898392000000)).to_string(),
    //         "1998-06-15T08:13:12"
    //     );
    //     assert_eq!(
    //         TimePoint::from(Microseconds::from(2147483647000000)).to_string(),
    //         "2038-01-19T03:14:07"
    //     );
    // }

    // #[test]
    // fn test_iso_string() {
    //     assert_eq!(TimePoint::from_iso_string("1970-01-01T00:00:00").elapsed, Microseconds::new());
    //     assert_eq!(TimePoint::from_iso_string("1998-06-15T08:13:12").elapsed.count(), 897898392000000);
    //     assert_eq!(TimePoint::from_iso_string("2020-01-01T00:00:00").elapsed.count(), 1577836800000000);
    //     assert_eq!(TimePoint::from_iso_string("2038-01-19T03:14:07").elapsed.count(), 2147483647000000);
    //     assert_eq!(TimePoint::from_iso_string("1998-06-15T08:13:12").to_string(), "1998-06-15T08:13:12");
    //     assert_eq!(TimePoint::from_iso_string("2038-01-19T03:14:07").to_string(), "2038-01-19T03:14:07");
    // }

    // #[test]
    // #[should_panic(expected = "date parsing failed")]
    // fn test_iso_string_panic() {
    //     TimePoint::from_iso_string("invalid_string").elapsed.count();
    // }

    // #[test]
    // #[should_panic(expected = "date parsing failed")]
    // fn test_iso_string_panic2() {
    //     TimePoint::from_iso_string("2010-13-81T00:00:00").elapsed.count();
    // }

    #[test]
    fn test_eq() {
        let tp1 = TimePoint::from(Microseconds::from(100));
        let tp2 = TimePoint::from(Microseconds::from(100));
        assert_eq!(tp1, tp2);
    }

    #[test]
    fn test_ne() {
        let tp1 = TimePoint::from(Microseconds::from(100));
        let tp2 = TimePoint::from(Microseconds::from(200));
        assert_ne!(tp1, tp2);
    }

    #[test]
    fn test_lt() {
        let tp1 = TimePoint::from(Microseconds::from(100));
        let tp2 = TimePoint::from(Microseconds::from(200));
        assert!(tp1 < tp2);
    }

    #[test]
    fn test_le() {
        let tp1 = TimePoint::from(Microseconds::from(100));
        let tp2 = TimePoint::from(Microseconds::from(200));
        let tp3 = TimePoint::from(Microseconds::from(100));
        assert!(tp1 <= tp2);
        assert!(tp1 <= tp3);
    }

    #[test]
    fn test_gt() {
        let tp1 = TimePoint::from(Microseconds::from(200));
        let tp2 = TimePoint::from(Microseconds::from(100));
        assert!(tp1 > tp2);
    }

    #[test]
    fn test_ge() {
        let tp1 = TimePoint::from(Microseconds::from(200));
        let tp2 = TimePoint::from(Microseconds::from(100));
        let tp3 = TimePoint::from(Microseconds::from(200));
        assert!(tp1 >= tp2);
        assert!(tp1 >= tp3);
    }

    #[test]
    fn test_add_assign() {
        let mut tp = TimePoint::from(Microseconds::from(100));
        let elapsed = Microseconds::from(50);
        tp += elapsed;
        assert_eq!(tp.elapsed, Microseconds::from(150));
    }

    #[test]
    fn test_add_assign_self() {
        let mut tp1 = TimePoint::from(Microseconds::from(100));
        let tp2 = TimePoint::from(Microseconds::from(50));
        tp1 += tp2;
        assert_eq!(tp1.elapsed, Microseconds::from(150));
    }

    #[test]
    fn test_sub_assign() {
        let mut tp = TimePoint::from(Microseconds::from(100));
        let elapsed = Microseconds::from(50);
        tp -= elapsed;
        assert_eq!(tp.elapsed, Microseconds::from(50));
    }

    #[test]
    fn test_sub_assign_self() {
        let mut tp1 = TimePoint::from(Microseconds::from(100));
        let tp2 = TimePoint::from(Microseconds::from(50));
        tp1 -= tp2;
        assert_eq!(tp1.elapsed, Microseconds::from(50));
    }

    #[test]
    fn test_add() {
        let tp1 = TimePoint::from(Microseconds::from(100));
        let tp2 = TimePoint::from(Microseconds::from(50));
        let tp3 = tp1 + tp2;
        assert_eq!(tp3.elapsed, Microseconds::from(150));
    }

    #[test]
    fn test_add_ms() {
        let tp = TimePoint::from(Microseconds::from(100));
        assert_eq!((tp + Microseconds::from(50)).elapsed, Microseconds::from(150));
    }

    #[test]
    fn test_sub() {
        let tp1 = TimePoint::from(Microseconds::from(100));
        let tp2 = TimePoint::from(Microseconds::from(50));
        assert_eq!((tp1 - tp2).elapsed, Microseconds::from(50));
    }

    #[test]
    fn test_sub_ms() {
        let tp = TimePoint::from(Microseconds::from(100));
        assert_eq!((tp - Microseconds::from(50)).elapsed, Microseconds::from(50));
    }
}
