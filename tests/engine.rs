#[cfg(test)]
mod tests {
    use rustmetrics::data::value_data::ValueData;
    use rustmetrics::data::vector_data::VectorData;
    use rustmetrics::definitions::Real;
    use rustmetrics::enums::{OptionDailySettlementType, OptionExerciseType, OptionType};
    use rustmetrics::instrument::{Instrument, Instruments};
    use rustmetrics::instruments::{
        bond::Bond, cash::Cash, futures::Futures, stock::Stock, vanilla_option::VanillaOption,
    };
    use rustmetrics::{
        Currency,
        FxCode,
    };
    use rustmetrics::pricing_engines::engine_generator::{EngineGenerator, InstrumentCategory};
    use rustmetrics::pricing_engines::match_parameter::MatchParameter;
    use rustmetrics::pricing_engines::{
        calculation_configuration::CalculationConfiguration, calculation_result::CalculationResult,
    };
    use rustmetrics::time::calendar::Calendar;
    use rustmetrics::time::calendars::{southkorea::SouthKorea, southkorea::SouthKoreaType};
    use rustmetrics::time::conventions::{
        BusinessDayConvention, DayCountConvention, PaymentFrequency,
    };
    use rustmetrics::time::jointcalendar::JointCalendar;
    use rustmetrics::{
        InstInfo,
        InstType,
        CreditRating,
        IssuerType,
        RankType,
        BondInfo,
        AccountingLevel,
    };
    use anyhow::{Context, Result};
    use ndarray::array;
    use ndarray::Array1;
    use rustc_hash::FxHashMap;
    use std::rc::Rc;
    use time::{macros::datetime, Duration};
    use flashlog::{info, get_unix_nano};
    use static_id::StaticId;

