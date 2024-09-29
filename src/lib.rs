//! # rustmetrics
//!
//! `rustmetrics` is a Rust crate for pricing financial instruments and calculating various risk metrics.
//! It aims to provide efficient and optimized calculations for a wide range of financial risk measures.
//!
//! ## Features
//!
//! - Pricing and risk calculation for plain financial instruments:
//!   - Bonds, Cross-Currency Swaps (CRS), Interest Rate Swaps (IRS), Futures,
//!     Vanilla Options, Korea Treasury Bond Futures (KTBF), FX Swaps, FX Forwards, FX Spots
//!
//! - Calculation of key risk metrics:
//!   - Delta, Gamma, Theta, Vega, Vega Structure, Vega Matrix, Rho, Rho Structure,
//!     Dividend Delta, Dividend Structure
//!
//! ## Design Philosophy
//!
//! The core design focuses on optimizing calculations by reducing repetitive and redundant operations
//! through two main components:
//!
//! 1. **InstrumentCategory**: Categorizes financial instruments based on their characteristics.
//! 2. **EngineGenerator**: Splits instruments into groups and creates specialized engines for each group.
//!
//! This design provides a foundation for future expansions, particularly for implementing numerical
//! simulations like Finite Difference Methods (FDM) and Monte Carlo simulations.
//!
//! ## Crate Structure
//!
//! - `data`: Raw market observations
//! - `parameters`: Objects generated from data objects for actual calculation
//! - `instruments`: Financial instruments (e.g., Futures, FxForward, VanillaOption, IRS)
//! - `time`: Calendars, conventions, handling holidays
//! - `pricing_engines`: Engine, EngineGenerator, and Pricer
//!
//! Key structs:
//! - `CalculationConfiguration`: All information for pricing
//! - `Pricer`: Enum containing pricers for each Instrument
//! - `Engine`: Creates parameters and executes Pricers for risk calculations
//! - `CalculationResult`: Contains price, greeks, and cashflows
//! - `EngineGenerator`: Groups instruments and creates Engines for each group
//!
//! ## Status and Future Plans
//!
//! Currently in the prototype stage, `rustmetrics` can handle basic plain instruments.
//! Future plans include implementing more complex financial instruments, adding support for
//! numerical simulations, optimizing performance, and expanding the range of supported risk metrics.
//!
//! ## License
//!
//! This project is dual-licensed under Apache License, Version 2.0 and MIT license.

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
