use crate::time::calendar_trait::CalendarTrait;
use crate::utils::string_arithmetic::{
    from_month_to_i32,
    from_i32_to_month,
};
//
use serde::{Deserialize, Serialize};
use anyhow::Result;
use time::{Date, OffsetDateTime};
pub type Tenor = Period;

/// Represents a time period with years, months, and days.
///
/// # Example
///
/// ```
/// use rustmetrics::Period;
/// use time::macros::datetime;
///
/// let period = Period::new(1, 2, 3); // 1 year, 2 months, 3 days
/// let datetime = datetime!(2023-12-31 00:00:00 +09:00);
/// let new_datetime = period.apply(&datetime);
/// assert_eq!(new_datetime.to_string(), "2025-03-03 00:00:00.0 +09:00");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Period {
    years: i32,
    months: i32,
    days: i32,
}

impl std::fmt::Display for Period {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}Y{}M{}D", self.years, self.months, self.days)
    }
}

impl Period {
     /// Creates a new Period with the specified years, months, and days.
    pub fn new(years: i32, months: i32, days: i32) -> Period {
        Period {
            years,
            months,
            days,
        }
    }

     /// Creates a new Period from a string representation.
    ///
    /// # Example
    ///
    /// ```
    /// use rustmetrics::Period;
    ///
    /// let period = Period::new_from_string("1Y2M3D").unwrap();
    /// assert_eq!(period.years(), 1);
    /// assert_eq!(period.months(), 2);
    /// assert_eq!(period.days(), 3);
    /// ```
    pub fn new_from_string(tenor: &str) -> Result<Period> {
        let mut years = 0;
        let mut months = 0;
        let mut days = 0;
        //let mut hours = 0;

        let mut num = 0;
        for c in tenor.chars() {
            if c.is_ascii_digit() {
                num = num * 10 + c.to_digit(10).unwrap() as i32;
            } else {
                match c {
                    'Y' => years = num,
                    'M' => months = num,
                    'W' => days = num * 7,
                    'D' => days = num,
                    //'h' => hours = num,
                    _ => {
                        let err = || anyhow::anyhow!("Invalid tenor string: {}", tenor);
                        return Err(err());
                    }
                }
                num = 0;
            }
        }

        Ok(Period {
            years,
            months,
            days,
            //hours,
        })
    }

    #[inline]
    #[must_use]
    pub fn years(&self) -> i32 {
        self.years
    }

    #[inline]
    #[must_use]
    pub fn months(&self) -> i32 {
        self.months
    }

    #[inline]
    #[must_use]
    pub fn days(&self) -> i32 {
        self.days
    }

     // Getter methods...

    /// Applies the year component of the Period to the given datetime.
    ///
    /// # Example
    ///
    /// ```
    /// use rustmetrics::Period;
    /// use time::macros::datetime;
    ///
    /// let period = Period::new(2, 0, 0);
    /// let datetime = datetime!(2023-12-31 00:00:00 +09:00);
    /// let new_datetime = period.apply_year(&datetime);
    /// assert_eq!(new_datetime.year(), 2025);
    /// ```
    pub fn apply_year(&self, datetime: &OffsetDateTime) -> OffsetDateTime {
        if self.years == 0 {
            *datetime
        } else {
            let mut new_datetime = *datetime;
            let new_year = new_datetime.year() + self.years;
            let new_month = new_datetime.month();
            let eom_new = crate::NULL_CALENDAR
                .last_day_of_month(new_year, new_month)
                .day();
            let new_day = match new_datetime.day() > eom_new {
                true => eom_new,
                false => new_datetime.day(),
            };
            new_datetime = OffsetDateTime::new_in_offset(
                Date::from_calendar_date(new_year, new_month, new_day)
                    .expect("Failed to create Date"),
                datetime.time(),
                datetime.offset(),
            );

            new_datetime
        } 
    }

