use crate::currency::{Currency, FxCode};
use crate::definitions::Real;
use crate::enums::{
    CreditRating, IssuerType, OptionDailySettlementType, OptionType, RankType,
};

use crate::instruments::schedule::Schedule;
use crate::instruments::{
    AccountingLevel,
    bond::Bond,
    bond_futures::BondFutures,
    cash::Cash,
    futures::Futures,
    fx_futures::FxFutures,
    ktbf::KTBF,
    plain_swap::{PlainSwap, PlainSwapType},
    stock::Stock,
    vanilla_option::VanillaOption,
};

use crate::parameters::{
    past_price::DailyClosePrice, rate_index::RateIndex, zero_curve::ZeroCurve,
};
use crate::pricing_engines::match_parameter::MatchParameter;
use crate::time::{conventions::PaymentFrequency, jointcalendar::JointCalendar};
//
use static_id::StaticId;
use anyhow::{anyhow, Context, Result};
use enum_dispatch::enum_dispatch;
use std::{
    cell::RefCell,
    ops::Index,
    rc::Rc,
};
use rustc_hash::{
    FxHashMap,
    FxHashSet,
};
use crate::InstInfo;
use time::OffsetDateTime;

#[enum_dispatch]
pub trait InstrumentTrait {
    // The following methods are mandatory for all instruments
    fn get_inst_info(&self) -> &InstInfo;
    fn get_id(&self) -> StaticId { self.get_inst_info().id }
    fn get_name(&self) -> &String { self.get_inst_info().get_name() }
    fn get_code_str(&self) -> &str { self.get_inst_info().code_str() }
    fn get_currency(&self) -> Currency { self.get_inst_info().currency }
    fn get_unit_notional(&self) -> Real { self.get_inst_info().unit_notional }
    fn get_type_name(&self) -> &'static str { self.get_inst_info().type_name() }
    fn get_average_trade_price(&self) -> Real { 0.0 }
    fn get_accountring_level(&self) -> AccountingLevel { self.get_inst_info().accounting_level }
    //
    // There is an instrument that does not have maturity date, so it is optional
    fn get_maturity(&self) -> Option<&OffsetDateTime> { self.get_inst_info().get_maturity() }
    fn get_issue_date(&self) -> Option<&OffsetDateTime> { self.get_inst_info().get_issue_date() }
    // There is an instrument that does not have underlying names,
    // so the default action is to return an empty vector
    fn get_underlying_ids(&self) -> Vec<StaticId> { vec![] }

    fn get_quanto_fxcode_und_pair(&self) -> Vec<(StaticId, &FxCode)> { vec![] }

    fn get_all_fxcodes_for_pricing(&self) -> Vec<FxCode> { vec![] }

    fn get_underlying_ids_requiring_volatility(&self) -> Vec<StaticId> { vec![] }
    /// only for bonds, so None must be allowed
    fn get_credit_rating(&self) -> Result<CreditRating> {
        let err = || anyhow!(
            "({}:{}) not supported instrument type on get_credit_rating",
            file!(),
            line!()
        );
        Err(err())
    }
    /// only for bonds, so None must be allowed
    fn get_issuer_type(&self) -> Result<IssuerType> {
        let lazy_err = || anyhow!(
            "({}:{}) not supported instrument type on get_issuer_type",
            file!(),
            line!()
        );
        Err(lazy_err())
    }
    /// only for bonds, so None must be allowed
    fn get_rank_type(&self) -> Result<RankType> {
        let lazy_err = || anyhow!(
            "({}:{}) not supported instrument type on get_rank_type",
            file!(),
            line!()
        );
        Err(lazy_err())
    }
    // only for bonds, so None must be allowed
    fn get_issuer_id(&self) -> Result<StaticId> {
        let err = || anyhow!(
            "({}:{}) not supported instrument type on get_issuer_name",
            file!(),
            line!()
        );
        Err(err())
    }

