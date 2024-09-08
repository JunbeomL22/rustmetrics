use crate::definitions::Real;
use crate::enums::Compounding;
use crate::evaluation_date::EvaluationDate;
use crate::instrument::{Instrument, InstrumentTrait};
use crate::parameters::zero_curve::ZeroCurve;
use crate::pricing_engines::{
    bond_pricer::BondPricer, krx_yield_pricer::KrxYieldPricer, npv_result::NpvResult,
    pricer::PricerTrait,
};
//
use anyhow::Result;
use std::{cell::RefCell, rc::Rc};

pub struct KtbfPricer {
    evaluation_date: Rc<RefCell<EvaluationDate>>,
    discount_curve: Rc<RefCell<ZeroCurve>>,
    borrowing_curve: Rc<RefCell<ZeroCurve>>,
}

impl KtbfPricer {
    pub fn new(
        evaluation_date: Rc<RefCell<EvaluationDate>>,
        discount_curve: Rc<RefCell<ZeroCurve>>,
        borrowing_curve: Rc<RefCell<ZeroCurve>>,
    ) -> KtbfPricer {
        KtbfPricer {
            evaluation_date,
            discount_curve,
            borrowing_curve,
        }
    }
}

impl PricerTrait for KtbfPricer {
    fn npv(&self, instrument: &Instrument) -> Result<Real> {
        let bond_pricer = BondPricer::new(
            self.evaluation_date.clone(),
            self.discount_curve.clone(),
            None,
            None,
        );

        let mut bond_yields = Vec::new();

        let underlying_bonds = instrument.get_underlying_bonds()?;

        let krx_yield_pricer = KrxYieldPricer::new(self.evaluation_date.clone(), 0.0, None, None);

        let init_guess = self
            .discount_curve
            .borrow()
            .get_forward_rate_from_evaluation_date(
                underlying_bonds[0].get_maturity().unwrap(),
                Compounding::Simple,
            )?;

        for bond in underlying_bonds.iter() {
            let inst = Instrument::Bond(bond.clone());
            let npv = bond_pricer.npv(&inst)?;
            let yield_ = krx_yield_pricer.find_bond_yield(bond.clone(), npv, Some(init_guess))?;
            bond_yields.push(yield_);
        }

        let average_yield = bond_yields.iter().sum::<Real>() / bond_yields.len() as Real;

        let mut ktbf_price = instrument.get_virtual_bond_npv(average_yield)?;

        let borrowing_cost = self
            .borrowing_curve
            .borrow()
            .get_discount_factor_at_date(instrument.get_maturity().unwrap())?;

        ktbf_price *= borrowing_cost;

        Ok(ktbf_price)
    }

    fn npv_result(&self, instrument: &Instrument) -> Result<NpvResult> {
        let npv = self.npv(instrument)?;
        Ok(NpvResult::new_from_npv(npv))
    }
}

#[cfg(test)]
mod tests {
    use crate::currency::Currency;
    use crate::data::vector_data::VectorData;
    use crate::enums::{CreditRating, IssuerType, RankType};
    use crate::evaluation_date::EvaluationDate;
    use crate::instrument::Instrument;
    use crate::instruments::bond::Bond;
    use crate::instruments::ktbf::{KtbfVirtualBond, KTBF};
    use crate::parameters::zero_curve::ZeroCurve;
    use crate::pricing_engines::{
        ktbf_pricer::KtbfPricer,
        pricer::{Pricer, PricerTrait},
    };
    use crate::time::{
        calendar::Calendar,
        calendars::southkorea::{SouthKorea, SouthKoreaType},
        conventions::{BusinessDayConvention, DayCountConvention, PaymentFrequency},
        jointcalendar::JointCalendar,
    };
    use crate::instruments::bond::BondInfo;
    use crate::{
        InstInfo,
        InstType,
    };
    //
    use anyhow::Result;
    use ndarray::array;
    use std::cell::RefCell;
    use std::rc::Rc;
    use time::macros::datetime;
    use time::Duration;
    use static_id::static_id::StaticId;