    /// Applies the month component of the Period to the given datetime.
    ///
    /// # Example
    ///
    /// ```
    /// use rustmetrics::Period;
    /// use time::macros::datetime;
    ///
    /// let period = Period::new(0, 14, 0);
    /// let datetime = datetime!(2023-12-31 00:00:00 +09:00);
    /// let new_datetime = period.apply_month(&datetime);
    /// assert_eq!(new_datetime.to_string(), "2025-02-28 00:00:00.0 +09:00");
    /// ```
    pub fn apply_month(&self, datetime: &OffsetDateTime) -> OffsetDateTime {
        if self.months == 0 {
            *datetime
        } else {
            let mut new_datetime = *datetime;
            let mut new_year = new_datetime.year();
            let month_i32 = from_month_to_i32(new_datetime.month());

            new_year += (month_i32 + self.months) / 12;
            let mut new_month_i32 = (month_i32 + self.months) % 12;
            new_month_i32 = if new_month_i32 < 0 { new_month_i32 + 12 } else { new_month_i32 };
            let new_month = from_i32_to_month(new_month_i32);
            let eom_new = crate::NULL_CALENDAR
                .last_day_of_month(new_year, new_month)
                .day();

            let new_day = match new_datetime.day() > eom_new {
                true => eom_new,
                false => new_datetime.day(),
            };
            new_datetime = OffsetDateTime::new_in_offset(
                Date::from_calendar_date(new_year, new_month, new_day)
                    .expect("Failed to create Date"),
                datetime.time(),
                datetime.offset(),
            );

            new_datetime
        }
    }

    /// Applies the day component of the Period to the given datetime.
    ///
    /// # Example
    ///
    /// ```
    /// use rustmetrics::Period;
    /// use time::macros::datetime;
    ///
    /// let period = Period::new(0, 0, 35);
    /// let datetime = datetime!(2023-12-31 00:00:00 +09:00);
    /// let new_datetime = period.apply_day(&datetime);
    /// assert_eq!(new_datetime.to_string(), "2024-02-04 00:00:00.0 +09:00");
    /// ```
    pub fn apply_day(&self, datetime: &OffsetDateTime) -> OffsetDateTime {
        if self.days == 0 {
            *datetime
        } else {
            let mut new_datetime = *datetime;
            let new_date = new_datetime.date() + time::Duration::days(self.days as i64);
            new_datetime = OffsetDateTime::new_in_offset(
                new_date,
                datetime.time(),
                datetime.offset(),
            );

            new_datetime
        }
    }

    /// Applies the entire Period to the given datetime.
    ///
    /// # Example
    ///
    /// ```
    /// use rustmetrics::Period;
    /// use time::macros::datetime;
    ///
    /// let period = Period::new(1, 1, 1);
    /// let datetime = datetime!(2023-12-31 00:00:00 +09:00);
    /// let new_datetime = period.apply(&datetime);
    /// assert_eq!(new_datetime.to_string(), "2025-02-01 00:00:00.0 +09:00");
    /// ```
    pub fn apply(&self, datetime: &OffsetDateTime) -> OffsetDateTime {
        let mut new_datetime = self.apply_year(datetime);
        new_datetime = self.apply_month(&new_datetime);
        new_datetime = self.apply_day(&new_datetime);
        new_datetime
    }
}


/// Represents a more granular time period, including hours, minutes, milliseconds, and nanoseconds.
///
/// # Example
///
/// ```
/// use rustmetrics::{Period, FinerPeriod};
/// use time::macros::datetime;
///
/// let period = Period::new(1, 2, 3);
/// let finer_period = FinerPeriod::new(period, 4, 5, 6, 7);
/// let datetime = datetime!(2023-12-31 00:00:00 +09:00);
/// let new_datetime = finer_period.apply(&datetime);
/// assert_eq!(new_datetime.to_string(), "2025-03-03 04:05:00.006000007 +09:00");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct FinerPeriod {
    period: Period,
    hours: i32,
    minutes: i32,
    milli_seconds: i32,
    nano_seconds: i32,
}

impl FinerPeriod {
    pub fn new(period: Period, hours: i32, minutes: i32, milli_seconds: i32, nano_seconds: i32) -> FinerPeriod {
        FinerPeriod {
            period,
            hours,
            minutes,
            milli_seconds,
            nano_seconds,
        }
    }

    /// Applies the FinerPeriod to the given datetime.
    pub fn apply(&self, datetime: &OffsetDateTime) -> OffsetDateTime {
        let mut new_datetime = self.period.apply(datetime);
        new_datetime += time::Duration::hours(self.hours as i64);
        new_datetime += time::Duration::minutes(self.minutes as i64);
        new_datetime += time::Duration::milliseconds(self.milli_seconds as i64);
        new_datetime += time::Duration::nanoseconds(self.nano_seconds as i64);

        new_datetime
    }

    #[inline]
    #[must_use]
    pub fn period(&self) -> &Period {
        &self.period
    }

    #[inline]
    #[must_use]
    pub fn hours(&self) -> i32 {
        self.hours
    }

    #[inline]
    #[must_use]
    pub fn minutes(&self) -> i32 {
        self.minutes
    }

