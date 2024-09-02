# RustMetrics

## A Pricing and risk engine

## Overview

## Crate structure

| Module | Description |
| ------ | ----------- |
| [data](./quantlib/src/data) | Raw market observations, which are not directly used for calculation. <br>  Data is shared by Engine object in multi-thread environment|
| [parameters](./quantlib/src/parameters) | Objects generated from data objects for actual calculation |
| [instruments](./quantlib/src/instruments) | ex) Futures, FxFutures, FxForward, FxSwap, VanillaOption, IRS, CCS, Bond, KtbFutures|
| [time](./quantlib/src/time) | Calendars, conventions, handling holiday |
| [pricing_engines](./quantlib/src/pricing_engines) | Engine, EngineGenerator, and Pricer |

| Struct \& Enum | Description |
|------- | ----------- |
|[CalculationConfiguration](./quantlib/src/pricing_engines/calculation_configuration.rs) | All information for pricing: delta bump ratio, gap days for theta calculation, etc
| [Pricer](./quantlib/src/pricing_engines/pricer.rs) | Enum containing pricers for each [Instrument](./quantlib/src/instrument.rs) |
| [Engine](./quantlib/src/pricing_engines/engine.rs) | An Engine takes data as Arc objects and creates parameters such as [ZeroCurve](./quantlib/src/parameters/zero_curve.rs), [DiscreteRatioDividend](./quantlib/src/parameters/discrete_ratio_dividend.rs), etc. The parameters, as Rc<RefCell<..>> objects, are shared only inside the Engine. Then the Engine excutes Pricers repeatedly for calculating risks, e.g., delta, gamma, theta, rho, etc|
| [CalculationResult](./quantlib/src/pricing_engines/calculation_result.rs)| price, greeks, cashflows |
| [EngineGenerator](./quantlib/src/pricing_engines/engine_generator.rs) | EngineGnerator groups instruments according to [InstrumentCategory](./quantlib/src/pricing_engines/engine_generator.rs), then [Engine](./quantlib/src/pricing_engines/engine.rs)s are created for each group of instruments. The purpose of separation is mmainly for compuation performance. This is especially useful for Monte-Carlo simulation (not yet developed) since the most of the computation cost in MC simulation is caused by path generation. |


```mermaid
---
title: quantlib structure
---
stateDiagram-v2
    EngineGenerator --> InstrumentCategory: instruments
    InstrumentCategory --> Engine1: inst group1
    InstrumentCategory --> Engine2: inst group2
    EngineGenerator --> Engine1: data & config
    EngineGenerator --> Engine2: data & config
    Engine1 --> CalculationResult: results
    Engine2 --> CalculationResult: results
    Engine1 --> Parameters: data for inst group 1
    Parameters --> Pricer1
    Parameters --> Pricer2
```
