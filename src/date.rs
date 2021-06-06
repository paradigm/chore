//
// All datetimes in chore are implicitly durations over the provided resolution.
// For example, "2001-02-03" describes 2001-02-03T00:00:00 to 2001-02-03T23:59:59.
//

use chrono::{Datelike, Timelike};

#[derive(Clone, Debug, PartialEq)]
pub struct Date {
    start: chrono::NaiveDateTime,
    duration: self::Duration,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Duration {
    Seconds(i64),
    Minutes(i64),
    Hours(i64),
    Days(i64),
    Months(i64),
    Years(i32),
}
use Duration::*;

impl Date {
    pub fn from_chrono(date: &chrono::NaiveDateTime) -> Date {
        Date {
            start: date.with_nanosecond(0).unwrap(),
            duration: Seconds(1),
        }
    }

    pub fn new(str: &str, cf: &Date) -> Option<Date> {
        Date::from_abs(str).or_else(|| Date::from_rel(str, cf))
    }

    pub fn from_abs(str: &str) -> Option<Date> {
        let (year, str) = (str.get(0..4)?.parse().ok()?, str.get(4..)?);
        if str.is_empty() {
            return Date::from_fields(year, 1, 1, 0, 0, 0, Years(1));
        }

        let (month, str) = Date::parse_field(str, '-')?;
        if str.is_empty() {
            return Date::from_fields(year, month, 1, 0, 0, 0, Months(1));
        }

        let (day, str) = Date::parse_field(str, '-')?;
        if str.is_empty() {
            return Date::from_fields(year, month, day, 0, 0, 0, Days(1));
        }

        let (hour, str) = Date::parse_field(str, 'T')?;
        if str.is_empty() {
            return Date::from_fields(year, month, day, hour, 0, 0, Hours(1));
        }

        let (minute, str) = Date::parse_field(str, ':')?;
        if str.is_empty() {
            return Date::from_fields(year, month, day, hour, minute, 0, Minutes(1));
        }

        let (second, str) = Date::parse_field(str, ':')?;
        if str.is_empty() {
            return Date::from_fields(year, month, day, hour, minute, second, Seconds(1));
        }

        None
    }

    pub fn from_rel(str: &str, cf: &Date) -> Option<Date> {
        Date::from_relative_hms(str, cf)
            .or_else(|| Date::from_relative_hm(str, cf))
            .or_else(|| Date::from_relative_hour(str, cf))
            .or_else(|| Date::from_relative_weekday(str, cf))
            .or_else(|| Date::from_relative_day_of_month(str, cf))
            .or_else(|| Date::from_relative_month(str, cf))
            .or_else(|| Date::from_relative_offset(str, cf))
            .or_else(|| Date::from_relative_named(str, cf))
    }

    fn from_relative_hms(str: &str, cf: &Date) -> Option<Date> {
        let target = chrono::NaiveTime::parse_from_str(str, "%H:%M:%S").ok()?;
        let date = Date {
            start: cf.start,
            duration: Seconds(1),
        };
        Some(Date::next_time(date, &target))
    }

    fn from_relative_hm(str: &str, cf: &Date) -> Option<Date> {
        let target = chrono::NaiveTime::parse_from_str(str, "%H:%M").ok()?;
        let date = Date {
            start: cf.start,
            duration: Minutes(1),
        };
        Some(Date::next_time(date, &target))
    }

    fn from_relative_hour(str: &str, cf: &Date) -> Option<Date> {
        let hour: u32 = str.parse().ok()?;
        let target = chrono::NaiveTime::from_hms_opt(hour, 0, 0)?;
        let date = Date {
            start: cf.start,
            duration: Hours(1),
        };
        Some(Date::next_time(date, &target))
    }

    fn from_relative_weekday(str: &str, cf: &Date) -> Option<Date> {
        let weekday = str.parse::<chrono::Weekday>().ok()?;
        let current_weekday: i64 = cf.start.weekday().number_from_monday().into();
        let target_weekday: i64 = weekday.number_from_monday().into();

        let date = Date {
            start: cf.start.with_second(0)?.with_minute(0)?.with_hour(0)?,
            duration: Days(1),
        };

        if target_weekday > current_weekday {
            Some(date + Days(target_weekday - current_weekday))
        } else {
            Some(date + Days(7 + target_weekday - current_weekday))
        }
    }

    fn from_relative_day_of_month(str: &str, cf: &Date) -> Option<Date> {
        let target_day = str
            .strip_suffix("st")
            .or_else(|| str.strip_suffix("nd"))
            .or_else(|| str.strip_suffix("rd"))
            .or_else(|| str.strip_suffix("th"))?
            .parse::<i64>()
            .ok()
            .filter(|&day| (1..=31).contains(&day))?;
        let current_day = cf.start.day() as i64;

        let mut date = Date {
            start: cf.start.with_second(0)?.with_minute(0)?.with_hour(0)?,
            duration: Days(1),
        };
        let day_delta = Days(target_day - current_day);

        if cf.start.day() as i64 >= target_day {
            date += Months(1);
        }
        if (&date + &day_delta).start.month() != date.start.month() {
            date += Months(1);
        }
        Some(date + day_delta)
    }

    fn from_relative_month(str: &str, cf: &Date) -> Option<Date> {
        let target_month = str.parse::<chrono::Month>().ok()?.number_from_month() as i64;
        let current_month = cf.start.month() as i64;
        let date = Date {
            start: cf
                .start
                .with_second(0)?
                .with_minute(0)?
                .with_hour(0)?
                .with_day(1)?,
            duration: Months(1),
        };

        if target_month > current_month {
            Some(date + Months(target_month - current_month))
        } else {
            Some(date + Years(1) + Months(target_month - current_month))
        }
    }

    fn from_relative_offset(str: &str, cf: &Date) -> Option<Date> {
        let offset = str
            .strip_suffix('s')
            .and_then(|str| str.parse::<i64>().ok())
            .map(Seconds)
            .or_else(|| {
                str.strip_suffix('m')
                    .and_then(|str| str.parse::<i64>().ok())
                    .map(Minutes)
            })
            .or_else(|| {
                str.strip_suffix('h')
                    .and_then(|str| str.parse::<i64>().ok())
                    .map(Hours)
            })
            .or_else(|| {
                str.strip_suffix('d')
                    .and_then(|str| str.parse::<i64>().ok())
                    .map(Days)
            })
            .or_else(|| {
                str.strip_suffix('W')
                    .and_then(|str| str.parse::<i64>().ok())
                    // Handle first sub-week section which goes through a
                    // weekend directly
                    .map(|dur| match (dur, cf.start.weekday()) {
                        (i64::MIN..=-1, chrono::Weekday::Sun) => (dur + 1, -2),
                        (0, chrono::Weekday::Sun) => (dur, 0),
                        (1..=i64::MAX, chrono::Weekday::Sun) => (dur - 1, 1),
                        (i64::MIN..=-1, chrono::Weekday::Mon) => (dur + 1, -3),
                        (0..=i64::MAX, chrono::Weekday::Mon) => (dur, 0),
                        (i64::MIN..=-2, chrono::Weekday::Tue) => (dur + 2, -4),
                        (-1..=3, chrono::Weekday::Tue) => (dur, 0),
                        (4..=i64::MAX, chrono::Weekday::Tue) => (dur - 4, 6),
                        (i64::MIN..=-3, chrono::Weekday::Wed) => (dur + 3, -5),
                        (-2..=2, chrono::Weekday::Wed) => (dur, 0),
                        (3..=i64::MAX, chrono::Weekday::Wed) => (dur - 3, 5),
                        (i64::MIN..=-4, chrono::Weekday::Thu) => (dur + 4, -6),
                        (-3..=1, chrono::Weekday::Thu) => (dur, 0),
                        (2..=i64::MAX, chrono::Weekday::Thu) => (dur - 2, 4),
                        (i64::MIN..=-5, chrono::Weekday::Fri) => (dur + 5, -7),
                        (-4..=0, chrono::Weekday::Fri) => (dur, 0),
                        (1..=i64::MAX, chrono::Weekday::Fri) => (dur - 1, 3),
                        (i64::MIN..=-6, chrono::Weekday::Sat) => (dur + 6, -8),
                        (-5, chrono::Weekday::Sat) => (dur + 5, -5),
                        (-4..=0, chrono::Weekday::Sat) => (dur, 0),
                        (1..=i64::MAX, chrono::Weekday::Sat) => (dur -1, 2),
                    })
                    // Do rest by converting every five weekdays to seven days
                    .map(|(dur, adj)| Days((dur / 5 * 7) + (dur % 5) + adj))
            })
            .or_else(|| {
                str.strip_suffix('w')
                    .and_then(|str| str.parse::<i64>().ok())
                    .map(|dur| Days(dur * 7))
            })
            .or_else(|| {
                str.strip_suffix('M')
                    .and_then(|str| str.parse::<i64>().ok())
                    .map(Months)
            })
            .or_else(|| {
                str.strip_suffix('y')
                    .and_then(|str| str.parse::<i32>().ok())
                    .map(Years)
            })?;

        Some(cf + offset)
    }