    // only for FloatingRateNote, IRS, OIS, and other swaps
    fn get_rate_index(&self) -> Result<Option<&RateIndex>> {
        let err = || anyhow!(
            "({}:{}) not supported instrument type on get_rate_index",
            file!(),
            line!()
        );
        Err(err())
    }

    fn get_bond_futures_borrowing_curve_ids(&self) -> Vec<StaticId> {
        vec![]
    }

    fn get_cashflows(
        &self,
        _pricing_date: &OffsetDateTime,
        _forward_curve: Option<Rc<RefCell<ZeroCurve>>>,
        _past_data: Option<Rc<DailyClosePrice>>,
    ) -> Result<FxHashMap<OffsetDateTime, Real>> {
        Err(anyhow!(
            "not supported instrument type on get_coupon_cashflow"
        ))
    }

    fn get_floating_cashflows(
        &self,
        _pricing_date: &OffsetDateTime,
        _forward_curve: Option<Rc<RefCell<ZeroCurve>>>,
        _past_data: Option<Rc<DailyClosePrice>>,
    ) -> Result<FxHashMap<OffsetDateTime, Real>> {
        Err(anyhow!(
            "not supported instrument type on get_floating_cashflows"
        ))
    }

    fn get_fixed_cashflows(
        &self,
        _pricing_date: &OffsetDateTime,
    ) -> Result<FxHashMap<OffsetDateTime, Real>> {
        Err(anyhow!(
            "not supported instrument type on get_fixed_cashflows"
        ))
    }

    fn get_pricing_date(&self) -> Result<Option<&OffsetDateTime>, anyhow::Error> {
        Err(anyhow!("not supported instrument type on get_pricing_date"))
    }

    fn is_coupon_strip(&self) -> Result<bool> {
        Err(anyhow!("not supported instrument type on is_coupon_strip"))
    }

    fn get_underlying_bonds(&self) -> Result<&Vec<Bond>> {
        Err(anyhow!(
            "not supported instrument type on get_underlying_bonds"
        ))
    }

    fn get_coupon_frequency(&self) -> Result<PaymentFrequency> {
        Err(anyhow!("not supported instrument type on get_frequency"))
    }

    fn get_calendar(&self) -> Result<&JointCalendar> {
        Err(anyhow!("not supported instrument type on get_calendar"))
    }

    fn get_virtual_bond_npv(&self, _bond_yield: Real) -> Result<Real> {
        Err(anyhow!(
            "not supported instrument type on get_virtual_bond_npv"
        ))
    }

    fn get_schedule(&self) -> Result<&Schedule> {
        Err(anyhow!("not supported instrument type on get_schedule"))
    }

    fn get_fixed_leg_currency(&self) -> Result<Currency> {
        Err(anyhow!(
            "not supported instrument type on get_fixed_leg_currency"
        ))
    }

    fn get_floating_leg_currency(&self) -> Result<Currency> {
        Err(anyhow!(
            "not supported instrument type on get_floating_leg_currency"
        ))
    }

    fn get_underlying_currency(&self) -> Result<Currency> {
        Err(anyhow!(
            "not supported instrument type on get_underlying_currency"
        ))
    }

    fn get_strike(&self) -> Result<Real> {
        Err(anyhow!("not supported instrument type on get_strike"))
    }

    fn get_option_type(&self) -> Result<OptionType> {
        Err(anyhow!("not supported instrument type on get_option_type"))
    }

    fn get_option_daily_settlement_type(&self) -> Result<OptionDailySettlementType> {
        Err(anyhow!(
            "not supported instrument type on get_option_daily_settlement_type"
        ))
    }

    fn get_fxfutres_und_fxcode(&self) -> Result<FxCode> {
        Err(anyhow!("not supported instrument type on get_fx_code"))
    }

    fn get_floating_to_fixed_fxcode(&self) -> Result<Option<FxCode>> {
        Err(anyhow!(
            "get_floating_to_fixed_fx allowed only for PlainSwap"
        ))
    }

    fn get_specific_plain_swap_type(&self) -> Result<PlainSwapType> {
        Err(anyhow!(
            "not supported instrument type on get_specific_plain_swap_type"
        ))
    }
}

