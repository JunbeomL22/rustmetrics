use crate::currency::Currency;
use crate::definitions::Real;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use time::OffsetDateTime;
use static_id::static_id::StaticId;

/// Represents constant variable data such as volatility, stock price, or other financial metrics.
/// This struct encapsulates various attributes related to a single data point,
/// including the value, timestamp, currency, and identifiers.
///
/// # Example
///
/// ```
/// use rustmetrics::currency::Currency;
/// use rustmetrics::data::value_data::ValueData;
/// use static_id::StaticId;
///
/// let value_data = ValueData::new(
///     1.0,
///     None,
///     Currency::NIL,
///     "test".to_string(),
///     StaticId::from_str("test", "test"),
/// ).expect("Failed to create ValueData");
///
/// assert!(value_data.get_value() == 1.0);
/// ```
#[derive(Clone, Serialize, Deserialize)]
pub struct ValueData {
    /// The numerical value of the data point.
    /// This could represent volatility, stock price, or any other constant financial metric.
    pub value: Real,

    /// The date and time of the market data point, if available.
    /// This field is optional as some constant data might not have a specific market datetime.
    pub market_datetime: Option<OffsetDateTime>,

    /// The currency in which the value is denominated.
    /// Can be set to Currency::NIL if not applicable.
    pub currency: Currency,

    /// A descriptive name for the data point.
    /// This could be used to identify the type of data (e.g., "Constant Volatility", "Stock Price").
    pub name: String,

    /// A unique identifier for this data point.
    /// Can be created using StaticId::from_str().
    pub id: StaticId,
}

impl Debug for ValueData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ValueData")
            .field("value", &self.value)
            .field("market_datetime", &self.market_datetime)
            .field("name", &self.name)
            .field("code", &self.id)
            .finish()
    }
}

impl ValueData {
    /// Creates a new ValueData instance.
    ///
    /// # Arguments
    ///
    /// * `value` - The numerical value of the data point.
    /// * `market_datetime` - Optional market date and time.
    /// * `currency` - The currency of the value.
    /// * `name` - A descriptive name for the data point.
    /// * `id` - A unique identifier for the data point.
    ///
    /// # Returns
    ///
    /// Returns a Result containing the new ValueData instance or an error.
    pub fn new(
        value: Real,
        market_datetime: Option<OffsetDateTime>,
        currency: Currency,
        name: String,
        id: StaticId,
    ) -> Result<ValueData> {
        Ok(ValueData {
            value,
            market_datetime,
            currency,
            name,
            id,
        })
    }

    pub fn get_value(&self) -> Real {
        self.value
    }

    pub fn get_market_datetime(&self) -> &Option<OffsetDateTime> {
        &self.market_datetime
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_currency(&self) -> Currency {
        self.currency
    }
}

#[cfg(test)]
mod tests {
    use crate::currency::Currency;
    use crate::data::value_data::ValueData;
    use anyhow::Result;
    use static_id::StaticId;

    #[test]
    fn test_creation() -> Result<()> {
        let value_data = ValueData::new(
            1.0,
            None, //OffsetDateTime::now_utc(),
            Currency::NIL,
            "test".to_string(),
            StaticId::from_str("test", "test"),
        )
        .expect("Failed to create ValueData");
        assert!(value_data.get_value() == 1.0);
        Ok(())
    }
}
