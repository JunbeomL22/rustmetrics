use crate::currency::FxCode;
use crate::enums::VanillaOptionCalculationMethod;
use crate::evaluation_date::EvaluationDate;
use crate::instrument::{Instrument, InstrumentTrait};
use crate::parameters::{market_price::MarketPrice, past_price::DailyClosePrice};
use crate::parameters::{
    quanto::Quanto, rate_index::RateIndex, volatility::Volatility, zero_curve::ZeroCurve,
};
use crate::pricing_engines::calculation_configuration::CalculationConfiguration;
use crate::pricing_engines::{
    bond_pricer::BondPricer, futures_pricer::FuturesPricer, fx_futures_pricer::FxFuturesPricer,
    identity_pricer::IdentityPricer, ktbf_pricer::KtbfPricer, match_parameter::MatchParameter,
    option_analytic_pricer::OptionAnalyticPricer, plain_swap_pricer::PlainSwapPricer,
    pricer::Pricer, unit_pricer::UnitPricer,
};
//
use static_id::StaticId;
use std::{cell::RefCell, rc::Rc};
use rustc_hash::FxHashMap;

use anyhow::{anyhow, Result};

/// dividend is not needed for this pricer factory
/// dividend is in herent in equities
pub struct PricerFactory {
    evaluation_date: Rc<RefCell<EvaluationDate>>,
    fxs: FxHashMap<FxCode, Rc<RefCell<MarketPrice>>>,
    equities: FxHashMap<StaticId, Rc<RefCell<MarketPrice>>>,
    zero_curves: FxHashMap<StaticId, Rc<RefCell<ZeroCurve>>>,
    underlying_volatilities: FxHashMap<StaticId, Rc<RefCell<Volatility>>>,
    quantos: FxHashMap<(StaticId, FxCode), Rc<RefCell<Quanto>>>, // (underlying_code, fx_code) -> Quanto
    past_close_data: FxHashMap<StaticId, Rc<DailyClosePrice>>,
    match_parameter: Rc<MatchParameter>,
    calculation_configuration: Rc<CalculationConfiguration>,
}

