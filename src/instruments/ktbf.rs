use crate::definitions::{Integer, Real};
use crate::instrument::InstrumentTrait;
use crate::instruments::bond::Bond;
use crate::time::conventions::PaymentFrequency;
use static_id::static_id::StaticId;
//
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use crate::InstInfo;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct KtbfVirtualBond {
    year: Integer,
    coupon_rate: Real,
    frequency: PaymentFrequency,
    unit_notional: Real,
}

impl KtbfVirtualBond {
    pub fn new(
        year: Integer,
        coupon_rate: Real,
        frequency: PaymentFrequency,
        unit_notional: Real,
    ) -> KtbfVirtualBond {
        KtbfVirtualBond {
            year,
            coupon_rate,
            frequency,
            unit_notional,
        }
    }

    /// 파생상품시장 업무규정 시행세칙
    /// https://law.krx.co.kr/las/LawRevJo.jsp?lawid=000114&pubno=0000022080&pubdt=20240205
    pub fn npv(&self, bond_yield: Real) -> Real {
        let coupon_payment_number = self.year * self.frequency as Integer;
        let calc_freq = self.frequency.as_real();
        let effective_yield = bond_yield / calc_freq;
        let effective_coupon = self.coupon_rate / calc_freq;
        let mut res = 0.0;
        for i in 1..=coupon_payment_number {
            res += effective_coupon / (1.0 + effective_yield).powi(i);
        }
        res += 1.0 / (1.0 + effective_yield).powi(coupon_payment_number);
        res * self.unit_notional
    }
}

/// If settlement_date is None, it means that the settlement date is equal to the maturity date.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct KTBF {
    pub isnt_info: InstInfo,
    pub settlement_date: Option<OffsetDateTime>,
    pub virtual_bond: KtbfVirtualBond,
    pub underlying_bonds: Vec<Bond>,
    pub borrowing_curve_id: StaticId,
}

impl KTBF {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        inst_info: InstInfo,
        settlement_date: Option<OffsetDateTime>,
        virtual_bond: KtbfVirtualBond,
        underlying_bonds: Vec<Bond>,
        borrowing_curve_id: StaticId,
    ) -> Result<KTBF> {
        // all underlying_bonds should have the same pricing date as the maturity
        for bond in underlying_bonds.iter() {
            let pricing_date = bond.get_pricing_date()?;
            if pricing_date.is_none() {
                let err = || anyhow!(
                    "({}:{}) The pricing date of the underlying bond {} ({}) is not set",
                    file!(), line!(),
                    bond.get_name(),
                    bond.get_id(),
                );
                return Err(err());
            }
            let maturity = inst_info.get_maturity().unwrap();
            if pricing_date.unwrap() != maturity {
                let err = || anyhow!(
                    "({}:{}) The pricing date of the underlying bond {} ({}) in ktbf ({:?}) is not the same as the ktbf maturity\n\
                    bond pricing date: {:?},\n\
                    ktbf maturity: {:?}",
                    file!(), line!(),
                    bond.get_name(),
                    bond.get_id(),
                    inst_info.id,
                    pricing_date.unwrap(),
                    maturity,
                );
                return Err(err());
            }
        }

        Ok(KTBF {
            isnt_info: inst_info,
            settlement_date,
            virtual_bond,
            underlying_bonds,
            borrowing_curve_id,
        })
    }
    pub fn get_underlying_bonds(&self) -> &Vec<Bond> {
        &self.underlying_bonds
    }
}

impl InstrumentTrait for KTBF {
    fn get_inst_info(&self) ->  &InstInfo {
        &self.isnt_info
    }

    fn get_type_name(&self) -> &'static str {
        "KTBF"
    }

    fn get_virtual_bond_npv(&self, bond_yield: Real) -> Result<Real> {
        Ok(self.virtual_bond.npv(bond_yield))
    }

    fn get_underlying_bonds(&self) -> Result<&Vec<Bond>> {
        Ok(&self.underlying_bonds)
    }

    fn get_bond_futures_borrowing_curve_ids(&self) -> Vec<StaticId> {
        vec![self.borrowing_curve_id]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        Currency,
        InstType,
    };
    
    
    use anyhow::Result;
    use time::macros::datetime;

    #[test]
    fn test_serde() -> Result<()> {
        let inst_id = StaticId::from_str("KR7005930003", "KRX");
        let inst_info = InstInfo {
            id: inst_id,
            inst_type: InstType::KTBF,
            name: String::from("KTBF"),
            currency: Currency::KRW,
            issue_date: Some(datetime!(2021-01-01 00:00:00 +09:00)),
            maturity: Some(datetime!(2022-01-01 00:00:00 +09:00)),
            unit_notional: 100.0,
            accounting_level: crate::AccountingLevel::L1,
        };

        let virtual_bond = KtbfVirtualBond::new(
            5, 0.03, PaymentFrequency::SemiAnnually, 100.0
        );

        let mut bond = Bond::default();
        let bond_id = StaticId::from_str("KR7005930003", "KRX");
        let bond_inst_info = InstInfo {
            id: bond_id,
            inst_type: InstType::Bond,
            name: String::from("KTBF"),
            currency: Currency::KRW,
            issue_date: Some(datetime!(2021-01-01 00:00:00 +09:00)),
            maturity: Some(datetime!(2022-01-01 00:00:00 +09:00)),
            unit_notional: 100.0,
            accounting_level: crate::AccountingLevel::L1,
        };
        bond.set_inst_info(bond_inst_info);
        bond.set_pricing_date(datetime!(2022-01-01 00:00:00 +09:00));
        let borrowing_curve_id = StaticId::from_str("KR7005930003", "KRX");
        
        let ktbf = KTBF::new(
            inst_info,
            None,
            virtual_bond,
            vec![bond],
            borrowing_curve_id,
        )?;

        let serialized = serde_json::to_string(&ktbf)?;
        let deserialized: KTBF = serde_json::from_str(&serialized)?;
        assert_eq!(ktbf, deserialized);
        Ok(())
    }
}
