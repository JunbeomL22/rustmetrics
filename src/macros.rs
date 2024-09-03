#[macro_export]
macro_rules! valuedatasample {
    ($value:expr, $currency:expr, $name:expr) => {
        data::value_data::ValueData::new($value, None, $currency, $name.to_string())
    };
}

