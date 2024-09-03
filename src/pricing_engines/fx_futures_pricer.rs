use crate::currency::Currency;
use crate::definitions::Real;
use crate::instrument::Instrument;
use crate::instrument::InstrumentTrait;
use crate::parameters::market_price::MarketPrice;
use crate::parameters::zero_curve::ZeroCurve;
use crate::pricing_engines::npv_result::NpvResult;
use crate::pricing_engines::pricer::PricerTrait;
//
use anyhow::{anyhow, Result};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

/// evaluation date is not needed for this pricer
/// all parameters have the evaluation date (shared in the form of Rc<RefCell<EvaluationDate>>)
pub struct FxFuturesPricer {
    //evaluation_date: Rc<RefCell<EvaluationDate>>, //not used
    fx: Rc<RefCell<MarketPrice>>, // floationg to fixed fx as in PlainSwapPricer.
    underlying_currency_curve: Rc<RefCell<ZeroCurve>>, // if you use implied dividend, this will be risk-free rate (or you can think of it as benchmark rate)
    futures_currency_curve: Rc<RefCell<ZeroCurve>>,    // or repo
}

impl FxFuturesPricer {
    pub fn new(
        //evaluation_date: Rc<RefCell<EvaluationDate>>,
        fx: Rc<RefCell<MarketPrice>>,
        underlying_currency_curve: Rc<RefCell<ZeroCurve>>,
        futures_currency_curve: Rc<RefCell<ZeroCurve>>,
    ) -> Self {
        FxFuturesPricer {
            //evaluation_date,
            fx,
            underlying_currency_curve,
            futures_currency_curve,
        }
    }
}

impl PricerTrait for FxFuturesPricer {
    fn npv(&self, instrument: &Instrument) -> Result<Real> {
        let fx_rate = self.fx.borrow().get_value();
        let maturity = match instrument.get_maturity() {
            Some(maturity) => maturity,
            None => {
                return Err(anyhow!(
                    "({}:{}) Maturity of {} ({}) is not set",
                    file!(),
                    line!(),
                    instrument.get_name(),
                    instrument.get_code_str(),
                ))
            }
        };

        let underlying_discount = self
            .underlying_currency_curve
            .borrow()
            .get_discount_factor_at_date(maturity)?;
        let futures_discount = self
            .futures_currency_curve
            .borrow()
            .get_discount_factor_at_date(maturity)?;

        let npv = fx_rate * underlying_discount / futures_discount;
        Ok(npv)
    }

    fn npv_result(&self, instrument: &Instrument) -> Result<NpvResult> {
        let npv = self.npv(instrument)?;
        Ok(NpvResult::new_from_npv(npv))
    }

