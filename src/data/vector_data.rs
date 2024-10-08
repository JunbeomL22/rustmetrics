use crate::currency::Currency;
use crate::definitions::{Real, Time};
use crate::time::{calendar_trait::CalendarTrait, calendars::nullcalendar::NullCalendar};
use std::fmt;
use time::OffsetDateTime;
use static_id::static_id::StaticId;
use anyhow::{anyhow, Result};
use ndarray::Array1;
use serde::{Deserialize, Serialize};

/// Represents a vector of data points, typically used for term structures.
/// This struct encapsulates various attributes related to a series of data points,
/// including values, dates, times, and other identifying information.
/// /// # Example
///
/// ```
/// use rustmetrics::data::vector_data::VectorData;
/// use rustmetrics::Currency;
/// use ndarray::array;
/// use serde_json;
/// use static_id::StaticId;
///
/// let vector_data = VectorData::new(
///     array![1.0, 2.0, 3.0, 4.0, 5.0],
///     None,
///     Some(array![0.0, 1.0, 2.0, 3.0, 4.0]),
///     None,
///     Currency::KRW,
///     "test_vector_data_serialization".to_string(),
///     StaticId::from_str("test_vector_data_serialization", "test"),
/// ).expect("failed to create VectorData");
///
/// // Serialization
/// let serialized = serde_json::to_string(&vector_data).unwrap();
///
/// // Deserialization
/// let deserialized: VectorData = serde_json::from_str(&serialized).unwrap();
///
/// // Equality checks
/// assert_eq!(vector_data.get_value_clone(), deserialized.get_value_clone());
/// assert_eq!(vector_data.get_times_clone(), deserialized.get_times_clone());
/// ```
#[derive(Serialize, Deserialize, Clone)]
pub struct VectorData {
    /// The vector of numerical values.
    pub value: Array1<Real>,

    /// Optional vector of dates corresponding to each value.
    pub dates: Option<Vec<OffsetDateTime>>,

    /// Vector of times corresponding to each value.
    pub times: Array1<Time>,

    /// The market datetime for this data set, if applicable.
    pub market_datetime: Option<OffsetDateTime>,

    /// The currency in which the values are denominated.
    pub currency: Currency,

    /// A descriptive name for this data set.
    pub name: String,

    /// A unique identifier for this data set.
    pub id: StaticId,
}

impl fmt::Debug for VectorData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("VectorData")
            .field("value", &self.value)
            .field("dates", &self.dates)
            .field("times", &self.times)
            .field("market_datetime", &self.market_datetime)
            .field("currency", &self.currency)
            .field("name", &self.name)
            .field("id", &self.id)
            .finish()
    }
}

