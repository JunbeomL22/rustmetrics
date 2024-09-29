use crate::currency::Currency;
use crate::definitions::Real;
use crate::utils::number_format::write_number_with_commas;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use crate::{
    InstType,
    AccountingLevel,
};
use static_id::StaticId;

/// Instrument information.
/// # Arguments
/// * `id` - (static_id::StaticId) A unique identifier for the instrument 
/// * `name` - (String) The name of the instrument
/// * `inst_type` - (InstType) The type of instrument
/// * `currency` - (Currency) The currency of the instrument
/// * `unit_notional` - (Real) The unit notional value of the instrument
/// * `issue_date` - (Option<OffsetDateTime>) The issue date of the instrument
/// * `maturity` - (Option<OffsetDateTime>) The maturity date of the instrument
/// * `accounting_level` - (AccountingLevel) The accounting level of the instrument
/// # Example
/// ```
/// use rustmetrics::definitions::Real;
/// use rustmetrics::Currency;
/// use rustmetrics::instruments::inst_info::InstInfo;
/// use rustmetrics::instruments::{AccountingLevel, InstType};
/// 
/// use serde_json;
/// use static_id::StaticId;
/// use time::OffsetDateTime;
/// 
/// let instrument_info = InstInfo {
///    id: StaticId::from_str("AAPL", "Some Brocker"),
///    name: "Apple Inc.".to_string(),
///    inst_type: InstType::Stock,
///    currency: Currency::USD,
///    unit_notional: 1.0,
///    issue_date: None,
///    maturity: None,
///    accounting_level: AccountingLevel::default(),
/// };
/// 
/// let serialized = serde_json::to_string_pretty(&instrument_info).unwrap();
/// 
/// println!("serialized = {}", serialized);
/// 
/// let deserialized: InstInfo = serde_json::from_str(&serialized).unwrap();
/// 
/// println!("deserialized = {:?}", deserialized);
///
/// assert_eq!(instrument_info, deserialized);
/// ```
#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct InstInfo {
    pub id: StaticId,
    pub name: String, // "" where not given
    pub inst_type: InstType,
    pub currency: Currency,
    pub unit_notional: Real,
    pub issue_date: Option<OffsetDateTime>,
    pub maturity: Option<OffsetDateTime>,
    pub accounting_level: AccountingLevel,
}

impl Default for InstInfo {
    fn default() -> InstInfo {
        InstInfo {
            id: StaticId::default(),
            name: "".to_string(),
            inst_type: InstType::default(),
            currency: Currency::default(),
            unit_notional: 1.0,
            issue_date: None,
            maturity: None,
            accounting_level: AccountingLevel::default(),
        }
    }
}

impl std::fmt::Debug for InstInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        writeln!(f, "    ID: {:?},", self.id)?;
        writeln!(f, "    name: {:?},", self.name)?;
        writeln!(f, "    instrument_type: {:?},", self.inst_type)?;
        writeln!(f, "    currency: {:?},", self.currency)?;
        write!(f, "    unit_notional: ")?;
        write_number_with_commas(f, self.unit_notional)?;
        writeln!(f)?;
        let _ = match self.issue_date {
            Some(issue_date) => writeln!(f, "    issue_date: {:?}", issue_date.date()),
            None => writeln!(f, "    issue_date: None"),
        };

        let _ = match self.maturity {
            Some(maturity) => writeln!(f, "    maturity: {:?}", maturity.date()),
            None => writeln!(f, "    maturity: None"),
        };

        let _ = writeln!(f, "    accounting_level: {:?}", self.accounting_level);

        Ok(())
    }
}

impl InstInfo {
    pub fn new(
        id: StaticId,
        name: String,
        inst_type: InstType,
        currency: Currency,
        unit_notional: Real,
        issue_date: Option<OffsetDateTime>,
        maturity: Option<OffsetDateTime>,
        accounting_level: AccountingLevel,
    ) -> InstInfo {
        InstInfo {
            id,
            name,
            inst_type,
            currency,
            unit_notional,
            issue_date,
            maturity,
            accounting_level,
        }
    }

    #[inline]
    pub fn type_name(&self) -> &'static str {
        self.inst_type.as_str()
    }

    #[inline]
    pub fn get_name(&self) -> &String {
        &self.name
    }

    #[inline]
    pub fn code_str(&self) -> &str {
        self.id.get_id().code.as_str()
    }

    #[inline]
    pub fn get_issue_date(&self) -> Option<&OffsetDateTime> {
        self.issue_date.as_ref()
    }

    #[inline]
    pub fn get_maturity(&self) -> Option<&OffsetDateTime> {
        self.maturity.as_ref()
    }

    #[inline]
    pub fn get_currency(&self) -> Currency {
        self.currency
    }

    #[inline]
    pub fn get_unit_notional(&self) -> Real {
        self.unit_notional
    }

    #[inline]
    pub fn get_accounting_level(&self) -> AccountingLevel {
        self.accounting_level
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::currency::Currency;
    //
    use serde_json;
    use anyhow::Result;

    #[test]
    fn test_instrument_info_serialization() -> Result<()> {
        let instrument_info = InstInfo {
            id: StaticId::from_str("AAPL", "KIS"),
            name: "Apple Inc.".to_string(),
            inst_type: InstType::Stock,
            currency: Currency::USD,
            unit_notional: 1.0,
            issue_date: None,
            maturity: None,
            accounting_level: AccountingLevel::default(),
        };

        let serialized = serde_json::to_string_pretty(&instrument_info).unwrap();

        println!("serialized = {}", serialized);

        let deserialized: InstInfo = serde_json::from_str(&serialized).unwrap();

        println!("deserialized = {:?}", deserialized);

        assert_eq!(instrument_info, deserialized);

        Ok(())
    }
}