    #[test]
    fn test_engine() -> Result<()> {
        let _logger = flashlog::Logger::initialize()
            .with_file("logs", "tests-engine")?
            .with_console_report(true)
            .with_msg_buffer_size(1_000_000)
            .with_msg_flush_interval(1_000_000)
            .with_max_log_level(flashlog::LogLevel::Info)
            .with_timezone(flashlog::TimeZone::Local)
            .launch();

        info!("engine test started");
        let theta_day = 100;
        let start_time_nano = get_unix_nano();
        // Set up rolling file appender
        
        let spot: Real = 350.0;
        // evaluation date = 2021-01-01 00:00:00 +09:00
        let dt = datetime!(2024-03-13 16:30:00 +09:00);
        // make zero curve named "KSD". First make vector data whose values are 0.03 and 0.04
        // then make it as hash map whose key is "KSD"
        let value = array![0.03358, 0.03358];
        let dates = vec![
            datetime!(2025-03-13 00:00:00 +09:00),
            datetime!(2026-03-13 00:00:00 +09:00),
        ];

        let times = None;
        let market_datetime = dt.clone();
        let zero_curve1_id = StaticId::from_str("KSD", "DataProvider");
        let zero_curve_data1 = VectorData::new(
            &value - 0.0005,
            Some(dates.clone()),
            times.clone(),
            Some(market_datetime),
            Currency::KRW,
            zero_curve1_id.code_str().to_string(),
            zero_curve1_id,
        )
        .expect("Failed to create VectorData for KSD");

        let zero_curve2_id = StaticId::from_str("KRWGOV", "DataProvider");
        let zero_curve2_name = "KRWGOV".to_string();
        let zero_curve_data2 = VectorData::new(
            value,
            Some(dates.clone()),
            times,
            Some(market_datetime),
            Currency::KRW,
            zero_curve2_name,
            zero_curve2_id,
        )
        .expect("Failed to create VectorData for KRWGOV");

        let funding_curve1_name = "Discount(KRW)".to_string();
        let funding_curve1_id = StaticId::from_str("Discount(KRW)", "DataProvider");
        let funding_curve_data1 = VectorData::new(
            array![0.04, 0.04],
            Some(dates.clone()),
            None,
            Some(market_datetime),
            Currency::KRW,
            funding_curve1_name,
            funding_curve1_id,
        )
        .expect("failed to make a vector data for funding curve");

        // the borrowing fee curve which amounts to 0.005
        let bor_curve_name = "KOSPI2".to_string();
        let bor_curve_id = StaticId::from_str("KOSPI2", "DataProvider");
        let borrowing_curve_data = VectorData::new(
            array![0.005, 0.005],
            Some(dates.clone()),
            None,
            Some(market_datetime),
            Currency::KRW,
            bor_curve_name,
            bor_curve_id,
        )
        .expect("failed to make a vector data for borrowing fee");

        //
        // mapping construction
        let mut zero_curve_map = FxHashMap::default();
        zero_curve_map.insert(zero_curve1_id, zero_curve_data1);
        zero_curve_map.insert(zero_curve2_id, zero_curve_data2);
        zero_curve_map.insert(bor_curve_id, borrowing_curve_data);
        zero_curve_map.insert(funding_curve1_id, funding_curve_data1);
        
        let mut equity_vol_map = FxHashMap::default();
        let equity_surface_map = FxHashMap::default();

        let und_name = "KOSPI2".to_string();
        let und_code = StaticId::from_str("KOSPI2", "KRX");
        //let _equity_surface_data = surfacedatasample!(&market_datetime, spot);
        let equity_constant_vol1 = ValueData::new(
            0.2,
            Some(market_datetime),
            Currency::KRW,
            und_name,
            und_code,
        )
        .expect("failed to make a value data for equity volatility");

        //equity_surface_map.insert("KOSPI2".to_string(), equity_surface_data);
        equity_vol_map.insert(und_code, equity_constant_vol1);

        let fx_name1 = "USDKRW".to_string();
        let fx_code1_id = StaticId::from_str("USDKRW", "DataProvider");
        let fx_code1 = FxCode::new(Currency::USD, Currency::KRW);
        let fx1 = ValueData::new(
            1300.0,
            Some(market_datetime),
            Currency::KRW,
            fx_name1,
            fx_code1_id,
        ).expect("failed to make a value data for fx rate");
        let mut fx_data_map = FxHashMap::default();

        fx_data_map.insert(fx_code1, fx1);

        // make a vector data for dividend ratio
        let div_name = "KOSPI2".to_string();
        let div_code = StaticId::from_str("KOSPI2", "KRX");
        let dividend_data = VectorData::new(
            Array1::from(vec![3.0, 3.0]),
            Some(vec![
                datetime!(2024-06-01 00:00:00 +09:00),
                datetime!(2025-01-01 00:00:00 +09:00),
            ]),
            None,
            Some(market_datetime),
            Currency::KRW,
            div_name,
            div_code,
        ).expect("failed to make a vector data for dividend ratio");

        let mut dividend_data_map = FxHashMap::default();
        dividend_data_map.insert(div_code, dividend_data.clone());

        // make a stock data
        let stock_name = "KOSPI2".to_string();
        let stock_code = StaticId::from_str("KOSPI2", "KRX");
        let stock_data = ValueData::new(
            spot,
            Some(market_datetime),
            Currency::KRW,
            stock_name,
            stock_code,
        )
        .expect("failed to make a stock data");

        let mut stock_data_map = FxHashMap::default();
        stock_data_map.insert(stock_code, stock_data.clone());

        // make two stock futures of two maturities with the same other specs
        // then make a Instruments object with the two stock futures
        let stock_futures1_id = StaticId::from_str("165XXX1", "KRX");
        let stock_futures1_und_id = StaticId::from_str("KOSPI2", "KRX");
        let stock_futures1_name = "KOSPI2 Fut Mar21".to_string();
        let stock_futures1_issue_date = datetime!(2021-01-01 00:00:00 +09:00);
        let stock_futures1_maturity = datetime!(2024-06-14 00:00:00 +09:00);
        let stock_futures1_info = InstInfo {
            id: stock_futures1_id,
            issue_date: Some(stock_futures1_issue_date),
            maturity: Some(stock_futures1_maturity),
            currency: Currency::KRW,
            inst_type: InstType::Futures,
            unit_notional: 250_000.0,
            name: stock_futures1_name,
            accounting_level: rustmetrics::AccountingLevel::L1,
        };
        let stock_futures1 = Futures::new(
            stock_futures1_info,
            350.0,
            None,
            Currency::KRW,
            stock_futures1_und_id
        );

        let stock_futures2_id = StaticId::from_str("165XXX2", "KRX");
        let stock_futures2_und_id = StaticId::from_str("KOSPI2", "KRX");
        let stock_futures2_name = "KOSPI2 Fut Jun21".to_string();
        let stock_futures2_issue_date = datetime!(2021-01-01 00:00:00 +09:00);
        let stock_futures2_maturity = datetime!(2025-06-14 00:00:00 +09:00);
        let stock_futures2_info = InstInfo {
            id: stock_futures2_id,
            issue_date: Some(stock_futures2_issue_date),
            maturity: Some(stock_futures2_maturity),
            currency: Currency::KRW,
            inst_type: InstType::Futures,
            unit_notional: 250_000.0,
            name: stock_futures2_name,
            accounting_level: rustmetrics::AccountingLevel::L1,
        };

        let stock_futures2 = Futures::new(
            stock_futures2_info,
            350.0,
            Some(datetime!(2021-01-11 00:00:00 +09:00)),
            Currency::KRW,
            stock_futures2_und_id,
        );

        let bond1_issuedate = datetime!(2020-01-01 16:30:00 +09:00);
        let bond1_maturity = bond1_issuedate + Duration::days(365 * 6);
        let bond_name = "Virtual KTB";
        let bond_code = StaticId::from_str("KRxxxxxxxxxx", "KRW");
        let sk = Calendar::SouthKorea(SouthKorea::new(SouthKoreaType::Settlement));
        let calendar = JointCalendar::new(vec![sk])?;

        let bond1_currency = Currency::KRW;
        let bond1_issuer_type = IssuerType::Government;
        let bond1_credit_rating = CreditRating::None;
        let bond1_issuer_id = StaticId::from_str("Government", "Korea");
        let bond1_rank = RankType::Undefined;

        let bond1_inst_info = InstInfo {
            id: bond_code,
            issue_date: Some(bond1_issuedate.clone()),
            maturity: Some(bond1_maturity.clone()),
            currency: bond1_currency,
            inst_type: InstType::Bond,
            unit_notional: 10_000.0,
            name: bond_name.to_string(),
            accounting_level: AccountingLevel::L1,
        };

        let bond1_bond_info = BondInfo {
            credit_rating: bond1_credit_rating,
            issuer_type: bond1_issuer_type,
            issuer_id: bond1_issuer_id,
            rank: bond1_rank,
        };

        let bond = Bond::new_from_conventions(
            bond1_inst_info,
            bond1_bond_info,
            false,
            None,
            None,
            None,
            Some(0.03),
            None,
            None,
            None,
            calendar,
            true,
            DayCountConvention::StreetConvention,
            BusinessDayConvention::Unadjusted,
            PaymentFrequency::SemiAnnually,
            0,
            0,
        )?;

        let bond2_issuedate = datetime!(2022-12-10 16:30:00 +09:00);
        let bond2_maturity = datetime!(2025-12-10 16:30:00 +09:00);
        let bond2_name = "국고채권 04250-2512(22-13)";
        let bond2_code = StaticId::from_str("KR103501GCC0", "KRX");
        let sk = Calendar::SouthKorea(SouthKorea::new(SouthKoreaType::Settlement));
        let calendar = JointCalendar::new(vec![sk])?;

        let bond2_currency2 = Currency::KRW;
        let issuer2_type = IssuerType::Government;
        let credit2_rating = CreditRating::None;
        let bond2_rank = RankType::Undefined;

        let bond2_inst_info = InstInfo {
            id: bond2_code,
            issue_date: Some(bond2_issuedate.clone()),
            maturity: Some(bond2_maturity.clone()),
            currency: bond2_currency2,
            inst_type: InstType::Bond,
            unit_notional: 10_000.0,
            name: bond2_name.to_string(),
            accounting_level: AccountingLevel::L1,
        };

        let bond2_bond_info = BondInfo {
            credit_rating: credit2_rating,
            issuer_type: issuer2_type,
            issuer_id: StaticId::from_str("Government", "Korea"),
            rank: bond2_rank,
        };

        dbg!(dt.clone());
        let bond2 = Bond::new_from_conventions(
            bond2_inst_info,
            bond2_bond_info,
            false,
            Some(bond2_issuedate.clone()),
            None,
            None,
            Some(0.0425),
            None,
            None,
            None,
            calendar,
            true,
            DayCountConvention::StreetConvention,
            BusinessDayConvention::Unadjusted,
            PaymentFrequency::SemiAnnually,
            0,
            0,
        )?;

        let option1_id = StaticId::from_str("165XXX3", "KRX");
        let option1_name = "KOSPI2 Put Sep21".to_string();
        let option1_inst_info = InstInfo {
            id: option1_id,
            issue_date: Some(datetime!(2021-01-01 00:00:00 +09:00)),
            maturity: Some(datetime!(2024-09-13 00:00:00 +09:00)),
            currency: Currency::KRW,
            inst_type: InstType::VanillaOption,
            unit_notional: 250_000.0,
            name: option1_name,
            accounting_level: AccountingLevel::L1,
        };
        
        let option1_und_id = StaticId::from_str("KOSPI2", "KRX");

        // option
        let option1 = VanillaOption::new(
            option1_inst_info,
            285.0,
            None,
            option1_und_id,
            Currency::KRW,
            OptionType::Put,
            OptionExerciseType::European,
            OptionDailySettlementType::NotSettled,
        );

        let cash_name = "USD Cash".to_string();
        let cash_code = StaticId::from_str("USD Cash", "Account");
        let currency = Currency::USD;
        let cash_info = InstInfo {
            id: cash_code,
            issue_date: None,
            maturity: None,
            currency,
            inst_type: InstType::Cash,
            unit_notional: 1.0,
            name: cash_name,
            accounting_level: AccountingLevel::L1,
        };
        
        let cash = Cash { inst_info: cash_info };
            
        let stock_name = "KOSPI2".to_string();
        let stock_code = StaticId::from_str("KOSPI2", "KRX");
        let stock_inst_info = InstInfo {
            id: stock_code,
            issue_date: None,
            maturity: None,
            currency: Currency::KRW,
            inst_type: InstType::Stock,
            unit_notional: 1.0,
            name: stock_name,
            accounting_level: AccountingLevel::L1,
        };

        let stock = Stock {
            inst_info: stock_inst_info,
            underlying_ids: vec![StaticId::from_str("KOSPI2", "KRX")],
            rank_type: rustmetrics::StockRankType::Common,
        };

        let inst1 = Instrument::Futures(stock_futures1);
        let inst2 = Instrument::Futures(stock_futures2);
        let inst3 = Instrument::Bond(bond);
        let inst4 = Instrument::Bond(bond2);
        let inst5 = Instrument::VanillaOption(option1);
        let inst6 = Instrument::Cash(cash);
        let inst7 = Instrument::Stock(stock);

        let inst_vec = vec![
            Rc::new(inst1),
            Rc::new(inst2),
            Rc::new(inst3),
            Rc::new(inst4),
            Rc::new(inst5),
            Rc::new(inst6),
            Rc::new(inst7),
        ];

        // make a calculation configuration
        let calculation_configuration = CalculationConfiguration::default()
            .with_delta_calculation(true)
            .with_gamma_calculation(true)
            .with_theta_calculation(true)
            .with_rho_calculation(true)
            .with_vega_calculation(true)
            .with_vega_structure_calculation(true)
            .with_div_delta_calculation(true)
            .with_rho_structure_calculation(true)
            .with_div_structure_calculation(true)
            .with_vega_matrix_calculation(true)
            .with_theta_day(theta_day);

        // make a match parameter
        let mut collateral_curve_map = FxHashMap::default();
        collateral_curve_map.insert(StaticId::from_str("KOSPI2", "KRX"), StaticId::from_str("KSD", "DataProvider"));

        let mut borrowing_curve_map = FxHashMap::default();
        borrowing_curve_map.insert(StaticId::from_str("KOSPI2", "KRX"), StaticId::from_str("KOSPI2", "DataProvider"));

        let mut bond_discount_curve_map = FxHashMap::default();
        bond_discount_curve_map.insert(
            (
                StaticId::from_str("Government", "Korea"),
                IssuerType::Government,
                CreditRating::None,
                Currency::KRW,
            ),
            StaticId::from_str("KRWGOV", "DataProvider"),
        );

        let rate_index_curve_map = FxHashMap::default();

        let mut crs_curve_map = FxHashMap::default();
        crs_curve_map.insert(Currency::KRW, StaticId::from_str("KRWCRS", "DataProvider"));
        crs_curve_map.insert(Currency::USD, StaticId::from_str("USDOIS", "DataProvider"));

        let mut funding_cost_map = FxHashMap::default();
        funding_cost_map.insert(Currency::KRW, funding_curve1_id);

        let match_parameter = MatchParameter::new(
            collateral_curve_map,
            borrowing_curve_map,
            bond_discount_curve_map,
            crs_curve_map,
            rate_index_curve_map,
            funding_cost_map,
        );

        let category1 = InstrumentCategory::new(
            Some(vec![
                "Futures".to_string(),
                "VanillaCall".to_string(),
                "VanillaPut".to_string(),
                "IRS".to_string(),
                "CRS".to_string(),
                "FxFutures".to_string(),
            ]),
            Some(vec![Currency::KRW]),
            Some(vec![StaticId::from_str("KOSPI2", "KRX")]),
        );

        let category2 = InstrumentCategory::new(
            Some(vec![
                "Bond".to_string(),
                "Cash".to_string(),
                "Stock".to_string(),
            ]),
            Some(vec![Currency::KRW, Currency::USD]),
            Some(vec![StaticId::from_str("KOSPI2", "KRX")]),
        );

        let instrument_categories = vec![category1, category2];

        let mut engine_builder = EngineGenerator::builder();
        let engine_generator = engine_builder
            .with_configuration(calculation_configuration, dt.clone(), match_parameter)?
            .with_instruments(Instruments::new(inst_vec))?
            .with_instrument_categories(instrument_categories)?
            .with_data(
                fx_data_map,
                stock_data_map,
                zero_curve_map,
                dividend_data_map,
                equity_vol_map,
                equity_surface_map,
                FxHashMap::default(),
                FxHashMap::default(),
                FxHashMap::default(),
            )?;

        engine_generator
            .distribute_instruments()
            .context("Failed to distribute instruments")?;
        engine_generator
            .calculate()
            .context("Failed to calculate")?;

        let calculation_results: &FxHashMap<StaticId, CalculationResult> =
            engine_generator.get_calculation_results();

        let key_npv = vec![
            (bond_code, 0.99930),
            (stock_futures1_id, 349.466208),
            (stock_futures2_id, 356.310592),
            (option1_id, 1.3148708),
            (cash_code, 1.0),
            (stock_code, 350.0),
            (bond2_code, 1.0254111),
        ];

        for (key, npv) in key_npv.iter() {
            let result = calculation_results
                .get(key)
                .ok_or_else(|| anyhow::anyhow!("No result found for key {}", key))?;
            let npv_comp = result
                .get_npv_result()
                .ok_or_else(|| anyhow::anyhow!("No npv result found for key {}", key))?
                .get_npv();
            
            assert!(
                (npv - npv_comp).abs() < 1e-6,
                "npv comparison failed for key {}: expected {}, got {}",
                key,
                npv,
                npv_comp,
            );
        }

        let elapsed_nano = get_unix_nano() - start_time_nano;
        let elapsed = rustmetrics::util::format_duration(elapsed_nano as f64 / 1_000_000_000 as f64);
        info!("engine test finished {:?}", elapsed);

        for (key, value) in calculation_results.iter() {
            println!("inst: {}, value: {:?}", key, value);
        }

        Ok(())
    }
}