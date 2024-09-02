use crate::definitions::{Real, Time};
use crate::parameters::volatility::VolatilityTrait;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use static_id::StaticId;
//
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstantVolatility {
    value: Real,
    name: String,
    id: StaticId,
}

impl ConstantVolatility {
    pub fn new(value: Real, name: String, id: StaticId) -> ConstantVolatility {
        ConstantVolatility { value, name, id }
    }
}

impl Default for ConstantVolatility {
    fn default() -> ConstantVolatility {
        ConstantVolatility {
            value: 0.0,
            name: "".to_string(),
            id: StaticId::default(),
        }
    }
}

impl VolatilityTrait for ConstantVolatility {
    fn get_value(&self, _t: Time, _x: Real) -> Real {
        self.value
    }

    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_code_str(&self) -> &str {
        self.id.code_str()
    }

    fn get_id(&self) -> StaticId {
        self.id
    }

    fn total_variance(&self, t: Time, _x: Real) -> Result<Real> {
        Ok(self.value * self.value * t)
    }

    fn total_deviation(&self, t: Time, _x: Real) -> Result<Real> {
        Ok(self.value * t.sqrt())
    }

    fn bump_volatility(
        &mut self,
        _time1: Option<Time>,
        _time2: Option<Time>,
        _left_spot_moneyness: Option<Real>,
        _right_spot_moneyness: Option<Real>,
        bump: Real,
    ) -> Result<()> {
        self.value += bump;
        Ok(())
    }
}
