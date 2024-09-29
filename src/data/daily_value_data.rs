use crate::definitions::Real;
use crate::time::calendar::Calendar;
use crate::time::calendars::southkorea::{SouthKorea, SouthKoreaType};
use serde::{Deserialize, Serialize};
use rustc_hash::FxHashMap;
use static_id::static_id::StaticId;
use time::{Date, Time, UtcOffset};

/// Represents daily value data for a specific financial instrument or metric.
/// This struct encapsulates various attributes related to daily data points,
/// including the actual values, timing information, and identifiers.
/// # Example
/// ```
/// use rustmetrics::data::daily_value_data::DailyValueData;
/// use rustmetrics::time::calendar::Calendar;
/// use rustmetrics::time::calendars::southkorea::{SouthKorea, SouthKoreaType};
/// use time::macros::date;
/// 
/// let mut data = DailyValueData::default();
/// let date1 = date!(2021 - 01 - 01);
/// let date2 = date!(2021 - 01 - 02);
/// let date3 = date!(2021 - 01 - 03);
/// data.insert(date1, 100.0);
/// data.insert(date2, 200.0);
/// data.insert(date3, 300.0);
///
/// assert_eq!(data.get(&date1), Some(&100.0));
/// assert_eq!(data.get(&date2), Some(&200.0));
/// assert_eq!(data.get(&date3), Some(&300.0));
/// let (ordered_datetime, ordered_value) = data.get_ordered_data_by_date();
/// assert_eq!(ordered_datetime, vec![date1, date2, date3]);
/// assert_eq!(ordered_value, vec![100.0, 200.0, 300.0]);
/// ```
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DailyValueData {
    /// A map of dates to their corresponding real values.
    /// Each entry represents a daily data point.
    pub value: FxHashMap<Date, Real>,
    /// The closing time for the daily data points.
    pub close_time: Time,
    /// The UTC offset for the time zone of the data.
    pub utc_offset: UtcOffset,
    /// The calendar used for date calculations and business day conventions.
    pub calendar: Calendar,
    /// The name or description of the data series.
    pub name: String,
    /// A unique identifier for this data series.
    pub id: StaticId,
}

impl Default for DailyValueData {
    fn default() -> Self {
        DailyValueData {
            value: FxHashMap::default(),
            // korea stock market
            close_time: Time::from_hms(15, 40, 0).unwrap(),
            utc_offset: UtcOffset::from_hms(9, 0, 0).unwrap(),
            calendar: Calendar::SouthKorea(SouthKorea::new(SouthKoreaType::Krx)),
            name: String::new(),
            id: StaticId::default(),
        }
    }
}

impl DailyValueData {
    pub fn new(
        value: FxHashMap<Date, Real>,
        close_time: Time,
        utc_offset: UtcOffset,
        calendar: Calendar,
        name: String,
        id: StaticId,
    ) -> DailyValueData {
        DailyValueData {
            value,
            close_time,
            utc_offset,
            calendar,
            name,
            id,
        }
    }

    pub fn get_value(&self) -> &FxHashMap<Date, Real> {
        &self.value
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_id(&self) -> StaticId {
        self.id
    }

    // Get method
    pub fn get(&self, key: &Date) -> Option<&Real> {
        self.value.get(key)
    }

    pub fn get_calendar(&self) -> &Calendar {
        &self.calendar
    }

    pub fn get_close_time(&self) -> &Time {
        &self.close_time
    }

    pub fn get_utc_offset(&self) -> &UtcOffset {
        &self.utc_offset
    }

    pub fn insert(&mut self, key: Date, value: Real) {
        self.value.insert(key, value);
    }

    // Get mutable method
    pub fn get_mut(&mut self, key: &Date) -> Option<&mut Real> {
        self.value.get_mut(key)
    }

    pub fn get_ordered_data_by_date(&self) -> (Vec<Date>, Vec<Real>) {
        let mut ordered_data = self.value.iter().collect::<Vec<_>>();
        ordered_data.sort_by(|a, b| a.0.cmp(b.0));
        let mut ordered_datetime = Vec::new();
        let mut ordered_value = Vec::new();
        for (datetime, value) in ordered_data {
            ordered_datetime.push(*datetime);
            ordered_value.push(*value);
        }
        (ordered_datetime, ordered_value)
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::macros::date;

    #[test]
    fn test_close_data() {
        let mut data = DailyValueData::default();
        let date1 = date!(2021 - 01 - 01);
        let date2 = date!(2021 - 01 - 02);
        let date3 = date!(2021 - 01 - 03);
        data.insert(date1, 100.0);
        data.insert(date2, 200.0);
        data.insert(date3, 300.0);

        assert_eq!(data.get(&date1), Some(&100.0));
        assert_eq!(data.get(&date2), Some(&200.0));
        assert_eq!(data.get(&date3), Some(&300.0));
        let (ordered_datetime, ordered_value) = data.get_ordered_data_by_date();
        assert_eq!(ordered_datetime, vec![date1, date2, date3]);
        assert_eq!(ordered_value, vec![100.0, 200.0, 300.0]);
    }
}