    fn fx_exposure(&self, instrument: &Instrument, _npv: Real) -> Result<HashMap<Currency, Real>> {
        let average_trade_price = instrument.get_average_trade_price();
        let futures_currency = instrument.get_currency();
        let underlying_currency = instrument.get_underlying_currency()?;
        let maturity = match instrument.get_maturity() {
            Some(maturity) => maturity,
            None => {
                return Err(anyhow!(
                    "({}:{}) Maturity of {} ({}) is not set",
                    file!(),
                    line!(),
                    instrument.get_name(),
                    instrument.get_id(),
                ))
            }
        };

        let underlying_discount = self
            .underlying_currency_curve
            .borrow()
            .get_discount_factor_at_date(maturity)?;
        let futures_discount = self
            .futures_currency_curve
            .borrow()
            .get_discount_factor_at_date(maturity)?;

        let mut res: HashMap<Currency, Real> = HashMap::new();
        res.insert(futures_currency, -futures_discount * average_trade_price);
        res.insert(underlying_currency, underlying_discount);

        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::currency::Currency;
    use crate::data::vector_data::VectorData;
    use crate::evaluation_date::EvaluationDate;
    use crate::instrument::Instrument;
    use crate::instruments::fx_futures::FxFutures;
    use crate::parameters::zero_curve::ZeroCurve;
    use crate::pricing_engines::fx_futures_pricer::FxFuturesPricer;
    use crate::pricing_engines::pricer::PricerTrait;
    use crate::{
        InstInfo,
        InstType,
    };
    use anyhow::Result;
    use ndarray::array;
    use std::cell::RefCell;
    use std::rc::Rc;
    use time::macros::datetime;
    use static_id::StaticId;

    #[test]
    fn test_fx_futures_pricer() -> Result<()> {
        let eval_date = datetime!(2024-01-02 00:00:00 UTC);
        let evaluation_date = Rc::new(RefCell::new(EvaluationDate::new(eval_date.clone())));
        let fx = Rc::new(RefCell::new(MarketPrice::new(
            1300.0,
            eval_date.clone(),
            None,
            Currency::KRW,
            "USDKRW".to_string(),
            StaticId::from_str("USDKRW", "SMB"),
        )));

        let underlying_curve_data = VectorData::new(
            array![0.04, 0.04],
            None,
            Some(array![0.5, 5.0]),
            Some(eval_date),
            Currency::KRW,
            "USDOIS".to_string(),
            StaticId::from_str("USDOIS", "KAP"),
        )?;

        let usdois_curve = Rc::new(RefCell::new(ZeroCurve::new(
            evaluation_date.clone(),
            &underlying_curve_data,
            "USDOIS".to_string(),
            StaticId::from_str("USDOIS", "KAP"),
        )?));

        let futures_curve_data = VectorData::new(
            array![0.04, 0.04],
            None,
            Some(array![0.5, 5.0]),
            Some(eval_date),
            Currency::KRW,
            "KRWCRS".to_string(),
            StaticId::from_str("KRWCRS", "KAP"),
        )?;

        let krwcrs_curve = Rc::new(RefCell::new(ZeroCurve::new(
            evaluation_date.clone(),
            &futures_curve_data,
            "KRWCRS".to_string(),
            StaticId::from_str("KRWCRS", "KAP"),
        )?));

        let pricer = FxFuturesPricer::new(
            //evaluation_date.clone(),
            fx.clone(),
            usdois_curve.clone(),
            krwcrs_curve.clone(),
        );

        let issue_date = datetime!(2023-12-15 00:00:00 UTC);
        let last_trade_date = datetime!(2024-12-15 00:00:00 UTC);

        let inst_id = StaticId::from_str("USDKRW Futures", "KRX");
        let inst_info = InstInfo {
            id: inst_id,
            inst_type: InstType::FxFutures,
            name: "USDKRW Futures".to_string(),
            issue_date: Some(issue_date.clone()),
            maturity: Some(last_trade_date.clone()),
            currency: Currency::KRW,
            unit_notional: 10_000.0,
            accounting_level: crate::AccountingLevel::L1,
        };

        let fxfutures = FxFutures::new(
            inst_info,
            1_300.0,
            None,
            Currency::USD,
        );

        let inst = Instrument::FxFutures(fxfutures);
        let npv = pricer.npv_result(&inst)?;
        let fx_exporsure = pricer.fx_exposure(&inst, npv.get_npv())?;

        println!("npv result {:?}", npv);
        println!("fx exposure {:?}", fx_exporsure);

        let expected_npv = 1_300.0;
        assert!(
            (npv.get_npv() - expected_npv).abs() < 1e-6,
            "npv is not correct: expected {}, got {}",
            expected_npv,
            npv.get_npv(),
        );

        let expected_krw_fx_exposure = -1251.4957;
        assert!(
            (fx_exporsure.get(&Currency::KRW).unwrap() - expected_krw_fx_exposure).abs() < 1e-6,
            "KRW fx exposure is not correct: expected {}, got {}",
            expected_krw_fx_exposure,
            fx_exporsure.get(&Currency::KRW).unwrap(),
        );

        let expected_usd_fx_exposure = 0.96268904;
        assert!(
            (fx_exporsure.get(&Currency::USD).unwrap() - expected_usd_fx_exposure).abs() < 1e-6,
            "USD fx exposure is not correct: expected {}, got {}",
            expected_usd_fx_exposure,
            fx_exporsure.get(&Currency::USD).unwrap(),
        );
        Ok(())
    }
}
