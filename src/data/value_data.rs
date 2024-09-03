use crate::currency::Currency;
use crate::definitions::Real;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use time::OffsetDateTime;
use static_id::StaticId;

/// value: Real, market_datetime: OffsetDateTime, name: String
/// The examples are flat volatility, constant continuous dividend yield
#[derive(Clone, Serialize, Deserialize)]
pub struct ValueData {
    pub value: Real,
    pub market_datetime: Option<OffsetDateTime>,
    pub currency: Currency,
    pub name: String,
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

    pub fn get_currency(&self) -> &Currency {
        &self.currency
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