impl PricerFactory {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        evaluation_date: Rc<RefCell<EvaluationDate>>,
        fxs: FxHashMap<FxCode, Rc<RefCell<MarketPrice>>>,
        equities: FxHashMap<StaticId, Rc<RefCell<MarketPrice>>>,
        zero_curves: FxHashMap<StaticId, Rc<RefCell<ZeroCurve>>>,
        underlying_volatilities: FxHashMap<StaticId, Rc<RefCell<Volatility>>>,
        quantos: FxHashMap<(StaticId, FxCode), Rc<RefCell<Quanto>>>,
        past_close_data: FxHashMap<StaticId, Rc<DailyClosePrice>>,
        match_parameter: Rc<MatchParameter>,
        calculation_configuration: Rc<CalculationConfiguration>,
    ) -> PricerFactory {
        PricerFactory {
            evaluation_date,
            fxs,
            equities,
            zero_curves,
            underlying_volatilities,
            quantos,
            past_close_data,
            match_parameter,
            calculation_configuration,
        }
    }

    pub fn create_pricer(&self, instrument: &Rc<Instrument>) -> Result<Pricer> {
        let pricer = match Rc::as_ref(instrument) {
            Instrument::Futures(_) => self.get_futures_pricer(instrument)?,
            Instrument::VanillaOption(_) => self.get_vanilla_option_pricer(instrument)?,
            Instrument::Bond(_) => self.get_bond_pricer(instrument)?,
            Instrument::KTBF(_) => self.get_ktbf_pricer(instrument)?,
            Instrument::FxFutures(_) => self.get_fx_futures_pricer(instrument)?,
            Instrument::PlainSwap(_) => self.get_plain_swap_pricer(instrument)?,
            Instrument::Stock(_) => self.get_stock_pricer(instrument)?,
            Instrument::Cash(_) => self.get_cash_pricer(instrument)?,
            //
            //
            _ => {
                return Err(anyhow!(
                    "({}:{})   pricer for {} ({}) is not implemented yet",
                    file!(),
                    line!(),
                    instrument.get_id(),
                    instrument.get_type_name(),
                ));
            }
        };
        Ok(pricer)
    }

    fn get_bond_pricer(&self, instrument: &Rc<Instrument>) -> Result<Pricer> {
        let discount_curve_id = self.match_parameter.get_discount_curve_id(instrument)?;
        let discount_curve = self
            .zero_curves
            .get(&discount_curve_id)
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "{}:{} (PricerFactory::get_bond_pricer)\n\
                    failed to get discount curve of {}. self.zero_curves ({:?}) does not have {:?}",
                    file!(),
                    line!(),
                    instrument.get_id(),
                    self.zero_curves.keys(),
                    discount_curve_id,
                )
            })?
            .clone();

        let rate_index: Option<&RateIndex> = instrument.get_rate_index()?;
        let forward_curve = match rate_index {
            None => {
                // the case of fixed coupon bond
                None
            }
            Some(_) => {
                let forward_curve_id = self.match_parameter.get_rate_index_curve_id(instrument)?;
                let res = self
                    .zero_curves
                    .get(&forward_curve_id)
                    .ok_or_else(|| {
                        anyhow::anyhow!(
                            "failed to get forward curve of {}.\nself.zero_curves does not have {}",
                            instrument.get_id(),
                            forward_curve_id,
                        )
                    })?
                    .clone();
                Some(res)
            }
        }; // the end of the forward curve construction which is optional

        let past_fixing_data = match rate_index {
            None => None,
            Some(rate_index) => {
                let past_fixing_data = self.past_close_data.get(&rate_index.get_id())
                    .ok_or_else(
                        || anyhow::anyhow!(
                            "failed to get past fixing data of {}.\nself.past_close_data does not have {}",
                            instrument.get_id(),
                            rate_index.get_id(),
                        ))?.clone();
                Some(past_fixing_data)
            }
        }; // the end of the past fixing data construction which is optional

        let core = BondPricer::new(
            self.evaluation_date.clone(),
            discount_curve,
            forward_curve,
            past_fixing_data,
        );
        Ok(Pricer::BondPricer(core))
    }
    
    fn get_futures_pricer(&self, instrument: &Rc<Instrument>) -> Result<Pricer> {
        let underlying_ids = instrument.get_underlying_ids();
        let equity = self.equities.get(&underlying_ids[0]).unwrap().clone();
        let collatral_curve_id = self.match_parameter.get_collateral_curve_ids(instrument)?[0];
        let borrowing_curve_id = self.match_parameter.get_borrowing_curve_ids(instrument)?[0];
        let core = FuturesPricer::new(
            //self.evaluation_date.clone(),
            equity,
            self.zero_curves
                .get(&collatral_curve_id)
                .ok_or_else(|| {
                    anyhow::anyhow!(
                        "failed to get collateral curve of {}.\nself.zero_curves does not have {}",
                        instrument.get_id(),
                        collatral_curve_id,
                    )
                })?
                .clone(),
            self.zero_curves
                .get(&borrowing_curve_id)
                .ok_or_else(|| {
                    anyhow::anyhow!(
                        "failed to get borrowing curve of {}.\nself.zero_curves does not have {}",
                        instrument.get_id(),
                        borrowing_curve_id,
                    )
                })?
                .clone(),
        );
        Ok(Pricer::FuturesPricer(core))
    }

    fn get_vanilla_option_pricer(&self, instrument: &Rc<Instrument>) -> Result<Pricer> {
        let equity = self
            .equities
            .get(&instrument.get_underlying_ids()[0])
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "({}:{}) failed to get equity of {}.\nself.equities does not have {}",
                    file!(),
                    line!(),
                    instrument.get_id(),
                    instrument.get_underlying_ids()[0],
                )
            })?
            .clone();
        let volatility = self.underlying_volatilities.get(&instrument.get_underlying_ids()[0])
            .ok_or_else(|| anyhow::anyhow!(
                "({}:{}) failed to get volatility of {}.\nself.equity_volatilities does not have {}",
                file!(), line!(), instrument.get_id(), instrument.get_underlying_ids()[0],
            ))?.clone();
        let discount_curve_id = self.match_parameter.get_discount_curve_id(instrument)?;
        let discount_curve = self
            .zero_curves.get(&discount_curve_id)
            .ok_or_else(|| {
                anyhow::anyhow!(
                "({}:{}) failed to get discount curve of {}.\nself.zero_curves does not have {}",
                file!(), line!(), instrument.get_id(), discount_curve_id,
            )
            })?
            .clone();

        let collateral_curve_id = self
            .match_parameter
            .get_collateral_curve_ids(instrument)?[0];
        let collatral_curve = self
            .zero_curves
            .get(&collateral_curve_id)
            .ok_or_else(|| {
                anyhow::anyhow!(
                "({}:{}) failed to get collateral curve of {}.\nself.zero_curves does not have {}",
                file!(), line!(), instrument.get_id(), collateral_curve_id,
            )
            })?
            .clone();
        let borrowing_curve_id = self.match_parameter.get_borrowing_curve_ids(instrument)?[0];
        let borrowing_curve = self
            .zero_curves
            .get(&borrowing_curve_id)
            .ok_or_else(|| {
                anyhow::anyhow!(
                "({}:{}) failed to get borrowing curve of {}.\nself.zero_curves does not have {}",
                file!(), line!(), instrument.get_id(), borrowing_curve_id,
            )
            })?
            .clone();

        let curr = instrument.get_currency();
        let und_curr = instrument.get_underlying_currency()?;
        let quanto = match und_curr == curr {
            false => {
                let fx_code = FxCode::new(und_curr, curr);
                let underlying_code = instrument.get_underlying_ids()[0];
                let key = (underlying_code, fx_code);
                let quanto =
                    self.quantos
                        .get(&key)
                        .ok_or_else(|| {
                            anyhow::anyhow!(
                        "({}:{}) failed to get quanto of {}.\nself.quantos does not have {:?}",
                        file!(), line!(), instrument.get_id(), key,
                    )
                        })?
                        .clone();
                Some(quanto)
            }
            true => None,
        };
        let core = match self
            .calculation_configuration
            .get_vanilla_option_calculation_method()
        {
            VanillaOptionCalculationMethod::Analytic => OptionAnalyticPricer::new(
                self.evaluation_date.clone(),
                equity,
                collatral_curve,
                borrowing_curve,
                discount_curve,
                volatility,
                quanto,
            ),
            _ => return Err(anyhow::Error::msg("Unsupported calculation method")),
        };
        Ok(Pricer::OptionAnalyticPricer(core))
    }

    fn get_ktbf_pricer(&self, instrument: &Rc<Instrument>) -> Result<Pricer> {
        let discount_curve_id = StaticId::from_str("KRWGOV", "KAP");
        let discount_curve = self
            .zero_curves
            .get(&discount_curve_id)
            .ok_or_else(|| {
                anyhow::anyhow!(
                "({}:{}) failed to get discount curve of {}.\nself.zero_curves does not have {}",
                file!(), line!(), instrument.get_id(), discount_curve_id,
            )
            })?
            .clone();
        let collateral_curve_id = self.match_parameter.get_collateral_curve_ids(instrument)?[0];
        let collateral_curve = self.zero_curves.get(&collateral_curve_id)
            .ok_or_else(|| {
                anyhow::anyhow!(
                "({}:{}) failed to get collateral curve of {}.\nself.zero_curves does not have {}",
                file!(), line!(), instrument.get_id(), collateral_curve_id,
            )})?.clone();
        let core = KtbfPricer::new(
            self.evaluation_date.clone(),
            discount_curve,
            collateral_curve,
        );

        Ok(Pricer::KtbfPricer(core))
    }

    fn get_fx_futures_pricer(&self, instrument: &Rc<Instrument>) -> Result<Pricer> {
        let fx_code = instrument.get_fxfutres_und_fxcode()?;

        let fx = self
            .fxs
            .get(&fx_code)
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "({}:{}) failed to get FX of {}.\nself.fxs does not have {:?}",
                    file!(),
                    line!(),
                    instrument.get_id(),
                    fx_code,
                )
            })?
            .clone();
        let underlying_currency_curve_id = self.match_parameter.get_floating_crs_curve_id(instrument)?;
        let underlying_currency_curve = self.zero_curves.get(&underlying_currency_curve_id)
            .ok_or_else(|| anyhow::anyhow!(
                "({}:{}) failed to get underlying currency curve of {}.\nself.zero_curves does not have {}",
                file!(), line!(), instrument.get_id(), underlying_currency_curve_id,
            ))?.clone();
        let futures_currency_curve_id = self.match_parameter.get_crs_curve_id(instrument)?;
        let futures_currency_curve = self.zero_curves.get(&futures_currency_curve_id)
            .ok_or_else(|| anyhow::anyhow!(
                "({}:{}) failed to get futures currency curve of {}.\nself.zero_curves does not have {}",
                file!(), line!(), instrument.get_id(), futures_currency_curve_id,
            ))?.clone();

        let core = FxFuturesPricer::new(
            //self.evaluation_date.clone(),
            fx,
            underlying_currency_curve,
            futures_currency_curve,
        );
        Ok(Pricer::FxFuturesPricer(core))
    }

    fn get_plain_swap_pricer(&self, instrument: &Rc<Instrument>) -> Result<Pricer> {
        let fixed_leg_discount_curve_id = self.match_parameter.get_crs_curve_id(instrument)?;
        let fixed_leg_discount_curve = self.zero_curves.get(&fixed_leg_discount_curve_id)
            .ok_or_else(|| anyhow::anyhow!(
                "({}:{}) failed to get fixed leg discount curve of {}.\nself.zero_curves does not have {}",
                file!(), line!(), instrument.get_id(), fixed_leg_discount_curve_id,
            ))?.clone();

        let floating_leg_discount_curve_id = self
            .match_parameter
            .get_floating_crs_curve_id(instrument)?;
        let floating_leg_discount_curve = self.zero_curves.get(&floating_leg_discount_curve_id)
            .ok_or_else(|| anyhow::anyhow!(
                "({}:{}) failed to get floating leg discount curve of {}.\nself.zero_curves does not have {}",
                file!(), line!(), instrument.get_id(), floating_leg_discount_curve_id,
            ))?.clone();

        let rate_index = instrument.get_rate_index()?;
        let forward_curve = match rate_index {
            Some(_) => {
                let forward_curve_id = self.match_parameter.get_rate_index_curve_id(instrument)?;
                let res = self.zero_curves.get(&forward_curve_id)
                    .ok_or_else(|| anyhow::anyhow!(
                        "({}:{}) failed to get forward curve of {}.\nself.zero_curves does not have {}",
                        file!(), line!(), instrument.get_id(), forward_curve_id,
                    ))?.clone();
                Some(res)
            }
            None => None,
        };

        let past_fixig_data = match rate_index {
            Some(rate_index) => {
                let past_fixing_data = self.past_close_data.get(&rate_index.get_id())
                    .ok_or_else(|| anyhow::anyhow!(
                        "({}:{}) failed to get past fixing data of {}.\nself.past_close_data does not have {}",
                        file!(), line!(), instrument.get_code_str(), rate_index.get_rate_index_code_str(),
                    ))?.clone();
                Some(past_fixing_data)
            }
            None => None,
        };

        let fx_code = instrument.get_floating_to_fixed_fxcode()?;
        let floating_to_fixed_fx = match fx_code {
            None => None,
            Some(fx_code) => {
                let fx = self
                    .fxs
                    .get(&fx_code)
                    .ok_or_else(|| {
                        anyhow::anyhow!(
                            "({}:{}) failed to get FX of {}.\nself.fxs does not have {:?}",
                            file!(),
                            line!(),
                            instrument.get_id(),
                            fx_code,
                        )
                    })?
                    .clone();
                Some(fx)
            }
        };

        let core = PlainSwapPricer::new(
            self.evaluation_date.clone(),
            fixed_leg_discount_curve,
            floating_leg_discount_curve,
            forward_curve,
            past_fixig_data,
            floating_to_fixed_fx,
        )?;

        Ok(Pricer::PlainSwapPricer(core))
    }

    fn get_stock_pricer(&self, instrument: &Rc<Instrument>) -> Result<Pricer> {
        let equity = self
            .equities
            .get(&instrument.get_id())
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "({}:{}) failed to get equity of {}",
                    file!(),
                    line!(),
                    instrument.get_id(),
                )
            })?
            .clone();
        let core = IdentityPricer::new(equity);
        Ok(Pricer::IdentityPricer(core))
    }

    fn get_cash_pricer(&self, _instrument: &Rc<Instrument>) -> Result<Pricer> {
        let core = UnitPricer::new();
        Ok(Pricer::UnitPricer(core))
    }
}
