use crate::currency::{Currency, FxCode};
use crate::data::{
    daily_value_data::DailyValueData, surface_data::SurfaceData, value_data::ValueData,
    vector_data::VectorData,
};
use crate::evaluation_date::EvaluationDate;
use crate::instrument::{Instrument, InstrumentTrait, Instruments};
use crate::pricing_engines::{
    calculation_configuration::CalculationConfiguration, calculation_result::CalculationResult,
    engine::Engine, match_parameter::MatchParameter,
};
//
use anyhow::{anyhow, Result};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use time::OffsetDateTime;
use static_id::StaticId;
use rustc_hash::FxHashMap;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct InstrumentCategory {
    pub type_names: Option<Vec<String>>,
    pub currency: Option<Vec<Currency>>,
    pub underlying_ids: Option<Vec<StaticId>>,
}

impl InstrumentCategory {
    pub fn new(
        type_names: Option<Vec<String>>,
        currency: Option<Vec<Currency>>,
        underlying_ids: Option<Vec<StaticId>>,
    ) -> InstrumentCategory {
        InstrumentCategory {
            type_names,
            currency,
            underlying_ids,
        }
    }

    pub fn contains(&self, instrument: &Instrument) -> Result<bool> {
        let instrument_type_inp = instrument.get_type_name().to_owned();
        let currency_inp = instrument.get_currency();
        let underlying_ids_inp = instrument.get_underlying_ids();

        let mut res: bool = true;
        // check instrument type is in type_names
        if let Some(type_names) = &self.type_names {
            if !type_names.contains(&instrument_type_inp) {
                res = false;
            }
        }
        // check currency is in currency
        if let Some(currency) = &self.currency {
            if !currency.contains(&currency_inp) {
                res = false;
            }
        }
        // check underlying codes are the same (not inclusion)
        if let Some(underlying_ids) = &self.underlying_ids {
            if !underlying_ids_inp.is_empty()
                && underlying_ids.to_vec() != underlying_ids_inp
            {
                res = false;
            }
        }
        // check the utc_offset
        Ok(res)
    }
}

pub struct EngineGenerator {
    instruments: Instruments,
    instrument_group_vec: Vec<Vec<Instrument>>,
    instrument_categories: Vec<InstrumentCategory>,
    //
    calculation_configuration: CalculationConfiguration,
    match_parameter: MatchParameter,
    //
    calculation_results: FxHashMap<StaticId, CalculationResult>,
    // evaluation date
    evaluation_date: EvaluationDate,
    // data
    fx_data: Arc<FxHashMap<FxCode, ValueData>>,
    stock_data: Arc<FxHashMap<StaticId, ValueData>>,
    curve_data: Arc<FxHashMap<StaticId, VectorData>>,
    dividend_data: Arc<FxHashMap<StaticId, VectorData>>,
    equity_constant_volatility_data: Arc<FxHashMap<StaticId, ValueData>>,
    equity_volatility_surface_data: Arc<FxHashMap<StaticId, SurfaceData>>,
    fx_constant_volatility_data: Arc<FxHashMap<FxCode, ValueData>>,
    quanto_correlation_data: Arc<FxHashMap<(StaticId, FxCode), ValueData>>,
    past_daily_value_data: Arc<FxHashMap<StaticId, DailyValueData>>,
}

impl Default for EngineGenerator {
    fn default() -> Self {
        EngineGenerator {
            instruments: Instruments::default(),
            instrument_group_vec: vec![],
            instrument_categories: vec![],
            //
            calculation_configuration: CalculationConfiguration::default(),
            match_parameter: MatchParameter::default(),
            //
            calculation_results: FxHashMap::default(),
            //
            evaluation_date: EvaluationDate::default(),
            //
            fx_data: Arc::new(FxHashMap::default()),
            stock_data: Arc::new(FxHashMap::default()),
            curve_data: Arc::new(FxHashMap::default()),
            dividend_data: Arc::new(FxHashMap::default()),
            equity_constant_volatility_data: Arc::new(FxHashMap::default()),
            equity_volatility_surface_data: Arc::new(FxHashMap::default()),
            fx_constant_volatility_data: Arc::new(FxHashMap::default()),
            quanto_correlation_data: Arc::new(FxHashMap::default()),
            past_daily_value_data: Arc::new(FxHashMap::default()),
        }
    }
}

impl EngineGenerator {
    pub fn builder() -> EngineGenerator {
        EngineGenerator::default()
    }

    pub fn with_configuration(
        &mut self,
        calculation_configuration: CalculationConfiguration,
        evalutation_datetime: OffsetDateTime,
        match_parameter: MatchParameter,
    ) -> Result<&mut Self> {
        self.calculation_configuration = calculation_configuration;
        self.evaluation_date = EvaluationDate::new(evalutation_datetime);
        self.match_parameter = match_parameter;
        Ok(self)
    }

    pub fn with_instruments(&mut self, instruments: Instruments) -> Result<&mut Self> {
        self.instruments = instruments;
        Ok(self)
    }