    fn from_relative_named(str: &str, cf: &Date) -> Option<Date> {
        let offset = if str.eq("today") {
            Duration::Days(0)
        } else if str.eq("tomorrow") || str.eq("tom") {
            Duration::Days(1)
        } else if str.eq("yesterday") || str.eq("yes") {
            Duration::Days(-1)
        } else if str.eq("now") {
            return Some(cf.clone());
        } else {
            return None;
        };

        Some(
            Date {
                start: cf.start.with_second(0)?.with_minute(0)?.with_hour(0)?,
                duration: Duration::Days(1),
            } + offset,
        )
    }

    fn parse_field(str: &str, sep: char) -> Option<(u32, &str)> {
        if !str.starts_with(sep) {
            None
        } else if let (Some(val), Some(str)) =
            (str.get(1..3).and_then(|s| s.parse().ok()), str.get(3..))
        {
            Some((val, str))
        } else {
            None
        }
    }

    fn from_fields(
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        minute: u32,
        second: u32,
        duration: Duration,
    ) -> Option<Date> {
        Some(Date {
            start: chrono::NaiveDate::from_ymd_opt(year, month, day)?
                .and_hms_opt(hour, minute, second)?,
            duration,
        })
    }

    fn next_time(mut date: Date, target: &chrono::NaiveTime) -> Date {
        if target.second() < date.start.second() {
            date += Minutes(1);
        };
        date += Seconds(target.second() as i64 - date.start.second() as i64);

        if target.minute() < date.start.minute() {
            date += Hours(1);
        };
        date += Minutes(target.minute() as i64 - date.start.minute() as i64);

        if target.hour() < date.start.hour() {
            date += Days(1);
        };
        date += Hours(target.hour() as i64 - date.start.hour() as i64);
        date
    }

    pub fn within(&self, other: &Date) -> bool {
        other.start <= self.start && other.end() >= self.end()
    }

    pub fn before(&self, other: &Date) -> bool {
        self.end() <= other.start
    }

    pub fn after(&self, other: &Date) -> bool {
        other.end() <= self.start
    }

    fn end(&self) -> chrono::NaiveDateTime {
        (self + &self.duration).start
    }
}

impl std::fmt::Display for Date {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.duration {
            Years(_) => write!(fmt, "{:04}", self.start.year()),
            Months(_) => {
                write!(fmt, "{:04}-{:02}", self.start.year(), self.start.month())
            }
            Days(_) => write!(
                fmt,
                "{:04}-{:02}-{:02}",
                self.start.year(),
                self.start.month(),
                self.start.day()
            ),
            Hours(_) => write!(
                fmt,
                "{:04}-{:02}-{:02}T{:02}",
                self.start.year(),
                self.start.month(),
                self.start.day(),
                self.start.hour()
            ),
            Minutes(_) => write!(
                fmt,
                "{:04}-{:02}-{:02}T{:02}:{:02}",
                self.start.year(),
                self.start.month(),
                self.start.day(),
                self.start.hour(),
                self.start.minute()
            ),
            Seconds(_) => write!(
                fmt,
                "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}",
                self.start.year(),
                self.start.month(),
                self.start.day(),
                self.start.hour(),
                self.start.minute(),
                self.start.second()
            ),
        }
    }
}

impl std::ops::Add<&Duration> for &Date {
    type Output = Date;

    fn add(self, other: &Duration) -> Date {
        let start = match *other {
            Seconds(dur) => self.start + chrono::Duration::seconds(dur),
            Minutes(dur) => self.start + chrono::Duration::minutes(dur),
            Hours(dur) => self.start + chrono::Duration::hours(dur),
            Days(dur) => self.start + chrono::Duration::days(dur),
            // chrono does not provide month duration because of variable month length.
            // Shorten to fit.
            Months(dur) => {
                let months_from_epoch =
                    (self.start.year() * 12) + (self.start.month0() as i32) + dur as i32;
                let new_year = months_from_epoch / 12;
                let new_month0 = (months_from_epoch % 12) as u32;
                let new_day = self.start.day();

                let new_date = self
                    .start
                    .with_day(1)
                    .unwrap() // e.g. January 32nd; can't happen with 1st
                    .with_month0(new_month0)
                    .unwrap() // e.g. 13th month; can't happen with above `% 12`
                    .with_year(new_year)
                    .unwrap(); // e.g. leap day within non-leap year; can't happen with 1st

                // Most extreme input is 31st but maximum day for the given month is 28th.  Try
                // reducing target date to fit up to 31-28 = 3 times.
                new_date
                    .with_day(new_day) // 31st
                    .or_else(|| new_date.with_day(new_day - 1)) // 30th
                    .or_else(|| new_date.with_day(new_day - 2)) // 29th
                    .or_else(|| new_date.with_day(new_day - 3)) // 28th
                    .unwrap()
            }
            // chrono does not provide month duration because of variable year length, i.e. leap
            // years day.  Shorten to fit.
            Years(dur) => {
                let target_year = self.start.year() + dur;
                self.start
                    .with_year(target_year)
                    .or_else(|| (self.start - chrono::Duration::days(1)).with_year(target_year))
                    .unwrap()
            }
        };

        Date {
            start,
            duration: self.duration.clone(),
        }
    }
}

impl std::ops::Add<Duration> for Date {
    type Output = Date;

    fn add(self, other: Duration) -> Date {
        &self + &other
    }
}

impl std::ops::Add<Duration> for &Date {
    type Output = Date;

    fn add(self, other: Duration) -> Date {
        self + &other
    }
}

