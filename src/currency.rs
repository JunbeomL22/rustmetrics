use serde::{Serialize, Deserialize, Serializer, Deserializer};
use serde::de::{self, Visitor};
use std::fmt::Display;
use std::hash::Hash;
use std::str::FromStr;  
use static_id::static_id::StaticId;

/// Enum representing various currencies.
/// # Example
/// ```
/// use serde_json;
/// use rustmetrics::Currency;
/// 
/// let currency = Currency::KRW;
/// let serialized = serde_json::to_string(&currency).unwrap();
/// assert_eq!(serialized, "\"KRW\"");
/// let deserialized: Currency = serde_json::from_str(&serialized).unwrap();
/// assert_eq!(deserialized, currency);
/// ```
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum Currency {
    /// Default value representing no currency.
    #[default]
    NIL,
    KRW,
    USD,
    EUR,
    JPY,
    CNY,
    CNH,
    GBP,
    AUD,
    CAD,
    CHF,
    NZD,
}

impl std::fmt::Display for Currency {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Currency::NIL => write!(f, "NIL"),
            Currency::KRW => write!(f, "KRW"),
            Currency::USD => write!(f, "USD"),
            Currency::EUR => write!(f, "EUR"),
            Currency::JPY => write!(f, "JPY"),
            Currency::CNY => write!(f, "CNY"),
            Currency::CNH => write!(f, "CNH"),
            Currency::GBP => write!(f, "GBP"),
            Currency::AUD => write!(f, "AUD"),
            Currency::CAD => write!(f, "CAD"),
            Currency::CHF => write!(f, "CHF"),
            Currency::NZD => write!(f, "NZD"),
        }
    }
}

impl Currency {
    pub fn as_str(&self) -> &'static str {
        match self {
            Currency::NIL => "NIL",
            Currency::KRW => "KRW",
            Currency::USD => "USD",
            Currency::EUR => "EUR",
            Currency::JPY => "JPY",
            Currency::CNY => "CNY",
            Currency::CNH => "CNH",
            Currency::GBP => "GBP",
            Currency::AUD => "AUD",
            Currency::CAD => "CAD",
            Currency::CHF => "CHF",
            Currency::NZD => "NZD",
        }
    }
}

impl FromStr for Currency {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "KRW" => Ok(Currency::KRW),
            "USD" => Ok(Currency::USD),
            "EUR" => Ok(Currency::EUR),
            "JPY" => Ok(Currency::JPY),
            "CNY" => Ok(Currency::CNY),
            "CNH" => Ok(Currency::CNH),
            "GBP" => Ok(Currency::GBP),
            "AUD" => Ok(Currency::AUD),
            "CAD" => Ok(Currency::CAD),
            "CHF" => Ok(Currency::CHF),
            "NZD" => Ok(Currency::NZD),
            _ => Err(format!("Invalid currency: {}", s)),
        }
    }
}

/// Implement conversion from &str to Currency.
impl From<&str> for Currency {
    fn from(s: &str) -> Self {
        match s {
            "KRW" => Currency::KRW,
            "USD" => Currency::USD,
            "EUR" => Currency::EUR,
            "JPY" => Currency::JPY,
            "CNY" => Currency::CNY,
            "CNH" => Currency::CNH,
            "GBP" => Currency::GBP,
            "AUD" => Currency::AUD,
            "CAD" => Currency::CAD,
            "CHF" => Currency::CHF,
            "NZD" => Currency::NZD,
            _ => Currency::NIL,
        }
    }
}

/// Struct representing a foreign exchange code, which consists of two currencies.
/// # Example
/// ```
/// use rustmetrics::FxCode;
/// use serde_json;
/// use rustmetrics::Currency;
/// 
/// let fxcode = FxCode::new(Currency::KRW, Currency::USD);
/// let serialized = serde_json::to_string(&fxcode).unwrap();
/// assert_eq!(serialized, "\"KRWUSD\"");
/// let deserialized: FxCode = serde_json::from_str(&serialized).unwrap();
/// assert_eq!(deserialized, fxcode);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub struct FxCode {
    pub currency1: Currency,
    pub currency2: Currency,
}