    #[inline]
    #[must_use]
    pub fn seconds(&self) -> i32 {
        self.milli_seconds / 1000
    }

    #[inline]
    #[must_use]
    pub fn milli_seconds(&self) -> i32 {
        self.milli_seconds
    }

    #[inline]
    #[must_use]
    pub fn micro_seconds(&self) -> i32 {
        self.nano_seconds / 1000
    }
    
    #[inline]
    #[must_use]
    pub fn nano_seconds(&self) -> i32 {
        self.nano_seconds
    }

        // Getter methods...

    /// Creates a new FinerPeriod from a string representation.
    ///
    /// # Example
    ///
    /// ```
    /// use rustmetrics::FinerPeriod;
    ///
    /// let finer_period = FinerPeriod::new_from_string("1Y2M3D4h5m6s7l8u9n").unwrap();
    /// assert_eq!(finer_period.period().years(), 1);
    /// assert_eq!(finer_period.period().months(), 2);
    /// assert_eq!(finer_period.period().days(), 3);
    /// assert_eq!(finer_period.hours(), 4);
    /// assert_eq!(finer_period.minutes(), 5);
    /// assert_eq!(finer_period.milli_seconds(), 6007);
    /// assert_eq!(finer_period.nano_seconds(), 8009);
    /// ```
    pub fn new_from_string(val: &str) -> Result<Self> {
        let mut period = Period::default();
        let mut hours = 0;
        let mut minutes = 0;
        let mut milli_seconds = 0;
        let mut nano_seconds = 0;

        let mut num = 0;
        let mut is_period = true;
        for c in val.chars() {
            if c.is_ascii_digit() {
                num = num * 10 + c.to_digit(10).unwrap() as i32;
            } else {
                match c {
                    'Y' => {
                        period.years = num;
                        is_period = true;
                    },
                    'M' => {
                        period.months = num;
                        is_period = true;
                    },
                    'W' => {
                        period.days = num * 7;
                        is_period = true;
                    },
                    'D' => {
                        period.days = num;
                        is_period = true;
                    },
                    'h' => {
                        hours = num;
                        is_period = false;
                    },
                    'm' => {
                        minutes = num;
                        is_period = false;
                    },
                    's' => {
                        milli_seconds += num * 1000;
                        is_period = false;
                    },
                    'l' => {
                        milli_seconds += num;
                        is_period = false;
                    },
                    'u' => {
                        nano_seconds += num * 1000;
                        is_period = false;
                    },
                    'n' => {
                        nano_seconds += num;
                        is_period = false;
                    },
                    _ => {
                        let err = || anyhow::anyhow!("Invalid tenor string: {}", val);
                        return Err(err());
                    }
                }
                num = 0;
            }
        }

        if is_period {
            Ok(FinerPeriod::new(period, hours, minutes, milli_seconds, nano_seconds))
        } else {
            let err = || anyhow::anyhow!("Invalid tenor string: {}", val);
            Err(err())
        }
    }


}

#[cfg(test)]
mod tests {
    use super::*;
    use time::macros::datetime;

    #[test]
    fn test_month_opration() {
        let datetime = datetime!(2023-12-31 00:00:00 +09:00);
        for i in 1..=36 {
            let period = Period::new(0, i, 0);
            
            let new_datetime = period.apply(&datetime);

            let res = (from_month_to_i32(datetime.month()) + i) % 12;
            let res_month = from_i32_to_month(res);
            assert_eq!(
                new_datetime.month(), res_month,
                "Failed to add {} months to {}. Result: {}",
                i, datetime, new_datetime
            );
        }

        for i in 1..=36 {
            let period = Period::new(0, -i, 0);
            
            let new_datetime = period.apply(&datetime);

            let res = (from_month_to_i32(datetime.month()) - i) % 12;
            let res_month = from_i32_to_month(res);
            assert_eq!(
                new_datetime.month(), res_month,
                "Failed to add {} months to {}. Result: {}",
                i, datetime, new_datetime
            );
        }
    }

    #[test]
    fn test_year_opration() {
        let datetime = datetime!(2023-12-31 00:00:00 +09:00);
        for i in 1..=36 {
            let period = Period::new(i, 0, 0);
            let year = 2023 + i;
            let new_datetime = period.apply(&datetime);

            assert_eq!(new_datetime.year(), year);
        }

        for i in 1..=36 {
            let period = Period::new(-i, 0, 0);
            let year = 2023 - i;
            let new_datetime = period.apply(&datetime);

            assert_eq!(new_datetime.year(), year);
        }
    }
}