    #[test]
    fn test_ktbf_pricer() -> Result<()> {
        let eval_date = datetime!(2024-01-02 00:00:00 UTC);
        let evaluation_date = Rc::new(RefCell::new(EvaluationDate::new(eval_date)));
        let curve_data = VectorData::new(
            array![0.030, 0.040],
            None,
            Some(array![0.5, 5.0]),
            None, //eval_date.clone(),
            Currency::KRW,
            "KRWGOV".to_string(),
            StaticId::from_str("KRWGOV", "KAP"),
        )?;
        let discount_curve = ZeroCurve::new(
            evaluation_date.clone(),
            &curve_data,
            "KRWGOV".to_string(),
            StaticId::from_str("KRWGOV", "KAP"),
        )?;

        let borrowing_curve_data = VectorData::new(
            array![0.003],
            None,
            Some(array![0.5]),
            None, //eval_date.clone(),
            Currency::KRW,
            "KTBF3Y".to_string(),
            StaticId::from_str("KTBF3Y", "KRX"),
        )?;
        let borrowing_curve = ZeroCurve::new(
            evaluation_date.clone(),
            &borrowing_curve_data,
            "KTBF3Y".to_string(),
            StaticId::from_str("KTBF3Y", "KRX"),
        )?;
        let sk = Calendar::SouthKorea(SouthKorea::new(SouthKoreaType::Settlement));
        let calendar = JointCalendar::new(vec![sk])?;
        let ktbf_maturity = eval_date + Duration::days(90);
        
        // make two bonds whose maturity is 3 year and 5 year from the evaluation date
        // both are fixed coupon bond which 3% coupon rate
        // make it from crate::instruments::bond::Bond::new_from_convention
        let bond1_issue_date = datetime!(2024-01-02 00:00:00 UTC);
        let bond1_maturity = datetime!(2027-01-02 00:00:00 UTC);
        let inst_id = StaticId::from_str("Bond1", "KRX");
        let inst_info = InstInfo {
            id: inst_id,
            name: "Bond1".to_string(),
            inst_type: InstType::Bond,
            currency: Currency::KRW,
            unit_notional: 10_000.0,
            issue_date: Some(bond1_issue_date.clone()),
            maturity: Some(bond1_maturity.clone()),
            accounting_level: crate::AccountingLevel::L1,
        };

        let issuer_id = StaticId::from_str("Korea Government", "KRX");
        let bond_info = BondInfo {
            issuer_id,
            issuer_type: IssuerType::Government,
            credit_rating: CreditRating::None,
            rank: RankType::Senior,
        };

        let bond1 = Bond::new_from_conventions(
            inst_info,
            bond_info,
            
            false,
            //
            Some(bond1_issue_date.clone()),
            Some(ktbf_maturity.clone()),            
            None,
            //
            Some(0.03),
            None,
            None,
            None,
            //
            calendar.clone(),
            //
            true,
            DayCountConvention::StreetConvention,
            BusinessDayConvention::Unadjusted,
            PaymentFrequency::SemiAnnually,
            0,
            0,
        )?;

        let bond2_issue_date = datetime!(2024-01-02 00:00:00 UTC);
        let bond2_maturity = datetime!(2029-01-02 00:00:00 UTC);
        let bond_id2 = StaticId::from_str("Bond2", "KRX");
        let inst_info2 = InstInfo {
            id: bond_id2,
            name: "Bond2".to_string(),
            inst_type: InstType::Bond,
            currency: Currency::KRW,
            unit_notional: 10_000.0,
            issue_date: Some(bond2_issue_date.clone()),
            maturity: Some(bond2_maturity.clone()),
            accounting_level: crate::AccountingLevel::L1,
        };

        let bond_info2 = BondInfo {
            issuer_id,
            issuer_type: IssuerType::Government,
            credit_rating: CreditRating::None,
            rank: RankType::Senior,
        };
        let bond2 = Bond::new_from_conventions(
            inst_info2,
            bond_info2,
            //
            false,
            //
            Some(bond2_issue_date.clone()),
            Some(ktbf_maturity.clone()),
            None,
            //
            Some(0.03),
            None,
            None,
            None,
            //
            calendar.clone(),
            //
            true,
            DayCountConvention::StreetConvention,
            BusinessDayConvention::Unadjusted,
            PaymentFrequency::SemiAnnually,
            0,
            0,
        )?;

        let virtual_bond = KtbfVirtualBond::new(3, 0.05, PaymentFrequency::SemiAnnually, 100.0);

        let ktbf_issue_date = datetime!(2024-01-02 00:00:00 UTC);
        let ktbf_id = StaticId::from_str("KTBF3Y", "KRX");
        let ktbf_info = InstInfo {
            id: ktbf_id,
            name: "KTBF3Y".to_string(),
            inst_type: InstType::KTBF,
            currency: Currency::KRW,
            unit_notional: 1_000_000.0,
            issue_date: Some(ktbf_issue_date.clone()),
            maturity: Some(ktbf_maturity.clone()),
            accounting_level: crate::AccountingLevel::L1,
        };

        let ktbf = KTBF::new(
            ktbf_info,
            Some(ktbf_issue_date.clone()),
            virtual_bond,
            vec![bond1, bond2],
            StaticId::from_str("KTBF3Y", "KRX"),
        )?;

        let ktbf_pricer = KtbfPricer::new(
            evaluation_date.clone(),
            Rc::new(RefCell::new(discount_curve)),
            Rc::new(RefCell::new(borrowing_curve)),
        );

        let pricer = Pricer::KtbfPricer(ktbf_pricer);
        let npv = pricer.npv(&Instrument::KTBF(ktbf))?;
        println!("KTBF NPV: {}", npv);

        Ok(())
    }
}