impl Serialize for FxCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let fxcode_str = format!("{}{}", self.currency1, self.currency2);
        serializer.serialize_str(&fxcode_str)
    }
}

struct FxCodeVisitor;

impl<'de> Visitor<'de> for FxCodeVisitor {
    type Value = FxCode;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a string with 6 uppercase letters representing two currency codes")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if value.len() != 6 {
            return Err(E::custom(format!("Invalid FxCode length: {}", value.len())));
        }

        let currency1 = Currency::from_str(&value[0..3])
            .map_err(|e| E::custom(format!("Invalid currency code: {}", e)))?;
        let currency2 = Currency::from_str(&value[3..6])
            .map_err(|e| E::custom(format!("Invalid currency code: {}", e)))?;

        Ok(FxCode { currency1, currency2 })
    }
}


impl<'de> Deserialize<'de> for FxCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(FxCodeVisitor)
    }
}

impl FxCode {
    /// Creates a new FxCode instance.
    /// # Arguments
    /// * `currency1` - The first currency in the pair.
    /// * `currency2` - The second currency in the pair.
    /// # Returns
    /// Returns a new FxCode instance.
    pub fn new(currency1: Currency, currency2: Currency) -> FxCode {
        FxCode {
            currency1,
            currency2,
        }
    }

    pub fn get_currency1(&self) -> Currency {
        self.currency1
    }

    pub fn get_currency2(&self) -> Currency {
        self.currency2
    }

    pub fn reciprocal(self) -> Self {
        FxCode {
            currency1: self.currency2,
            currency2: self.currency1,
        }
    }

    pub fn to_static_id(&self) -> StaticId {
        let s = format!("{:?}{:?}", self.currency1, self.currency2);
        StaticId::from_str(s.as_str(), "")
    }

}
impl Display for FxCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}{}", self.currency1.as_str(), self.currency2.as_str())
    }
}

impl Default for FxCode {
    fn default() -> FxCode {
        FxCode {
            currency1: Currency::NIL,
            currency2: Currency::NIL,
        }
    }
}
impl From<&str> for FxCode {
    fn from(code: &str) -> FxCode {
        let currency1 = Currency::from(&code[0..3]);
        let currency2 = Currency::from(&code[3..6]);

        FxCode {
            currency1,
            currency2,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{from_str, json, to_string};
    use crate::utils::memory_investigation::print_struct_info;

    #[test]
    fn show_memory_size() {
        print_struct_info(Currency::NIL);
        print_struct_info(&Currency::NIL);
        print_struct_info(FxCode::default());
    }

    #[test]
    fn test_currency_serialization() {
        let currency = Currency::KRW;
        let serialized = to_string(&currency).unwrap();

        assert_eq!(serialized, "\"KRW\"");
        let deserialized: Currency = from_str(&serialized).unwrap();

        assert_eq!(deserialized, currency);
    }

    #[test] // test for make json
    fn test_currency_json() {
        let currency = Currency::KRW;
        let json = json!(currency);

        assert_eq!(json, json!("KRW"));
        let deserialized: Currency = serde_json::from_value(json).unwrap();

        assert_eq!(deserialized, currency);
    }

    #[test] // test for as_str
    fn test_currency_as_str() {
        let currency = Currency::KRW;
        let as_str = currency.as_str();

        assert_eq!(as_str, "KRW");
    }

    #[test]
    fn test_fxcode_serialization() {
        let fxcode = FxCode::new(Currency::KRW, Currency::USD);
        let serialized = to_string(&fxcode).unwrap();
        assert_eq!(serialized, "\"KRWUSD\"");
        let deserialized: FxCode = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, fxcode);
    }
}