impl std::ops::AddAssign<Duration> for Date {
    fn add_assign(&mut self, other: Duration) {
        *self = &(*self) + &other;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_now() -> Date {
        Date::from_fields(2001, 2, 3, 4, 5, 6, Seconds(1)).unwrap()
    }

    #[test]
    fn from_abs() {
        for (input, expect) in &[
            // happy path
            (
                "2001-02-03T04:05:06",
                Date::from_fields(2001, 2, 3, 4, 5, 6, Seconds(1)),
            ),
            (
                "2001-02-03T04:05",
                Date::from_fields(2001, 2, 3, 4, 5, 0, Minutes(1)),
            ),
            (
                "2001-02-03T04",
                Date::from_fields(2001, 2, 3, 4, 0, 0, Hours(1)),
            ),
            (
                "2001-02-03",
                Date::from_fields(2001, 2, 3, 0, 0, 0, Days(1)),
            ),
            ("2001-02", Date::from_fields(2001, 2, 1, 0, 0, 0, Months(1))),
            ("2001", Date::from_fields(2001, 1, 1, 0, 0, 0, Years(1))),
            // unhappy path: non-num fields
            ("2001-02-03T04:05:xx", None),
            ("2001-02-03T04:05x", None),
            ("2001-02-03T04:xx", None),
            ("2001-02-03Txx", None),
            ("2001-02-xx", None),
            ("2001-xx", None),
            // unhappy path: invalid dates
            ("2001-01-32T00:00:00", None),
            ("2001-02-03T04:05:60", None),
            ("2000-02-30", None),
            ("2001-02-29", None),
            // unhappy path: bad field sizes
            ("2001-02-03T04:05:6", None),
            ("2001-02-03T04:5:06", None),
            ("2001-02-03T4:05:06", None),
            ("2001-02-3T04:05:06", None),
            ("2001-2-03T04:05:06", None),
            ("201-02-03T04:05:06", None),
            // unhappy path: bad field separators
            ("2001-02-03t04:05:06", None),
            ("2001:02-03T04:05:06", None),
            ("2001-02:03T04:05:06", None),
            ("2001-02-03:04:05:06", None),
            ("2001T02-03T04:05:06", None),
            ("2001-02T03T04:05:06", None),
            ("2001-02-03T04T05:06", None),
            ("2001-02-03T04:05T06", None),
            ("2001-02-03-04:05:06", None),
            ("2001-02-03T04-05:06", None),
            ("2001-02-03T04:05-06", None),
            // unhappy path: trailing content
            ("2001-02-03T04:05:06 ", None),
            // unhappy path: relative dates
            ("tomorrow", None),
            ("2h", None),
            // unhappy path: not a date
            ("this is not a date", None),
            ("", None),
        ] {
            let abs_input = Date::from_abs(input);
            assert_eq!(abs_input, *expect);
            if expect.is_some() {
                let any_input = Date::new(input, &test_now());
                assert_eq!(any_input, *expect);
            }
        }
    }

    #[test]
    fn from_relative_hms() {
        for (input, expect) in &[
            // wrap day
            ("00:00:00", Date::from_abs("2001-02-04T00:00:00")),
            ("01:02:03", Date::from_abs("2001-02-04T01:02:03")),
            ("04:05:05", Date::from_abs("2001-02-04T04:05:05")),
            ("04:04:06", Date::from_abs("2001-02-04T04:04:06")),
            ("03:05:06", Date::from_abs("2001-02-04T03:05:06")),
            // same as current time, retain time
            ("04:05:06", Date::from_abs("2001-02-03T04:05:06")),
            // same day
            ("04:05:07", Date::from_abs("2001-02-03T04:05:07")),
            ("04:06:06", Date::from_abs("2001-02-03T04:06:06")),
            ("05:05:06", Date::from_abs("2001-02-03T05:05:06")),
            // allow short field sizes
            ("4:05:06", Date::from_abs("2001-02-03T04:05:06")),
            ("04:5:06", Date::from_abs("2001-02-03T04:05:06")),
            ("04:05:6", Date::from_abs("2001-02-03T04:05:06")),
            // unhappy path, bad field sizes
            ("004:05:06", None),
            ("04:005:06", None),
            ("04:05:006", None),
            ("04::05:06", None),
            ("04::5:06", None),
            // unhappy path: impossible times
            ("04:05:61", None),
            ("04:60:06", None),
            ("24:25:06", None),
        ] {
            let rel_input = Date::from_rel(input, &test_now());
            assert_eq!(rel_input, *expect);
            if expect.is_some() {
                let any_input = Date::new(input, &test_now());
                assert_eq!(any_input, *expect);
            }
        }
    }

    #[test]
    fn from_relative_hm() {
        for (input, expect) in &[
            // wrap day
            ("00:00", Date::from_abs("2001-02-04T00:00")),
            ("01:02", Date::from_abs("2001-02-04T01:02")),
            ("04:04", Date::from_abs("2001-02-04T04:04")),
            ("03:05", Date::from_abs("2001-02-04T03:05")),
            ("04:05", Date::from_abs("2001-02-04T04:05")),
            // retain day
            ("04:06", Date::from_abs("2001-02-03T04:06")),
            ("05:05", Date::from_abs("2001-02-03T05:05")),
            // allow short field sizes
            ("1:2", Date::from_abs("2001-02-04T01:02")),
            // unhappy path
            ("004:05", None),
            ("04:005", None),
            ("04::05", None),
            ("04::5", None),
        ] {
            let rel_input = Date::from_rel(input, &test_now());
            assert_eq!(rel_input, *expect);
            if expect.is_some() {
                let any_input = Date::new(input, &test_now());
                assert_eq!(any_input, *expect);
            }
        }
    }

    #[test]
    fn from_relative_hour() {
        for (input, expect) in &[
            // wrap day
            ("00", Date::from_abs("2001-02-04T00")),
            // ("01", Date::from_abs("2001-02-04T01")),
            // ("04", Date::from_abs("2001-02-04T04")),
            // ("03", Date::from_abs("2001-02-04T03")),
            // // retain day
            // ("05", Date::from_abs("2001-02-03T05")),
            // // allow short field sizes
            // ("1",  Date::from_abs("2001-02-04T01")),
            // // unhappy path
            // ("004:", None),
            // (":005", None),
            // ("4::", None),
            // (":4:", None),
        ] {
            let rel_input = Date::from_rel(input, &test_now());
            assert_eq!(rel_input, *expect);
            if expect.is_some() {
                let any_input = Date::new(input, &test_now());
                assert_eq!(any_input, *expect);
            }
        }
    }

    #[test]
    fn from_relative_weekday() {
        for (input, expect) in &[
            ("sun", Date::from_abs("2001-02-04")),
            ("Sun", Date::from_abs("2001-02-04")),
            ("sunday", Date::from_abs("2001-02-04")),
            ("Sunday", Date::from_abs("2001-02-04")),
            ("mon", Date::from_abs("2001-02-05")),
            ("Mon", Date::from_abs("2001-02-05")),
            ("monday", Date::from_abs("2001-02-05")),
            ("Monday", Date::from_abs("2001-02-05")),
            ("tue", Date::from_abs("2001-02-06")),
            ("Tue", Date::from_abs("2001-02-06")),
            ("tuesday", Date::from_abs("2001-02-06")),
            ("Tuesday", Date::from_abs("2001-02-06")),
            ("wed", Date::from_abs("2001-02-07")),
            ("Wed", Date::from_abs("2001-02-07")),
            ("wednesday", Date::from_abs("2001-02-07")),
            ("Wednesday", Date::from_abs("2001-02-07")),
            ("thu", Date::from_abs("2001-02-08")),
            ("Thu", Date::from_abs("2001-02-08")),
            ("thursday", Date::from_abs("2001-02-08")),
            ("Thursday", Date::from_abs("2001-02-08")),
            ("fri", Date::from_abs("2001-02-09")),
            ("Fri", Date::from_abs("2001-02-09")),
            ("friday", Date::from_abs("2001-02-09")),
            ("Friday", Date::from_abs("2001-02-09")),
            ("sat", Date::from_abs("2001-02-10")),
            ("Sat", Date::from_abs("2001-02-10")),
            // same as current day, bump to next week
            ("saturday", Date::from_abs("2001-02-10")),
            ("Saturday", Date::from_abs("2001-02-10")),
            // unhappy path
            ("", None),
            ("s", None),
            ("satu", None),
            ("stu", None),
            ("saturdya", None),
            ("saturdayx", None),
            ("satxurday", None),
        ] {
            let rel_input = Date::from_rel(input, &test_now());
            assert_eq!(rel_input, *expect);
            if expect.is_some() {
                let any_input = Date::new(input, &test_now());
                assert_eq!(any_input, *expect);
            }
        }
    }

    #[test]
    fn from_relative_day_of_month() {
        // input is relative to 2001-02-03T04:05:06
        for (input, expect) in &[
            // wrap month
            ("1st", Date::from_abs("2001-03-01")),
            ("2nd", Date::from_abs("2001-03-02")),
            ("3rd", Date::from_abs("2001-03-03")),
            // retain month
            ("4th", Date::from_abs("2001-02-04")),
            ("5th", Date::from_abs("2001-02-05")),
            ("6th", Date::from_abs("2001-02-06")),
            ("7th", Date::from_abs("2001-02-07")),
            ("8th", Date::from_abs("2001-02-08")),
            ("9th", Date::from_abs("2001-02-09")),
            ("10th", Date::from_abs("2001-02-10")),
            ("11th", Date::from_abs("2001-02-11")),
            ("12th", Date::from_abs("2001-02-12")),
            ("13th", Date::from_abs("2001-02-13")),
            ("14th", Date::from_abs("2001-02-14")),
            ("15th", Date::from_abs("2001-02-15")),
            ("16th", Date::from_abs("2001-02-16")),
            ("17th", Date::from_abs("2001-02-17")),
            ("18th", Date::from_abs("2001-02-18")),
            ("19th", Date::from_abs("2001-02-19")),
            ("20th", Date::from_abs("2001-02-20")),
            ("21st", Date::from_abs("2001-02-21")),
            ("22nd", Date::from_abs("2001-02-22")),
            ("23rd", Date::from_abs("2001-02-23")),
            ("24th", Date::from_abs("2001-02-24")),
            ("25th", Date::from_abs("2001-02-25")),
            ("26th", Date::from_abs("2001-02-26")),
            ("27th", Date::from_abs("2001-02-27")),
            ("28th", Date::from_abs("2001-02-28")),
            // day does not fit in current month, wrap
            ("29th", Date::from_abs("2001-03-29")),
            // ("30th", Date::from_abs("2001-03-30")),
            // ("31th", Date::from_abs("2001-03-31")),
            // unhappy path
            ("0th", None),
            ("32nd", None),
            ("x1st", None),
        ] {
            let rel_input = Date::from_rel(input, &test_now());
            assert_eq!(rel_input, *expect);
            if expect.is_some() {
                let any_input = Date::new(input, &test_now());
                assert_eq!(any_input, *expect);
            }
        }
    }

    #[test]
    fn from_relative_month() {
        for (input, expect) in &[
            // wrap year
            ("jan", Date::from_abs("2002-01")),
            ("Jan", Date::from_abs("2002-01")),
            ("january", Date::from_abs("2002-01")),
            ("January", Date::from_abs("2002-01")),
            ("feb", Date::from_abs("2002-02")),
            ("Feb", Date::from_abs("2002-02")),
            // same as current month, bump to next year
            ("february", Date::from_abs("2002-02")),
            ("February", Date::from_abs("2002-02")),
            // retain year
            ("mar", Date::from_abs("2001-03")),
            ("Mar", Date::from_abs("2001-03")),
            ("march", Date::from_abs("2001-03")),
            ("March", Date::from_abs("2001-03")),
            ("apr", Date::from_abs("2001-04")),
            ("Apr", Date::from_abs("2001-04")),
            ("april", Date::from_abs("2001-04")),
            ("April", Date::from_abs("2001-04")),
            ("may", Date::from_abs("2001-05")),
            ("May", Date::from_abs("2001-05")),
            ("jun", Date::from_abs("2001-06")),
            ("Jun", Date::from_abs("2001-06")),
            ("june", Date::from_abs("2001-06")),
            ("June", Date::from_abs("2001-06")),
            ("jul", Date::from_abs("2001-07")),
            ("Jul", Date::from_abs("2001-07")),
            ("july", Date::from_abs("2001-07")),
            ("July", Date::from_abs("2001-07")),
            ("aug", Date::from_abs("2001-08")),
            ("Aug", Date::from_abs("2001-08")),
            ("august", Date::from_abs("2001-08")),
            ("August", Date::from_abs("2001-08")),
            ("sep", Date::from_abs("2001-09")),
            ("Sep", Date::from_abs("2001-09")),
            ("september", Date::from_abs("2001-09")),
            ("September", Date::from_abs("2001-09")),
            ("oct", Date::from_abs("2001-10")),
            ("Oct", Date::from_abs("2001-10")),
            ("october", Date::from_abs("2001-10")),
            ("October", Date::from_abs("2001-10")),
            ("nov", Date::from_abs("2001-11")),
            ("Nov", Date::from_abs("2001-11")),
            ("november", Date::from_abs("2001-11")),
            ("November", Date::from_abs("2001-11")),
            ("dec", Date::from_abs("2001-12")),
            ("Dec", Date::from_abs("2001-12")),
            ("december", Date::from_abs("2001-12")),
            ("December", Date::from_abs("2001-12")),
            // unhappy path
            ("janu", None),
            ("xjan", None),
        ] {
            let rel_input = Date::from_rel(input, &test_now());
            assert_eq!(rel_input, *expect);
            if expect.is_some() {
                let any_input = Date::new(input, &test_now());
                assert_eq!(any_input, *expect);
            }
        }
    }

    #[test]
    fn from_relative_offset() {
        for (cf, input, expect) in &[
            (Date::from_abs("2001-02-03T04:05:06"), "-1s", Date::from_abs("2001-02-03T04:05:05")),
            (Date::from_abs("2001-02-03T04:05:06"), "0s", Date::from_abs("2001-02-03T04:05:06")),
            (Date::from_abs("2001-02-03T04:05:06"), "1s", Date::from_abs("2001-02-03T04:05:07")),
            (Date::from_abs("2001-02-03T04:05:06"), "59s", Date::from_abs("2001-02-03T04:06:05")),
            (Date::from_abs("2001-02-03T04:05:06"), "-1m", Date::from_abs("2001-02-03T04:04:06")),
            (Date::from_abs("2001-02-03T04:05:06"), "0m", Date::from_abs("2001-02-03T04:05:06")),
            (Date::from_abs("2001-02-03T04:05:06"), "1m", Date::from_abs("2001-02-03T04:06:06")),
            (Date::from_abs("2001-02-03T04:05:06"), "59m", Date::from_abs("2001-02-03T05:04:06")),
            (Date::from_abs("2001-02-03T04:05:06"), "-1h", Date::from_abs("2001-02-03T03:05:06")),
            (Date::from_abs("2001-02-03T04:05:06"), "0h", Date::from_abs("2001-02-03T04:05:06")),
            (Date::from_abs("2001-02-03T04:05:06"), "1h", Date::from_abs("2001-02-03T05:05:06")),
            (Date::from_abs("2001-02-03T04:05:06"), "23h", Date::from_abs("2001-02-04T03:05:06")),
            (Date::from_abs("2001-02-03T04:05:06"), "-1d", Date::from_abs("2001-02-02T04:05:06")),
            (Date::from_abs("2001-02-03T04:05:06"), "0d", Date::from_abs("2001-02-03T04:05:06")),
            (Date::from_abs("2001-02-03T04:05:06"), "1d", Date::from_abs("2001-02-04T04:05:06")),
            (Date::from_abs("2001-02-03T04:05:06"), "27d", Date::from_abs("2001-03-02T04:05:06")),
            (Date::from_abs("2001-02-03T04:05:06"), "-1w", Date::from_abs("2001-01-27T04:05:06")),
            (Date::from_abs("2001-02-03T04:05:06"), "0w", Date::from_abs("2001-02-03T04:05:06")),
            (Date::from_abs("2001-02-03T04:05:06"), "1w", Date::from_abs("2001-02-10T04:05:06")),
            (Date::from_abs("2001-02-03T04:05:06"), "52w", Date::from_abs("2002-02-02T04:05:06")),
            (Date::from_abs("2001-02-03T04:05:06"), "-1M", Date::from_abs("2001-01-03T04:05:06")),
            (Date::from_abs("2001-02-03T04:05:06"), "0M", Date::from_abs("2001-02-03T04:05:06")),
            (Date::from_abs("2001-02-03T04:05:06"), "1M", Date::from_abs("2001-03-03T04:05:06")),
            (Date::from_abs("2001-02-03T04:05:06"), "11M", Date::from_abs("2002-01-03T04:05:06")),
            (Date::from_abs("2001-02-03T04:05:06"), "-1y", Date::from_abs("2000-02-03T04:05:06")),
            (Date::from_abs("2001-02-03T04:05:06"), "0y", Date::from_abs("2001-02-03T04:05:06")),
            (Date::from_abs("2001-02-03T04:05:06"), "1y", Date::from_abs("2002-02-03T04:05:06")),
            (Date::from_abs("2001-02-03T04:05:06"), "10y", Date::from_abs("2011-02-03T04:05:06")),
            // Relative weekday from a Sunday
            (Date::from_abs("2001-02-04T04:05:06"), "-11W", Date::from_abs("2001-01-19T04:05:06")),
            (Date::from_abs("2001-02-04T04:05:06"), "-10W", Date::from_abs("2001-01-22T04:05:06")),
            (Date::from_abs("2001-02-04T04:05:06"), "-6W", Date::from_abs("2001-01-26T04:05:06")),
            (Date::from_abs("2001-02-04T04:05:06"), "-5W", Date::from_abs("2001-01-29T04:05:06")),
            (Date::from_abs("2001-02-04T04:05:06"), "-4W", Date::from_abs("2001-01-30T04:05:06")),
            (Date::from_abs("2001-02-04T04:05:06"), "-3W", Date::from_abs("2001-01-31T04:05:06")),
            (Date::from_abs("2001-02-04T04:05:06"), "-2W", Date::from_abs("2001-02-01T04:05:06")),
            (Date::from_abs("2001-02-04T04:05:06"), "-1W", Date::from_abs("2001-02-02T04:05:06")),
            (Date::from_abs("2001-02-04T04:05:06"), "0W", Date::from_abs("2001-02-04T04:05:06")),
            (Date::from_abs("2001-02-04T04:05:06"), "1W", Date::from_abs("2001-02-05T04:05:06")),
            (Date::from_abs("2001-02-04T04:05:06"), "2W", Date::from_abs("2001-02-06T04:05:06")),
            (Date::from_abs("2001-02-04T04:05:06"), "3W", Date::from_abs("2001-02-07T04:05:06")),
            (Date::from_abs("2001-02-04T04:05:06"), "4W", Date::from_abs("2001-02-08T04:05:06")),
            (Date::from_abs("2001-02-04T04:05:06"), "5W", Date::from_abs("2001-02-09T04:05:06")),
            (Date::from_abs("2001-02-04T04:05:06"), "6W", Date::from_abs("2001-02-12T04:05:06")),
            (Date::from_abs("2001-02-04T04:05:06"), "10W", Date::from_abs("2001-02-16T04:05:06")),
            (Date::from_abs("2001-02-04T04:05:06"), "11W", Date::from_abs("2001-02-19T04:05:06")),
            // Relative weekday from a Monday
            (Date::from_abs("2001-02-05T04:05:06"), "-11W", Date::from_abs("2001-01-19T04:05:06")),
            (Date::from_abs("2001-02-05T04:05:06"), "-10W", Date::from_abs("2001-01-22T04:05:06")),
            (Date::from_abs("2001-02-05T04:05:06"), "-6W", Date::from_abs("2001-01-26T04:05:06")),
            (Date::from_abs("2001-02-05T04:05:06"), "-5W", Date::from_abs("2001-01-29T04:05:06")),
            (Date::from_abs("2001-02-05T04:05:06"), "-4W", Date::from_abs("2001-01-30T04:05:06")),
            (Date::from_abs("2001-02-05T04:05:06"), "-3W", Date::from_abs("2001-01-31T04:05:06")),
            (Date::from_abs("2001-02-05T04:05:06"), "-2W", Date::from_abs("2001-02-01T04:05:06")),
            (Date::from_abs("2001-02-05T04:05:06"), "-1W", Date::from_abs("2001-02-02T04:05:06")),
            (Date::from_abs("2001-02-05T04:05:06"), "0W", Date::from_abs("2001-02-05T04:05:06")),
            (Date::from_abs("2001-02-05T04:05:06"), "1W", Date::from_abs("2001-02-06T04:05:06")),
            (Date::from_abs("2001-02-05T04:05:06"), "2W", Date::from_abs("2001-02-07T04:05:06")),
            (Date::from_abs("2001-02-05T04:05:06"), "3W", Date::from_abs("2001-02-08T04:05:06")),
            (Date::from_abs("2001-02-05T04:05:06"), "4W", Date::from_abs("2001-02-09T04:05:06")),
            (Date::from_abs("2001-02-05T04:05:06"), "5W", Date::from_abs("2001-02-12T04:05:06")),
            (Date::from_abs("2001-02-05T04:05:06"), "6W", Date::from_abs("2001-02-13T04:05:06")),
            (Date::from_abs("2001-02-05T04:05:06"), "10W", Date::from_abs("2001-02-19T04:05:06")),
            (Date::from_abs("2001-02-05T04:05:06"), "11W", Date::from_abs("2001-02-20T04:05:06")),
            // Relative weekday from a Tuesday
            (Date::from_abs("2001-02-06T04:05:06"), "-11W", Date::from_abs("2001-01-22T04:05:06")),
            (Date::from_abs("2001-02-06T04:05:06"), "-10W", Date::from_abs("2001-01-23T04:05:06")),
            (Date::from_abs("2001-02-06T04:05:06"), "-6W", Date::from_abs("2001-01-29T04:05:06")),
            (Date::from_abs("2001-02-06T04:05:06"), "-5W", Date::from_abs("2001-01-30T04:05:06")),
            (Date::from_abs("2001-02-06T04:05:06"), "-4W", Date::from_abs("2001-01-31T04:05:06")),
            (Date::from_abs("2001-02-06T04:05:06"), "-3W", Date::from_abs("2001-02-01T04:05:06")),
            (Date::from_abs("2001-02-06T04:05:06"), "-2W", Date::from_abs("2001-02-02T04:05:06")),
            (Date::from_abs("2001-02-06T04:05:06"), "-1W", Date::from_abs("2001-02-05T04:05:06")),
            (Date::from_abs("2001-02-06T04:05:06"), "0W", Date::from_abs("2001-02-06T04:05:06")),
            (Date::from_abs("2001-02-06T04:05:06"), "1W", Date::from_abs("2001-02-07T04:05:06")),
            (Date::from_abs("2001-02-06T04:05:06"), "2W", Date::from_abs("2001-02-08T04:05:06")),
            (Date::from_abs("2001-02-06T04:05:06"), "3W", Date::from_abs("2001-02-09T04:05:06")),
            (Date::from_abs("2001-02-06T04:05:06"), "4W", Date::from_abs("2001-02-12T04:05:06")),
            (Date::from_abs("2001-02-06T04:05:06"), "5W", Date::from_abs("2001-02-13T04:05:06")),
            (Date::from_abs("2001-02-06T04:05:06"), "6W", Date::from_abs("2001-02-14T04:05:06")),
            (Date::from_abs("2001-02-06T04:05:06"), "10W", Date::from_abs("2001-02-20T04:05:06")),
            (Date::from_abs("2001-02-06T04:05:06"), "11W", Date::from_abs("2001-02-21T04:05:06")),
            // Relative weekday from a Wednesday
            (Date::from_abs("2001-02-07T04:05:06"), "-11W", Date::from_abs("2001-01-23T04:05:06")),
            (Date::from_abs("2001-02-07T04:05:06"), "-10W", Date::from_abs("2001-01-24T04:05:06")),
            (Date::from_abs("2001-02-07T04:05:06"), "-6W", Date::from_abs("2001-01-30T04:05:06")),
            (Date::from_abs("2001-02-07T04:05:06"), "-5W", Date::from_abs("2001-01-31T04:05:06")),
            (Date::from_abs("2001-02-07T04:05:06"), "-4W", Date::from_abs("2001-02-01T04:05:06")),
            (Date::from_abs("2001-02-07T04:05:06"), "-3W", Date::from_abs("2001-02-02T04:05:06")),
            (Date::from_abs("2001-02-07T04:05:06"), "-2W", Date::from_abs("2001-02-05T04:05:06")),
            (Date::from_abs("2001-02-07T04:05:06"), "-1W", Date::from_abs("2001-02-06T04:05:06")),
            (Date::from_abs("2001-02-07T04:05:06"), "0W", Date::from_abs("2001-02-07T04:05:06")),
            (Date::from_abs("2001-02-07T04:05:06"), "1W", Date::from_abs("2001-02-08T04:05:06")),
            (Date::from_abs("2001-02-07T04:05:06"), "2W", Date::from_abs("2001-02-09T04:05:06")),
            (Date::from_abs("2001-02-07T04:05:06"), "3W", Date::from_abs("2001-02-12T04:05:06")),
            (Date::from_abs("2001-02-07T04:05:06"), "4W", Date::from_abs("2001-02-13T04:05:06")),
            (Date::from_abs("2001-02-07T04:05:06"), "5W", Date::from_abs("2001-02-14T04:05:06")),
            (Date::from_abs("2001-02-07T04:05:06"), "6W", Date::from_abs("2001-02-15T04:05:06")),
            (Date::from_abs("2001-02-07T04:05:06"), "10W", Date::from_abs("2001-02-21T04:05:06")),
            (Date::from_abs("2001-02-07T04:05:06"), "11W", Date::from_abs("2001-02-22T04:05:06")),
            // Relative weekday from a Thursday
            (Date::from_abs("2001-02-08T04:05:06"), "-11W", Date::from_abs("2001-01-24T04:05:06")),
            (Date::from_abs("2001-02-08T04:05:06"), "-10W", Date::from_abs("2001-01-25T04:05:06")),
            (Date::from_abs("2001-02-08T04:05:06"), "-6W", Date::from_abs("2001-01-31T04:05:06")),
            (Date::from_abs("2001-02-08T04:05:06"), "-5W", Date::from_abs("2001-02-01T04:05:06")),
            (Date::from_abs("2001-02-08T04:05:06"), "-4W", Date::from_abs("2001-02-02T04:05:06")),
            (Date::from_abs("2001-02-08T04:05:06"), "-3W", Date::from_abs("2001-02-05T04:05:06")),
            (Date::from_abs("2001-02-08T04:05:06"), "-2W", Date::from_abs("2001-02-06T04:05:06")),
            (Date::from_abs("2001-02-08T04:05:06"), "-1W", Date::from_abs("2001-02-07T04:05:06")),
            (Date::from_abs("2001-02-08T04:05:06"), "0W", Date::from_abs("2001-02-08T04:05:06")),
            (Date::from_abs("2001-02-08T04:05:06"), "1W", Date::from_abs("2001-02-09T04:05:06")),
            (Date::from_abs("2001-02-08T04:05:06"), "2W", Date::from_abs("2001-02-12T04:05:06")),
            (Date::from_abs("2001-02-08T04:05:06"), "3W", Date::from_abs("2001-02-13T04:05:06")),
            (Date::from_abs("2001-02-08T04:05:06"), "4W", Date::from_abs("2001-02-14T04:05:06")),
            (Date::from_abs("2001-02-08T04:05:06"), "5W", Date::from_abs("2001-02-15T04:05:06")),
            (Date::from_abs("2001-02-08T04:05:06"), "6W", Date::from_abs("2001-02-16T04:05:06")),
            (Date::from_abs("2001-02-08T04:05:06"), "10W", Date::from_abs("2001-02-22T04:05:06")),
            (Date::from_abs("2001-02-08T04:05:06"), "11W", Date::from_abs("2001-02-23T04:05:06")),
            // Relative weekday from a Friday
            (Date::from_abs("2001-02-09T04:05:06"), "-11W", Date::from_abs("2001-01-25T04:05:06")),
            (Date::from_abs("2001-02-09T04:05:06"), "-10W", Date::from_abs("2001-01-26T04:05:06")),
            (Date::from_abs("2001-02-09T04:05:06"), "-6W", Date::from_abs("2001-02-01T04:05:06")),
            (Date::from_abs("2001-02-09T04:05:06"), "-5W", Date::from_abs("2001-02-02T04:05:06")),
            (Date::from_abs("2001-02-09T04:05:06"), "-4W", Date::from_abs("2001-02-05T04:05:06")),
            (Date::from_abs("2001-02-09T04:05:06"), "-3W", Date::from_abs("2001-02-06T04:05:06")),
            (Date::from_abs("2001-02-09T04:05:06"), "-2W", Date::from_abs("2001-02-07T04:05:06")),
            (Date::from_abs("2001-02-09T04:05:06"), "-1W", Date::from_abs("2001-02-08T04:05:06")),
            (Date::from_abs("2001-02-09T04:05:06"), "0W", Date::from_abs("2001-02-09T04:05:06")),
            (Date::from_abs("2001-02-09T04:05:06"), "1W", Date::from_abs("2001-02-12T04:05:06")),
            (Date::from_abs("2001-02-09T04:05:06"), "2W", Date::from_abs("2001-02-13T04:05:06")),
            (Date::from_abs("2001-02-09T04:05:06"), "3W", Date::from_abs("2001-02-14T04:05:06")),
            (Date::from_abs("2001-02-09T04:05:06"), "4W", Date::from_abs("2001-02-15T04:05:06")),
            (Date::from_abs("2001-02-09T04:05:06"), "5W", Date::from_abs("2001-02-16T04:05:06")),
            (Date::from_abs("2001-02-09T04:05:06"), "6W", Date::from_abs("2001-02-19T04:05:06")),
            (Date::from_abs("2001-02-09T04:05:06"), "10W", Date::from_abs("2001-02-23T04:05:06")),
            (Date::from_abs("2001-02-09T04:05:06"), "11W", Date::from_abs("2001-02-26T04:05:06")),
            // Relative weekday from a Saturday
            (Date::from_abs("2001-02-10T04:05:06"), "-11W", Date::from_abs("2001-01-26T04:05:06")),
            (Date::from_abs("2001-02-10T04:05:06"), "-10W", Date::from_abs("2001-01-29T04:05:06")),
            (Date::from_abs("2001-02-10T04:05:06"), "-6W", Date::from_abs("2001-02-02T04:05:06")),
            (Date::from_abs("2001-02-10T04:05:06"), "-5W", Date::from_abs("2001-02-05T04:05:06")),
            (Date::from_abs("2001-02-10T04:05:06"), "-4W", Date::from_abs("2001-02-06T04:05:06")),
            (Date::from_abs("2001-02-10T04:05:06"), "-3W", Date::from_abs("2001-02-07T04:05:06")),
            (Date::from_abs("2001-02-10T04:05:06"), "-2W", Date::from_abs("2001-02-08T04:05:06")),
            (Date::from_abs("2001-02-10T04:05:06"), "-1W", Date::from_abs("2001-02-09T04:05:06")),
            (Date::from_abs("2001-02-10T04:05:06"), "0W", Date::from_abs("2001-02-10T04:05:06")),
            (Date::from_abs("2001-02-10T04:05:06"), "1W", Date::from_abs("2001-02-12T04:05:06")),
            (Date::from_abs("2001-02-10T04:05:06"), "2W", Date::from_abs("2001-02-13T04:05:06")),
            (Date::from_abs("2001-02-10T04:05:06"), "3W", Date::from_abs("2001-02-14T04:05:06")),
            (Date::from_abs("2001-02-10T04:05:06"), "4W", Date::from_abs("2001-02-15T04:05:06")),
            (Date::from_abs("2001-02-10T04:05:06"), "5W", Date::from_abs("2001-02-16T04:05:06")),
            (Date::from_abs("2001-02-10T04:05:06"), "6W", Date::from_abs("2001-02-19T04:05:06")),
            (Date::from_abs("2001-02-10T04:05:06"), "10W", Date::from_abs("2001-02-23T04:05:06")),
            (Date::from_abs("2001-02-10T04:05:06"), "11W", Date::from_abs("2001-02-26T04:05:06")),
            // unhappy path
            (Date::from_abs("2001-02-03T04:05:06"), "1x", None),
        ] {
            let rel_input = Date::from_rel(input, cf.as_ref().unwrap());
            assert_eq!(rel_input, *expect);
            if expect.is_some() {
                let any_input = Date::new(input, cf.as_ref().unwrap());
                assert_eq!(any_input, *expect);
            }
        }
    }

    #[test]
    fn from_relative_named() {
        for (input, expect) in &[
            ("today", Date::from_abs("2001-02-03")),
            ("tomorrow", Date::from_abs("2001-02-04")),
            ("tom", Date::from_abs("2001-02-04")),
            ("yesterday", Date::from_abs("2001-02-02")),
            ("yes", Date::from_abs("2001-02-02")),
            ("now", Date::from_abs("2001-02-03T04:05:06")),
            // unhappy path
            ("then", None),
        ] {
            let rel_input = Date::from_rel(input, &test_now());
            assert_eq!(rel_input, *expect);
            if expect.is_some() {
                let any_input = Date::new(input, &test_now());
                assert_eq!(any_input, *expect);
            }
        }
    }

    #[test]
    fn add() {
        for (start, dur, expect) in &[
            ("2001-02-03T04:05:06", Seconds(-59), "2001-02-03T04:04:07"),
            ("2001-02-03T04:05:06", Seconds(-1), "2001-02-03T04:05:05"),
            ("2001-02-03T04:05:06", Seconds(0), "2001-02-03T04:05:06"),
            ("2001-02-03T04:05:06", Seconds(1), "2001-02-03T04:05:07"),
            ("2001-02-03T04:05:06", Seconds(59), "2001-02-03T04:06:05"),
            ("2001-02-03T04:05:06", Seconds(119), "2001-02-03T04:07:05"),
            ("2001-02-03T04:05:06", Minutes(-59), "2001-02-03T03:06:06"),
            ("2001-02-03T04:05:06", Minutes(-1), "2001-02-03T04:04:06"),
            ("2001-02-03T04:05:06", Minutes(0), "2001-02-03T04:05:06"),
            ("2001-02-03T04:05:06", Minutes(1), "2001-02-03T04:06:06"),
            ("2001-02-03T04:05:06", Minutes(59), "2001-02-03T05:04:06"),
            ("2001-02-03T04:05:06", Minutes(119), "2001-02-03T06:04:06"),
            ("2001-02-03T04:05:06", Hours(-23), "2001-02-02T05:05:06"),
            ("2001-02-03T04:05:06", Hours(-1), "2001-02-03T03:05:06"),
            ("2001-02-03T04:05:06", Hours(0), "2001-02-03T04:05:06"),
            ("2001-02-03T04:05:06", Hours(1), "2001-02-03T05:05:06"),
            ("2001-02-03T04:05:06", Hours(23), "2001-02-04T03:05:06"),
            ("2001-02-03T04:05:06", Hours(47), "2001-02-05T03:05:06"),
            ("2001-02-03T04:05:06", Days(-30), "2001-01-04T04:05:06"),
            ("2001-02-03T04:05:06", Days(-1), "2001-02-02T04:05:06"),
            ("2001-02-03T04:05:06", Days(0), "2001-02-03T04:05:06"),
            ("2001-02-03T04:05:06", Days(1), "2001-02-04T04:05:06"),
            ("2001-02-03T04:05:06", Days(27), "2001-03-02T04:05:06"),
            ("2001-02-03T04:05:06", Days(58), "2001-04-02T04:05:06"),
            ("2001-02-03T04:05:06", Days(7), "2001-02-10T04:05:06"),
            ("2001-02-03T04:05:06", Months(-11), "2000-03-03T04:05:06"),
            ("2001-02-03T04:05:06", Months(-1), "2001-01-03T04:05:06"),
            ("2001-02-03T04:05:06", Months(0), "2001-02-03T04:05:06"),
            ("2001-02-03T04:05:06", Months(1), "2001-03-03T04:05:06"),
            ("2001-02-03T04:05:06", Months(11), "2002-01-03T04:05:06"),
            ("2001-02-03T04:05:06", Months(23), "2003-01-03T04:05:06"),
            ("2000-01-01T00:00:00", Months(24), "2002-01-01T00:00:00"),
            ("2000-02-29T00:00:00", Months(12), "2001-02-28T00:00:00"),
            ("2001-02-03T04:05:06", Years(-1), "2000-02-03T04:05:06"),
            ("2000-02-29T04:05:06", Years(-1), "1999-02-28T04:05:06"),
            ("2001-02-03T04:05:06", Years(0), "2001-02-03T04:05:06"),
            ("2000-02-29T04:05:06", Years(1), "2001-02-28T04:05:06"),
            ("2000-02-29T04:05:06", Years(2), "2002-02-28T04:05:06"),
            ("2001-02-03T04:05:06", Years(20), "2021-02-03T04:05:06"),
        ] {
            let start = Date::from_abs(start).unwrap();
            let expect = Date::from_abs(expect).unwrap();
            assert_eq!(&start + dur, expect);
        }
    }

    #[test]
    fn on_or_within() {
        for (inside, outside, expect) in &[
            ("2001-02-03T04:05:06", "2001-02-03T04:05:06", true),
            ("2001-02-03T04:05:06", "2001-02-03T04:05:07", false),
            ("2001-02-03T04:05:06", "2001-02-03T04:05:05", false),
            ("2001-02-03T04:05:00", "2001-02-03T04:05", true),
            ("2001-02-03T04:05:30", "2001-02-03T04:05", true),
            ("2001-02-03T04:05", "2001-02-03T04:05:00", false),
            ("2001-02-03T04:05:00", "2001-02-03T04", true),
            ("2001-02-03T04:05:30", "2001-02-03T04", true),
            ("2001-02-03T04", "2001-02-03T04:05:00", false),
            ("2001-02-03T04:05", "2001-02-03T04", true),
            ("2001-02-03T04", "2001-02-03T04:05", false),
            ("2001-02-03T04:05", "2001-02-03", true),
            ("2001-02-03", "2001-02-03T04:05", false),
            ("2001-02-03", "2001-02", true),
            ("2001-02", "2001-02-03", false),
            ("2001-02-03", "2001", true),
            ("2001", "2001-02-03", false),
        ] {
            let inside = Date::from_abs(inside).unwrap();
            let outside = Date::from_abs(outside).unwrap();
            assert_eq!(inside.within(&outside), *expect);
        }
    }

    #[test]
    fn before() {
        for (before, after, expect) in &[
            ("2001-02-03T04:05:06", "2001-02-03T04:05:07", true),
            ("2001-02-03T04:05:06", "2001-02-03T04:05:06", false),
            ("2001-02-03T04:05:06", "2001-02-03T04:05:05", false),
            ("2001-02-03T04:05:06", "2001-02-03T04:06:06", true),
            ("2001-02-03T04:06:06", "2001-02-03T04:05:05", false),
            ("2001-02-03T04:05:06", "2001-02-03T05:05:06", true),
            ("2001-02-03T04:06:06", "2001-02-03T03:05:06", false),
            ("2001-02-03T04:05:06", "2001-02-04T04:05:06", true),
            ("2001-02-03T04:06:06", "2001-02-02T04:05:06", false),
            ("2001-02-03T04:05:06", "2001-03-03T04:05:06", true),
            ("2001-02-03T04:06:06", "2001-01-03T04:05:06", false),
            ("2001-02-03T04:05:06", "2002-02-03T04:05:06", true),
            ("2001-02-03T04:06:06", "2000-02-03T04:05:06", false),
            ("2001-02-03T04:05", "2001-02-03T04:06", true),
            ("2001-02-03T04:05", "2001-02-03T04:05", false),
            ("2001-02-03T04:05", "2001-02-03T04:04", false),
            ("2001-02-03T04:05", "2001-02-03T05:05", true),
            ("2001-02-03T04:05", "2001-02-03T03:04", false),
            ("2001-02-03T04:05", "2001-02-04T04:05", true),
            ("2001-02-03T04:05", "2001-02-02T04:04", false),
            ("2001-02-03T04:05", "2001-03-03T04:05", true),
            ("2001-02-03T04:05", "2001-01-03T04:05", false),
            ("2001-02-03T04:05", "2002-02-03T04:05", true),
            ("2001-02-03T04:05", "2000-02-03T04:05", false),
            ("2001-02-03", "2001-02-04", true),
            ("2001-02-03", "2001-02-03", false),
            ("2001-02-03", "2001-02-02", false),
            ("2001-02-03T04:05:06", "2001-02-03T04:06", true),
            ("2001-02-03T04:05", "2001-02-03T04:05:06", false),
            ("2001-02-03T04:05:06", "2001-02-03T04:05", false),
            ("2001-02-03T04:05", "2001-02-03T05", true),
            ("2001-02-03T04", "2001-02-03T04:05:06", false),
            ("2001-02-03T04:05", "2001-02-03T04", false),
            ("2001-02-03T04", "2001-02-04", true),
            ("2001-02-03", "2001-02-03T04:05", false),
            ("2001-02-03T04", "2001-02-03", false),
        ] {
            let before = Date::from_abs(before).unwrap();
            let after = Date::from_abs(after).unwrap();
            assert_eq!(before.before(&after), *expect);
        }
    }

    #[test]
    fn after() {
        for (after, before, expect) in &[
            ("2001-02-03T04:05:07", "2001-02-03T04:05:06", true),
            ("2001-02-03T04:05:06", "2001-02-03T04:05:06", false),
            ("2001-02-03T04:05:05", "2001-02-03T04:05:06", false),
            ("2001-02-03T04:06:06", "2001-02-03T04:05:06", true),
            ("2001-02-03T04:05:05", "2001-02-03T04:06:06", false),
            ("2001-02-03T05:05:06", "2001-02-03T04:05:06", true),
            ("2001-02-03T03:05:06", "2001-02-03T04:06:06", false),
            ("2001-02-04T04:05:06", "2001-02-03T04:05:06", true),
            ("2001-02-02T04:05:06", "2001-02-03T04:06:06", false),
            ("2001-03-03T04:05:06", "2001-02-03T04:05:06", true),
            ("2001-01-03T04:05:06", "2001-02-03T04:06:06", false),
            ("2002-02-03T04:05:06", "2001-02-03T04:05:06", true),
            ("2000-02-03T04:05:06", "2001-02-03T04:06:06", false),
            ("2001-02-03T04:06", "2001-02-03T04:05", true),
            ("2001-02-03T04:05", "2001-02-03T04:05", false),
            ("2001-02-03T04:04", "2001-02-03T04:05", false),
            ("2001-02-03T05:05", "2001-02-03T04:05", true),
            ("2001-02-03T03:04", "2001-02-03T04:05", false),
            ("2001-02-04T04:05", "2001-02-03T04:05", true),
            ("2001-02-02T04:04", "2001-02-03T04:05", false),
            ("2001-03-03T04:05", "2001-02-03T04:05", true),
            ("2001-01-03T04:05", "2001-02-03T04:05", false),
            ("2002-02-03T04:05", "2001-02-03T04:05", true),
            ("2000-02-03T04:05", "2001-02-03T04:05", false),
            ("2001-02-04", "2001-02-03", true),
            ("2001-02-03", "2001-02-03", false),
            ("2001-02-02", "2001-02-03", false),
            ("2001-02-03T04:06", "2001-02-03T04:05:06", true),
            ("2001-02-03T04:05:06", "2001-02-03T04:05", false),
            ("2001-02-03T04:05", "2001-02-03T04:05:06", false),
            ("2001-02-03T05", "2001-02-03T04:05", true),
            ("2001-02-03T04:05:06", "2001-02-03T04", false),
            ("2001-02-03T04", "2001-02-03T04:05", false),
            ("2001-02-04", "2001-02-03T04", true),
            ("2001-02-03T04:05", "2001-02-03", false),
            ("2001-02-03", "2001-02-03T04", false),
        ] {
            let after = Date::from_abs(after).unwrap();
            let before = Date::from_abs(before).unwrap();
            assert_eq!(after.after(&before), *expect);
        }
    }

    #[test]
    fn fmt() {
        for (input, expect) in &[
            (
                Date::from_fields(2001, 2, 3, 4, 5, 6, Duration::Seconds(1)).unwrap(),
                "2001-02-03T04:05:06",
            ),
            (
                Date::from_fields(2001, 2, 3, 4, 5, 0, Duration::Minutes(1)).unwrap(),
                "2001-02-03T04:05",
            ),
            (
                // confirm fields below duration are ignored
                Date::from_fields(2001, 2, 3, 4, 5, 6, Duration::Minutes(1)).unwrap(),
                "2001-02-03T04:05",
            ),
            (
                Date::from_fields(2001, 2, 3, 4, 0, 0, Duration::Hours(1)).unwrap(),
                "2001-02-03T04",
            ),
            (
                Date::from_fields(2001, 2, 3, 0, 0, 0, Duration::Days(1)).unwrap(),
                "2001-02-03",
            ),
            (
                Date::from_fields(2001, 2, 3, 0, 0, 0, Duration::Months(1)).unwrap(),
                "2001-02",
            ),
            (
                Date::from_fields(2001, 2, 3, 0, 0, 0, Duration::Years(1)).unwrap(),
                "2001",
            ),
        ] {
            assert_eq!(input.to_string(), *expect);
        }
    }
}
