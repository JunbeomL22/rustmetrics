//! # rustmetrics
//!
//! `rustmetrics` is a Rust crate for pricing financial instruments and calculating various risk metrics.
//! This crate aims to provide efficient and optimized calculations for a wide range of financial risk measures.
//! 
//! ## Contributing
//! 
//! We welcome all contributions! Whether it's comments, minor changes, or structural code modifications, we're eager to review your input. Here's how you can contribute:
//! 
//! - Open an issue for any bugs you find or features you'd like to see.
//! - Submit a pull request for any improvements you've made.
//! - Provide feedback on existing issues and pull requests.
//! 
//! No contribution is too small, and all input is valued. Let's work together to make this project even better!
//! 
//! ## Features
//!
//! - Pricing and risk calculation for plain financial instruments:
//!   - Bonds
//!   - Cross-Currency Swaps (CRS)
//!   - Interest Rate Swaps (IRS)
//!   - Futures
//!   - Vanilla Options
//!   - Korea Treasury Bond Futures (KTBF)
//!   - FX Swaps / Forwards / Spots
//!
//! - Calculation of key risk metrics:
//!   - Delta
//!   - Gamma
//!   - Theta
//!   - Vega
//!   - Vega Structure
//!   - Vega Matrix
//!   - Rho
//!   - Rho Structure
//!   - Dividend Delta
//!   - Dividend Structure
//!
//! ## Design Philosophy
//!
//! The core design of `rustmetrics` focuses on optimizing calculations by reducing repetitive and redundant operations.
//! This is achieved through two main components:
//!
//! 1. **InstrumentCategory**: Categorizes financial instruments based on their characteristics.
//! 2. **EngineGenerator**: Splits instruments into groups and creates specialized engines for each group.
//!
//! This design provides a solid foundation for future expansions, particularly for implementing numerical
//! simulations like Finite Difference Methods (FDM) and Monte Carlo simulations.
//!
//! ## Crate Structure
//!
//! - `data`: Raw market observations, shared by Engine object in multi-thread environment.
//! - `parameters`: Objects generated from data objects for actual calculation.
//! - `instruments`: Defines various financial instruments (e.g., Futures, FxFutures, VanillaOption, IRS, CCS, Bond).
//! - `time`: Handles calendars, conventions, and holiday calculations.
//! - `pricing_engines`: Contains Engine, EngineGenerator, and Pricer implementations.
//!
//! ## Usage
//!
//! For standard examples, see the tests in `./tests/engine.rs`. The following example is a simplified version of the
//! 
//! # RustMetrics
//!
//! `rustmetrics` is a Rust crate for pricing financial instruments and calculating various risk metrics.
//!
//! ## Example Usage
//!
//! Here's a simplified example of how to use `rustmetrics`:
//!
//! ```rust
//! use rustmetrics::*;
//! use time::macros::datetime;
//! use anyhow::Result;
//!
//! fn main() -> Result<()> {
//!     /* Set up the evaluation date */
//!     let evaluation_date = datetime!(2024-03-13 16:30:00 +09:00);
//!
//!     /* Create market data (zero curves, equity volatility, FX rates, dividends, etc.) */
//!     let zero_curve_map = create_zero_curve_map();
//!     let equity_vol_map = create_equity_vol_map();
//!     let fx_data_map = create_fx_data_map();
//!     let dividend_data_map = create_dividend_data_map();
//!
//!     /* Create financial instruments */
//!     let instruments = vec![
//!         Rc::new(Instrument::Futures(create_stock_futures())),
//!         Rc::new(Instrument::Bond(create_bond())),
//!         Rc::new(Instrument::VanillaOption(create_vanilla_option())),
//!         Rc::new(Instrument::Cash(create_cash())),
//!         Rc::new(Instrument::Stock(create_stock())),
//!     ];
//!
//!     /* Set up calculation configuration */
//!     let calculation_configuration = CalculationConfiguration::default()
//!         .with_delta_calculation(true)
//!         .with_gamma_calculation(true)
//!         .with_theta_calculation(true)
//!         /* ... other risk measures ... */;
//!
//!     /* Create EngineGenerator and perform calculations */
//!     let mut engine_generator = EngineGenerator::builder()
//!         .with_configuration(calculation_configuration, evaluation_date, create_match_parameter())?
//!         .with_instruments(Instruments::new(instruments))?
//!         .with_instrument_categories(create_instrument_categories())?
//!         .with_data(
//!             fx_data_map,
//!             create_stock_data_map(),
//!             zero_curve_map,
//!             dividend_data_map,
//!             equity_vol_map,
//!             Default::default(),
//!             Default::default(),
//!             Default::default(),
//!             Default::default(),
//!         )?
//!         .build()?;
//!
//!     engine_generator.distribute_instruments()?;
//!     engine_generator.calculate()?;
//!
//!     /* Retrieve and process calculation results */
//!     let calculation_results = engine_generator.get_calculation_results();
//!     for (instrument_id, result) in calculation_results.iter() {
//!         println!("Instrument: {}, NPV: {:?}", instrument_id, result.get_npv_result());
//!     }
//!
//!     Ok(())
//! }
//!
//! /* Example helper functions to create market data and instruments (there are no such things in this project) */
//! fn create_zero_curve_map() -> FxHashMap<StaticId, VectorData> { /* ... */ }
//! fn create_equity_vol_map() -> FxHashMap<StaticId, ValueData> { /* ... */ }
//! fn create_fx_data_map() -> FxHashMap<FxCode, ValueData> { /* ... */ }
//! fn create_dividend_data_map() -> FxHashMap<StaticId, VectorData> { /* ... */ }
//! fn create_stock_futures() -> Futures { /* ... */ }
//! fn create_bond() -> Bond { /* ... */ }
//! fn create_vanilla_option() -> VanillaOption { /* ... */ }
//! fn create_cash() -> Cash { /* ... */ }
//! fn create_stock() -> Stock { /* ... */ }
//! fn create_match_parameter() -> MatchParameter { /* ... */ }
//! fn create_instrument_categories() -> Vec<InstrumentCategory> { /* ... */ }
//! fn create_stock_data_map() -> FxHashMap<StaticId, ValueData> { /* ... */ }
//! ```
//!
//! This example demonstrates the basic workflow of using `rustmetrics`:
//! 1. Set up market data
//! 2. Create financial instruments
//! 3. Configure calculation parameters
//! 4. Use EngineGenerator to perform calculations
//! 5. Retrieve and process results
//!
//! For more detailed examples and API documentation, please refer to the individual module and struct documentation.
//!
//! ## Status and Future Plans
//!
//! This crate is currently in its first prototype stage. While it can handle basic plain instruments,
//! it is actively under development with plans for expansion and optimization, including:
//!
//! - Implementing more complex financial instruments
//! - Adding support for numerical simulations (FDM, Monte Carlo)
//! - Optimizing performance for large-scale calculations
//! - Expanding the range of supported risk metrics
//!
//! ## License
//!
//! This project is dual-licensed under Apache License, Version 2.0 and MIT License.
pub mod definitions;
pub mod instrument;
pub mod instruments;
pub mod math;
pub mod parameters;
pub mod time;
pub mod util;
pub mod utils;
pub mod currency;
pub mod data;
pub mod enums;
pub mod evaluation_date;
pub mod pricing_engines;
#[macro_use]
pub mod macros;

pub use definitions::{Real, Time};
pub use utils::find_index::{binary_search_index, vectorized_search_index_for_sorted_vector};
pub use utils::find_index_ndarray::{
    binary_search_index_ndarray, vectorized_search_index_for_sorted_ndarray,
};

pub use instrument::{Instrument, Instruments};
pub use crate::instruments::{
    InstType,
    inst_info::InstInfo,
    AccountingLevel,
};

pub use crate::utils::string_arithmetic::{
    add_period,
    sub_period,
};
pub use crate::time::period::{
    Period,
    FinerPeriod,
    Tenor,
};

pub use crate::time::calendars::nullcalendar::NullCalendar;
use once_cell::sync::Lazy;
static NULL_CALENDAR: Lazy<NullCalendar> = Lazy::new(NullCalendar::default);

pub use crate::enums::{
    CreditRating,
    RankType,
    IssuerType,
    StockRankType,
};
pub use crate::instrument::InstrumentTrait;

pub use crate::currency::{
    Currency,
    FxCode,
};
pub use crate::instruments::bond::BondInfo;
