use crate::currency::FxCode;
use crate::definitions::{Real, Time};
use crate::parameters::{
    volatilities::constant_volatility::ConstantVolatility, volatility::Volatility,
};
use std::{cell::RefCell, rc::Rc};
use static_id::static_id::StaticId;

/// Quanto parameter.
/// It is assumed that the correlation are constant.
#[derive(Debug, Clone)]
pub struct Quanto {
    fx_volatility: Rc<RefCell<Volatility>>,
    correlation: Real,
    fx_code: FxCode,
    underlying_id: StaticId,
}

impl Quanto {
    pub fn new(
        fx_volatility: Rc<RefCell<Volatility>>,
        correlation: Real,
        fx_code: FxCode,
        underlying_id: StaticId,
    ) -> Quanto {
        Quanto {
            fx_volatility,
            correlation,
            fx_code,
            underlying_id,
        }
    }

    pub fn quanto_adjust(&self, t: Time, forward_moneyness: Real) -> Real {
        self.fx_volatility.borrow().get_value(t, forward_moneyness) * self.correlation
    }

    pub fn get_underlying_id(&self) -> StaticId {
        self.underlying_id
    }

    pub fn get_fx_code(&self) -> &FxCode {
        &self.fx_code
    }
}

impl Default for Quanto {
    fn default() -> Quanto {
        Quanto {
            fx_volatility: Rc::new(RefCell::new(Volatility::ConstantVolatility(
                ConstantVolatility::default(),
            ))),
            correlation: 0.0,
            fx_code: FxCode::default(),
            underlying_id: StaticId::default(),
        }
    }
}