    pub fn with_instrument_categories(
        &mut self,
        instrument_categories: Vec<InstrumentCategory>,
    ) -> Result<&mut Self> {
        self.instrument_categories = instrument_categories;
        Ok(self)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn with_data(
        &mut self,
        fx_data: FxHashMap<FxCode, ValueData>,
        stock_data: FxHashMap<StaticId, ValueData>,
        curve_data: FxHashMap<StaticId, VectorData>,
        dividend_data: FxHashMap<StaticId, VectorData>,
        equity_constant_volatility_data: FxHashMap<StaticId, ValueData>,
        equity_volatility_surface_data: FxHashMap<StaticId, SurfaceData>,
        fx_constant_volatility_data: FxHashMap<FxCode, ValueData>,
        quanto_correlation_data: FxHashMap<(StaticId, FxCode), ValueData>,
        past_daily_value_data: FxHashMap<StaticId, DailyValueData>,
    ) -> Result<&mut Self> {
        self.fx_data = Arc::new(fx_data);
        self.stock_data = Arc::new(stock_data);
        self.curve_data = Arc::new(curve_data);
        self.dividend_data = Arc::new(dividend_data);
        self.equity_constant_volatility_data = Arc::new(equity_constant_volatility_data);
        self.equity_volatility_surface_data = Arc::new(equity_volatility_surface_data);
        self.fx_constant_volatility_data = Arc::new(fx_constant_volatility_data);
        self.quanto_correlation_data = Arc::new(quanto_correlation_data);
        self.past_daily_value_data = Arc::new(past_daily_value_data);
        Ok(self)
    }

    pub fn distribute_instruments(&mut self) -> Result<()> {
        let mut distribution_checker: Vec<bool> = vec![false; self.instruments.len()];

        let mut instrument_group_vec: Vec<Vec<Instrument>> = vec![];
        for instrument_category in &self.instrument_categories {
            let mut instrument_group: Vec<Instrument> = vec![];
            for (inst_id, instrument) in self.instruments.iter().enumerate() {
                if !distribution_checker[inst_id] && instrument_category.contains(instrument)? {
                    instrument_group.push(instrument.as_ref().clone());
                    distribution_checker[inst_id] = true;
                }
            }
            if !instrument_group.is_empty() {
                instrument_group_vec.push(instrument_group);
            }
        }

        let mut inst_name_msgs: Vec<String> = vec![];
        for (inst_id, is_distributed) in distribution_checker.iter().enumerate() {
            if !is_distributed {
                let msg = format!(
                    "{} ({})\n\
                    type: {}\n\
                    currency: {}\n\
                    underlying_ids: {:?}\n",
                    self.instruments[inst_id].get_name(),
                    self.instruments[inst_id].get_code_str(),
                    self.instruments[inst_id].get_type_name(),
                    self.instruments[inst_id].get_currency(),
                    self.instruments[inst_id].get_underlying_ids(),
                );

                inst_name_msgs.push(msg);
            }
        }

        if !inst_name_msgs.is_empty() {
            return Err(anyhow!(
                "The following instruments are not distributed:\n{}",
                inst_name_msgs.join("\n"),
            ));
        }

        self.instrument_group_vec = instrument_group_vec;

        Ok(())
    }

    /// spawn threads to create engine and calculate
    pub fn calculate(&mut self) -> Result<()> {
        let shared_results = Arc::new(Mutex::new(FxHashMap::<StaticId, CalculationResult>::default()));
        let dt = self.evaluation_date.get_date_clone();
        let calc_res: Result<()> = self
            .instrument_group_vec
            .par_iter()
            .enumerate()
            .map(|(group_id, instrument_group)| {
                let engine = Engine::builder(
                    group_id,
                    self.calculation_configuration.clone(),
                    dt,
                    self.match_parameter.clone(),
                );

                let engine = match engine.with_instruments(instrument_group.clone()) {
                    Ok(engine) => engine,
                    Err(e) => return Err(e),
                };

                let mut engine = match engine.with_parameter_data(
                    self.fx_data.clone(),
                    self.stock_data.clone(),
                    self.curve_data.clone(),
                    self.dividend_data.clone(),
                    self.equity_constant_volatility_data.clone(),
                    self.equity_volatility_surface_data.clone(),
                    self.fx_constant_volatility_data.clone(),
                    self.quanto_correlation_data.clone(),
                    self.past_daily_value_data.clone(),
                ) {
                    Ok(engine) => engine,
                    Err(e) => return Err(e),
                };

                engine.initialize_pricers()?;
                engine.calculate()?;

                let result = engine.get_calculation_result();
                let mut mut_res = shared_results.lock().unwrap();

                for (key, value) in result.iter() {
                    mut_res.insert(*key, value.borrow().clone());
                }

                Ok(())
            })
            .collect();

        //self.calculation_results = shared_results.lock().unwrap().clone();
        self.calculation_results
            .clone_from(&shared_results.lock().unwrap());

        match calc_res {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub fn get_calculation_results(&self) -> &FxHashMap<StaticId, CalculationResult> {
        &self.calculation_results
    }
}