#[enum_dispatch(InstrumentTrait)]
#[derive(Clone, Debug)]
pub enum Instrument {
    Futures(Futures),
    Bond(Bond),
    BondFutures(BondFutures),
    KTBF(KTBF),
    PlainSwap(PlainSwap),
    FxFutures(FxFutures),
    VanillaOption(VanillaOption),
    Stock(Stock),
    Cash(Cash),
}

/// calculation groups for calculation optimization,
/// On the group, again select calculation sets based on currency and underlying assets (not sub|superset, exact the same assets)
/// currency and underlying_assets categorization
/// GROUP1: Vec<&'static str> = vec!["StockFutures"];
/// GROUP2: Vec<&'static str> = vec!["FixedCouponBond", "BondFutures", "KTBF"];
/// GROUP3: Vec<&'static str> = vec!["StructuredProduct"];
#[derive(Clone, Debug, Default)]
pub struct Instruments {
    instruments: Vec<Rc<Instrument>>,
}

impl Index<usize> for Instruments {
    type Output = Instrument;

    fn index(&self, index: usize) -> &Self::Output {
        &self.instruments[index]
    }
}

impl Instruments {
    pub fn iter(&self) -> std::slice::Iter<'_, Rc<Instrument>> {
        self.instruments.iter()
    }

    pub fn new(instruments: Vec<Rc<Instrument>>) -> Instruments {
        Instruments { instruments }
    }

    pub fn len(&self) -> usize {
        self.instruments.len()
    }

    pub fn is_empty(&self) -> bool {
        self.instruments.is_empty()
    }

    pub fn get_instruments_clone(&self) -> Vec<Rc<Instrument>> {
        let mut res = Vec::<Rc<Instrument>>::new();
        for instrument in self.instruments.iter() {
            res.push(instrument.clone());
        }
        res
    }

    pub fn get_all_underlying_ids(&self) -> Vec<StaticId> {
        let mut underlying_ids = Vec::<StaticId>::new();
        for instrument in self.instruments.iter() {
            let ids = instrument.get_underlying_ids();
            for id in ids.iter() {
                if !underlying_ids.contains(id) {
                    underlying_ids.push(*id);
                }
            }
        }
        underlying_ids
    }

    pub fn get_all_fxcodes_for_pricing(&self) -> Vec<FxCode> {
        let mut fxcodes = Vec::<FxCode>::new();
        for instrument in self.instruments.iter() {
            let codes = instrument.get_all_fxcodes_for_pricing();
            for code in codes.iter() {
                if !fxcodes.contains(code) {
                    fxcodes.push(*code);
                }
            }
        }
        fxcodes
    }

    pub fn get_all_quanto_fxcode_und_pairs(&self) -> FxHashSet<(StaticId, &FxCode)> {
        let mut fxcodes = FxHashSet::default();
        for instrument in self.instruments.iter() {
            let codes = instrument.get_quanto_fxcode_und_pair();
            for code in codes.iter() {
                fxcodes.insert(*code);
            }
        }
        fxcodes
    }

    pub fn get_all_type_names(&self) -> Vec<&'static str> {
        let mut type_names = Vec::<&'static str>::new();
        for instrument in self.instruments.iter() {
            let name = instrument.get_type_name();
            if !type_names.contains(&name) {
                type_names.push(name);
            }
        }
        type_names
    }

    pub fn get_all_currencies(&self) -> Result<Vec<Currency>> {
        let mut currencies = Vec::<Currency>::new();
        for instrument in self.instruments.iter() {
            let currency = instrument.get_currency();
            if !currencies.contains(&currency) {
                currencies.push(currency);
            }

            match instrument.get_type_name() {
                "Futures" | "FxFutures" => {
                    let currency = instrument.get_underlying_currency().with_context(|| {
                        anyhow!(
                            "({}:{}) get_underlying_currency failed for {} ({})",
                            file!(),
                            line!(),
                            instrument.get_name(),
                            instrument.get_code_str(),
                        )
                    })?;
                    if !currencies.contains(&currency) {
                        currencies.push(currency);
                    }
                }
                "PlainSwap" => {
                    let currency = instrument.get_floating_leg_currency().with_context(|| {
                        anyhow!(
                            "({}:{}) get_floating_leg_currency failed for {} ({})",
                            file!(),
                            line!(),
                            instrument.get_name(),
                            instrument.get_code_str(),
                        )
                    })?;
                    if !currencies.contains(&currency) {
                        currencies.push(currency);
                    }
                }
                _ => {}
            }
        }
        Ok(currencies)
    }

    pub fn instruments_with_underlying(
        &self,
        und_id: StaticId,
        exclude_type: Option<Vec<&str>>,
    ) -> Vec<Rc<Instrument>> {
        let exclude_type = exclude_type.unwrap_or_default();
        let mut res = Vec::<Rc<Instrument>>::new();
        for instrument in self.instruments.iter() {
            let ids = instrument.get_underlying_ids();
            let type_name = instrument.get_type_name();
            if ids.contains(&und_id) && !exclude_type.contains(&type_name) {
                res.push(instrument.clone());
            }
        }
        res
    }

    pub fn instruments_with_currency(&self, currency: Currency) -> Vec<Rc<Instrument>> {
        let mut res = Vec::<Rc<Instrument>>::new();
        for instrument in self.instruments.iter() {
            if instrument.get_currency() == currency {
                res.push(instrument.clone());
            }
        }
        res
    }

    pub fn instruments_with_types(&self, type_names: Vec<&str>) -> Vec<Rc<Instrument>> {
        let mut res = Vec::<Rc<Instrument>>::new();
        for instrument in self.instruments.iter() {
            let type_name = instrument.get_type_name();
            if type_names.contains(&type_name) {
                res.push(instrument.clone());
            }
        }
        res
    }

    pub fn instruments_using_curve(
        &self,
        curve_id: StaticId,
        match_parameter: &MatchParameter,
        exclude_type: Option<Vec<&str>>,
    ) -> Result<Vec<Rc<Instrument>>> {
        let mut res = Vec::<Rc<Instrument>>::new();
        let exclude_type = exclude_type.unwrap_or_default();
        // 1) discount curve
        // 2) collateral curves
        // 3) rate index forward curves
        // borrowing curve can not be hedged, so it skips
        for instrument in self.instruments.iter() {
            if exclude_type.contains(&instrument.get_type_name()) {
                continue;
            }
            // 1)
            if match_parameter.get_discount_curve_id(instrument)? == curve_id {
                res.push(instrument.clone());
            }
            // 2)
            if match_parameter
                .get_collateral_curve_ids(instrument)?
                .contains(&curve_id)
            {
                res.push(instrument.clone());
            }
            // 3) forward curve
            if match_parameter.get_rate_index_curve_id(instrument)? == curve_id {
                res.push(instrument.clone());
            }
            // 4) crs curve
            if match_parameter.get_crs_curve_id(instrument)? == curve_id {
                res.push(instrument.clone());
            }
            // 5) floating crs curve
            if match_parameter.get_floating_crs_curve_id(instrument)? == curve_id {
                res.push(instrument.clone());
            }
        }
        Ok(res)
    }

    // all curve names including discount, collateral, and rate index forward curves
    pub fn get_all_curve_ids<'a>(
        &'a self,
        match_parameter: &'a MatchParameter,
    ) -> Result<Vec<StaticId>> {
        let mut res = Vec::<StaticId>::new();
        let dummy_id = StaticId::default();
        for instrument in self.instruments.iter() {
            let discount_curve_id = match_parameter.get_discount_curve_id(instrument)?;
            if !res.contains(&discount_curve_id) && discount_curve_id != dummy_id {
                res.push(discount_curve_id);
            }
            let collateral_curve_ids = match_parameter.get_collateral_curve_ids(instrument)?;
            for id in collateral_curve_ids.iter() {
                if !res.contains(id) && *id != dummy_id {
                    res.push(*id);
                }
            }
            let rate_index_curve_id = match_parameter.get_rate_index_curve_id(instrument)?;
            if !res.contains(&rate_index_curve_id) && rate_index_curve_id != dummy_id {
                res.push(rate_index_curve_id);
            }
            let crs_curve_name = match_parameter.get_crs_curve_id(instrument)?;
            if !res.contains(&crs_curve_name) && crs_curve_name != dummy_id {
                res.push(crs_curve_name);
            }
            let floating_crs_curve_name =
                match_parameter.get_floating_crs_curve_id(instrument)?;
            if !res.contains(&floating_crs_curve_name) && floating_crs_curve_name != dummy_id {
                res.push(floating_crs_curve_name);
            }
        }
        Ok(res)
    }

    pub fn instruments_with_maturity_upto(
        &self,
        instruments: Option<&Vec<Rc<Instrument>>>,
        maturity: &OffsetDateTime,
        exlucde_type: Option<Vec<&str>>,
    ) -> Vec<Rc<Instrument>> {
        let exlucde_type = exlucde_type.unwrap_or_default();

        match instruments {
            Some(instruments) => {
                let mut res = Vec::<Rc<Instrument>>::new();
                for instrument in instruments.iter() {
                    if exlucde_type.contains(&instrument.get_type_name()) {
                        continue;
                    }
                    if let Some(m) = instrument.get_maturity() {
                        if m <= maturity {
                            res.push(instrument.clone());
                        }
                    }
                }
                res
            }
            None => {
                let mut res = Vec::<Rc<Instrument>>::new();
                for instrument in self.instruments.iter() {
                    if exlucde_type.contains(&instrument.get_type_name()) {
                        continue;
                    }
                    if let Some(m) = instrument.get_maturity() {
                        if m <= maturity {
                            res.push(instrument.clone());
                        }
                    }
                }
                res
            }
        }
    }

    pub fn instruments_with_maturity_over(
        &self,
        instruments: Option<&Vec<Rc<Instrument>>>,
        maturity: &OffsetDateTime,
        exclude_type: Option<Vec<&str>>,
    ) -> Vec<Rc<Instrument>> {
        let exclude_type = exclude_type.unwrap_or_default();

        match instruments {
            Some(instruments) => {
                let mut res = Vec::<Rc<Instrument>>::new();
                for instrument in instruments.iter() {
                    if exclude_type.contains(&instrument.get_type_name()) {
                        continue;
                    }

                    if instrument.get_maturity().is_none() {
                        res.push(instrument.clone());
                    }

                    if let Some(m) = instrument.get_maturity() {
                        if m > maturity {
                            res.push(instrument.clone());
                        }
                    }
                }
                res
            }
            None => {
                let mut res = Vec::<Rc<Instrument>>::new();
                for instrument in self.instruments.iter() {
                    if exclude_type.contains(&instrument.get_type_name()) {
                        continue;
                    }

                    if instrument.get_maturity().is_none() {
                        res.push(instrument.clone());
                    }

                    if let Some(m) = instrument.get_maturity() {
                        if m > maturity {
                            res.push(instrument.clone());
                        }
                    }
                }
                res
            }
        }
    }

    /// This method return the shortest maturity of the given instruments
    /// If instruments is None, it gives the shortest maturity of all instruments
    /// None maturity is taken as an infinite maturity
    /// Therefore, if there is no maturity, it is considered as the longest maturity
    pub fn get_shortest_maturity(
        &self,
        instruments: Option<&Vec<Rc<Instrument>>>,
    ) -> Option<OffsetDateTime> {
        match instruments {
            Some(instruments) => {
                let mut shortest_maturity: Option<OffsetDateTime> = None;
                for instrument in instruments.iter() {
                    if let Some(m) = instrument.get_maturity() {
                        if let Some(sm) = shortest_maturity {
                            if *m < sm {
                                shortest_maturity = Some(*m);
                            }
                        } else {
                            shortest_maturity = Some(*m);
                        }
                    }
                }
                shortest_maturity
            }
            None => {
                let mut shortest_maturity: Option<OffsetDateTime> = None;
                for instrument in self.instruments.iter() {
                    if let Some(m) = instrument.get_maturity() {
                        if let Some(sm) = shortest_maturity {
                            if *m < sm {
                                shortest_maturity = Some(*m);
                            }
                        } else {
                            shortest_maturity = Some(*m);
                        }
                    }
                }
                shortest_maturity
            }
        }
    }

    /// This method return the longest maturity of the given instruments
    /// If instruments is None, it gives the longest maturity of all instruments
    /// None maturity is taken as an infinite maturity
    /// Therefore, if there is no maturity, it is considered as the longest maturity
    pub fn get_longest_maturity(
        &self,
        instruments: Option<&Vec<Rc<Instrument>>>,
    ) -> Option<OffsetDateTime> {
        match instruments {
            Some(instruments) => {
                let mut longest_maturity: Option<OffsetDateTime> = None;
                for instrument in instruments.iter() {
                    match instrument.get_maturity() {
                        Some(m) => {
                            if let Some(sm) = longest_maturity {
                                if *m > sm {
                                    longest_maturity = Some(*m);
                                }
                            } else {
                                longest_maturity = Some(*m);
                            }
                        }
                        None => {
                            longest_maturity = None;
                            break;
                        }
                    }
                }
                longest_maturity
            }
            None => {
                let mut longest_maturity: Option<OffsetDateTime> = None;
                for instrument in self.instruments.iter() {
                    match instrument.get_maturity() {
                        Some(m) => {
                            if let Some(sm) = longest_maturity {
                                if *m > sm {
                                    longest_maturity = Some(*m);
                                }
                            } else {
                                longest_maturity = Some(*m);
                            }
                        }
                        None => {
                            longest_maturity = None;
                            break;
                        }
                    }
                }
                longest_maturity
            }
        }
    }

    pub fn get_all_inst_id(
        &self,
        instruments: Option<&Vec<Rc<Instrument>>>,
    ) -> Vec<StaticId> {
        match instruments {
            Some(instruments) => {
                let mut res = Vec::<String>::new();
                for instrument in instruments.iter() {
                    res.push(instrument.get_code_str());
                }
                res
            }
            None => {
                let mut res = Vec::<String>::new();
                for instrument in self.instruments.iter() {
                    res.push(instrument.get_code_str().clone());
                }
                res
            }
        }
    }

    pub fn get_all_unerlying_ids_requiring_volatility(
        &self,
        instruments: Option<&Vec<Rc<Instrument>>>,
    ) -> Vec<StaticId> {
        match instruments {
            Some(instruments) => {
                let mut res = Vec::<StaticId>::new();
                for instrument in instruments.iter() {
                    let ids = instrument.get_underlying_ids_requiring_volatility();
                    for id in ids.iter() {
                        if !res.contains(id) {
                            res.push(*id);
                        }
                    }
                }
                res
            }
            None => {
                let mut res = Vec::<StaticId>::new();
                for instrument in self.instruments.iter() {
                    let ids = instrument.get_underlying_ids_requiring_volatility();
                    for id in ids.iter() {
                        if !res.contains(id) {
                            res.push(*id);
                        }
                    }
                }
                res
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::currency::Currency;
    use crate::instruments::futures::Futures;
    use crate::instruments::plain_swap::PlainSwap;
    use crate::parameters::rate_index::RateIndex;
    //use crate::enums::RateIndexCode;
    use crate::time::conventions::{BusinessDayConvention, DayCountConvention, PaymentFrequency};
    use crate::time::{
        calendar::Calendar,
        calendars::southkorea::{SouthKorea, SouthKoreaType},
        jointcalendar::JointCalendar,
    };
    use crate::InstType;
    use anyhow::Result;
    use time::macros::datetime;

    #[test]
    fn test_instruments() -> Result<()> {
        let kospi_und_id = StaticId::from_str("KOSPI2", "KRX");
        let inst_id = StaticId::from_str("KOSPI2 FUT", "KRX");

        let issue_date = datetime!(2022-01-01 09:00:00 UTC);  
        let maturity = datetime!(2022-12-01 09:00:00 UTC);
        let inst_info = InstInfo {
            id: inst_id,
            name: "KOSPI2 FUT".to_string(),
            inst_type: InstType::Futures,
            currency: Currency::KRW,
            unit_notional: 250_000.0,
            issue_date: Some(issue_date),
            maturity: Some(maturity),
            accounting_level: AccountingLevel::L1,
        };
        
        let fut1 = Futures::new(
            inst_info,
            340.0,
            None,
            Currency::KRW,
            kospi_und_id,
        );

        let issue_date = datetime!(2022-12-01 09:00:00 UTC);
        let maturity = datetime!(2024-03-01 09:00:00 UTC);
        let inst_id = StaticId::from_str("SPX FUT", "CME");
        let spx_und_id = StaticId::from_str("SPX", "CME");
        let inst_info = InstInfo {
            id: inst_id,
            name: "SPX FUT".to_string(),
            inst_type: InstType::Futures,
            currency: Currency::USD,
            unit_notional: 50.0,
            issue_date: Some(issue_date),
            maturity: Some(maturity),
            accounting_level: AccountingLevel::L1,
        };

        let fut2 = Futures::new(
            inst_info,
            5000.0,
            None,
            Currency::USD,
            spx_und_id,
        );

        let sk = SouthKorea::new(SouthKoreaType::Settlement);
        let _sk = Calendar::SouthKorea(sk);

        let rate_id = StaticId::from_str("CD 91D", "KRX");
        let rate_tenor = crate::Tenor::new_from_string("91D")?;
        let rate_index = RateIndex::new(
            rate_id,
            rate_tenor,
            Currency::KRW,
            String::from("CD 91D"),
        )?;

        let sk = SouthKorea::new(SouthKoreaType::Settlement);
        let sk = Calendar::SouthKorea(sk);
        let joint_calendar = JointCalendar::new(vec![sk])?;

        let issue_date = datetime!(2021-01-01 09:00:00 UTC);
        let maturity_date = datetime!(2021-12-31 09:00:00 UTC);

        let swap_id = StaticId::from_str("KRW IRS Code", "KRX");
        let swap_info = InstInfo {
            id: swap_id,
            name: "KRW IRS".to_string(),
            inst_type: InstType::PlainSwap,
            currency: Currency::KRW,
            unit_notional: 10_000_000_000.0,
            issue_date: Some(issue_date.clone()),
            maturity: Some(maturity_date),
            accounting_level: AccountingLevel::L2,
        };

        let irs = PlainSwap::new_from_conventions(
            inst_info,
            Currency::KRW,
            //
            None,
            None,
            None,
            None,
            //
            issue_date.clone(),
            //
            Some(0.02),
            Some(rate_index.clone()),
            None,
            //
            true,
            DayCountConvention::Actual365Fixed,
            DayCountConvention::Actual365Fixed,
            BusinessDayConvention::ModifiedFollowing,
            BusinessDayConvention::ModifiedFollowing,
            PaymentFrequency::Quarterly,
            PaymentFrequency::Quarterly,
            //
            1,
            0,
            //
            joint_calendar,
        )?;

        // make Instrument using fut1, fut2, irs
        let instruments = Instruments::new(vec![
            Rc::new(Instrument::Futures(fut1.clone())),
            Rc::new(Instrument::Futures(fut2.clone())),
            Rc::new(Instrument::PlainSwap(irs.clone())),
        ]);

        // make MatchParameter
        let mut collateral_curve_map = FxHashMap::<StaticId, StaticId>::default();
        let mut rate_index_curve_map = FxHashMap::<StaticId, StaticId>::default();
        let borrowing_curve_map = FxHashMap::<StaticId, StaticId>::default();
        let bond_curve_map = FxHashMap::<(StaticId, IssuerType, CreditRating, Currency), StaticId>::default();

        let mut crs_curve_map = FxHashMap::<Currency, StaticId>::default();
        // "KOSPI2" -> "KRWGOV"
        // "SPX" -> "USGOV"
        // RateIndexCode::CD -> "KRWIRS"
        collateral_curve_map.insert(kospi_und_id, StaticId::from_str("KRWGOV", "KAP"));
        collateral_curve_map.insert(spx_und_id, StaticId::from_str("USGOV", "KAP"));
        rate_index_curve_map.insert(rate_id, StaticId::from_str("KRWIRS", "KAP"));
        crs_curve_map.insert(Currency::KRW, StaticId::from_str("KRWCRS", "KAP"));
        crs_curve_map.insert(Currency::USD, StaticId::from_str("USDCRS", "KAP"));

        let funding_cost_map = FxHashMap::<Currency, StaticId>::default();
        let crs_curve_map = FxHashMap::<Currency, StaticId>::default();
        let match_parameter = MatchParameter::new(
            collateral_curve_map,
            borrowing_curve_map,
            bond_curve_map,
            crs_curve_map,
            rate_index_curve_map,
            funding_cost_map,
        );

        // test get_all_underlying_codes
        let underlying_codes = instruments.get_all_underlying_ids();
        assert_eq!(
            underlying_codes,
            vec![
                StaticId::from_str("KOSPI2", "KRX"),
                StaticId::from_str("SPX", "CME"),
            ]
        );
        // test instruments_with_underlying
        let instruments_with_kospi2 =
            instruments.instruments_with_underlying(
                StaticId::from_str("KOSPI2", "KRX"),
                None);

        assert_eq!(fut1.get_code(), instruments_with_kospi2[0].get_code());
        assert_eq!(fut1.get_name(), instruments_with_kospi2[0].get_name());
        assert_eq!(
            fut1.get_currency(),
            instruments_with_kospi2[0].get_currency()
        );

        // test get_all_curve_names
        let all_curve_ids = instruments.get_all_curve_ids(&match_parameter)?;
        assert_eq!(
            all_curve_ids, 
            vec![
                StaticId::from_str("KRWGOV", "KAP"),
                StaticId::from_str("USGOV", "KAP"),
                StaticId::from_str("KRWIRS", "KAP"),
            ]   
        );
          
        // test instruments_using_curve
        let instruments_using_krw_gov =
            instruments.instruments_using_curve(StaticId::from_str("KRWGOV", "KAP"), &match_parameter, None)?;

        assert_eq!(fut1.get_code(), instruments_using_krw_gov[0].get_code());

        // test discount curve
        let instruments_using_krw_irs =
            instruments.instruments_using_curve(
                StaticId::from_str("KRWIRS", "KAP"),
                &match_parameter, 
                None)?;

        assert_eq!(irs.get_code(), instruments_using_krw_irs[0].get_code());

        // test instruments_with_currency
        let instruments_with_krw = instruments.instruments_with_currency(Currency::KRW);
        assert_eq!(fut1.get_code(), instruments_with_krw[0].get_code());
        assert_eq!(irs.get_code(), instruments_with_krw[1].get_code());

        // test instruments_with_type
        let instruments_with_equity_futures = instruments.instruments_with_types(vec!["Futures"]);
        assert_eq!(
            fut1.get_code(),
            instruments_with_equity_futures[0].get_code()
        );
        assert_eq!(
            fut2.get_code(),
            instruments_with_equity_futures[1].get_code()
        );

        let instruments_with_irs = instruments.instruments_with_types(vec!["IRS"]);
        assert_eq!(irs.get_code(), instruments_with_irs[0].get_code());

        // test instruments_with_maturity_upto
        let instruments_with_maturity_upto = instruments.instruments_with_maturity_upto(
            None,
            &datetime!(2022-12-01 09:00:00 UTC),
            None,
        );
        assert_eq!(
            fut1.get_code(),
            instruments_with_maturity_upto[0].get_code()
        );
        assert_eq!(irs.get_code(), instruments_with_maturity_upto[1].get_code());

        Ok(())
    }
}