impl VectorData {
    /// Creates a new VectorData instance.
    ///
    /// # Arguments
    ///
    /// * `value` - The vector of numerical values.
    /// * `dates` - Optional vector of dates corresponding to each value.
    /// * `times` - Optional vector of times corresponding to each value.
    /// * `market_datetime` - The market datetime for this data set, if applicable.
    /// * `currency` - The currency of the values.
    /// * `name` - A descriptive name for this data set.
    /// * `id` - A unique identifier for this data set.
    ///
    /// # Returns
    ///
    /// Returns a Result containing the new VectorData instance or an error.
    ///
    /// # Notes
    ///
    /// Either `dates` or `times` must be `Some`. If both are `None`, an error will be returned.
    pub fn new(
        value: Array1<Real>,
        dates: Option<Vec<OffsetDateTime>>,
        times: Option<Array1<Time>>,
        market_datetime: Option<OffsetDateTime>,
        currency: Currency,
        name: String,
        id: StaticId,
    ) -> Result<VectorData> {
        // sanity check first
        if dates.is_none() && times.is_none() {
            return Err(anyhow!("dates and times are both None"));
        }

        if let Some(dates) = &dates {
            // change the following assertion to return Err
            if value.len() != dates.len() {
                return Err(anyhow!(
                    "The length of value and dates must be the same\n\
                    value: {:?}, dates: {:?}",
                    value,
                    dates,
                ));
            }

            let market_datetime = match market_datetime {
                Some(market_datetime) => market_datetime,
                None => {
                    return Err(anyhow!(
                        "({}:{}) the dates in VectorData of {} is not None, but market_datetime is None\n\
                        Thus, it is vague to calculate the time difference between market_datetime and dates",
                        file!(), line!(), name
                    ));
                }
            };
            let time_calculator = NullCalendar::default();
            let times: Array1<Time> = dates
                .iter()
                .map(|date| time_calculator.get_time_difference(&market_datetime, date))
                .collect();

            let res = VectorData {
                value,
                dates: Some(dates.to_vec()),
                times,
                market_datetime: Some(market_datetime),
                currency,
                name,
                id,
            };
            Ok(res)
        } else if let Some(times) = times {
            if value.len() != times.len() {
                return Err(anyhow!(
                    "The length of value and times must be the same\n\
                    value: {:?}, times: {:?}",
                    value,
                    times,
                ));
            } else {
                let res = VectorData {
                    value,
                    dates,
                    times,
                    market_datetime,
                    currency,
                    name,
                    id,
                };
                Ok(res)
            }
        } else {
            return Err(anyhow!("dates and times are both None"));
        }
    }

    pub fn get_name_clone(&self) -> String {
        self.name.clone()
    }

    pub fn get_value_clone(&self) -> Array1<Real> {
        self.value.clone()
    }

    pub fn get_times_clone(&self) -> Array1<Time> {
        self.times.clone()
    }

    pub fn get_dates_clone(&self) -> Option<Vec<OffsetDateTime>> {
        self.dates.clone()
    }

    pub fn test_curve_data(value: Real, currency: Currency) -> Result<VectorData> {
        let value = Array1::from(vec![value]);
        let times = Array1::from(vec![0.0]);
        
        let name = "test_vector_data".to_string();
        let id = StaticId::from_str(name.as_str(), "test");
        VectorData::new(
            value,
            None,
            Some(times),
            None,
            currency,
            name,
            id,
        )
    }

    pub fn test_dividend_data(value: Real, currency: Currency) -> Result<VectorData> {
        let values = Array1::from(vec![value]);
        let dates = vec![OffsetDateTime::now_utc() + time::Duration::days(1)];
        
        let name = "test_vector_data".to_string();
        let id = StaticId::from_str(name.as_str(), "test");
        VectorData::new(
            values,
            Some(dates),
            None,
            None,
            currency,
            name,
            id,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::array;
    use serde_json;
    
    #[test]
    fn test_vector_data_serialization() {
        let vector_data = VectorData::new(
            array![1.0, 2.0, 3.0, 4.0, 5.0],
            None,
            Some(array![0.0, 1.0, 2.0, 3.0, 4.0]),
            None, //datetime!(2020-01-01 00:00:00 UTC),
            Currency::KRW,
            "test_vector_data_serialization".to_string(),
            StaticId::from_str("test_vector_data_serialization", "test"),
        )
        .expect("failed to create VectorData");

        let serialized = serde_json::to_string(&vector_data).unwrap();
        println!("VectorData serialized = {}", serialized);
        let desrialized: VectorData = serde_json::from_str(&serialized).unwrap();
        println!("VectorData deserialized = {:?}", desrialized);

        // value check
        assert_eq!(vector_data.get_value_clone(), desrialized.get_value_clone());
        // times check
        assert_eq!(vector_data.get_times_clone(), desrialized.get_times_clone());
    }

    #[test]
    fn test_vector_data_test_data() {
        let vector_data = VectorData::test_curve_data(1.0, Currency::KRW).expect("failed to create VectorData");
        println!("VectorData = {:?}", vector_data);
        assert_eq!(vector_data.get_value_clone(), array![1.0]);
        assert_eq!(vector_data.get_times_clone(), array![0.0]);
    }
}
