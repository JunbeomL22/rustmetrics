use crate::utils::string_arithmetic::{add_period, sub_period};
//use crate::data::observable::Observable;
use crate::parameters::{
    discrete_ratio_dividend::DiscreteRatioDividend, market_price::MarketPrice,
};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;
use std::{
    cmp::Ordering,
    ops::{Add, AddAssign, Sub, SubAssign},
};
use time::{Date, OffsetDateTime};

/// # EvaluationDate
///
/// `EvaluationDate` is a central struct in the `rustmetrics` crate that represents a specific
/// date and time for financial evaluations. It manages observers that need to be updated when
/// the evaluation date changes.
///
/// ## Features
///
/// - Stores a date and time as an `OffsetDateTime`
/// - Manages lists of observers (MarketPrice and DiscreteRatioDividend)
/// - Provides methods to add observers and notify them of changes
/// - Implements comparison operations with `OffsetDateTime` and `Date`
/// - Supports serialization and deserialization (observers are skipped during serialization)
///
/// ## Usage
///
/// ```rust
/// use rustmetrics::EvaluationDate;
/// use time::OffsetDateTime;
///
/// // Create a new EvaluationDate
/// let eval_date = EvaluationDate::new(OffsetDateTime::now_utc());
///
/// // Get the date
/// let date = eval_date.date();
///
/// // Set a new date
/// eval_date.set_date(OffsetDateTime::now_utc());
/// ```
///
/// ## Observer Pattern
///
/// `EvaluationDate` implements the Observer pattern, allowing `MarketPrice` and
/// `DiscreteRatioDividend` objects to be notified when the evaluation date changes.
///
/// ```rust
/// use rustmetrics::{EvaluationDate, DiscreteRatioDividend, MarketPrice};
/// use std::rc::Rc;
/// use std::cell::RefCell;
///
/// let mut eval_date = EvaluationDate::new(OffsetDateTime::now_utc());
///
/// // Add observers
/// let dividend = Rc::new(RefCell::new(DiscreteRatioDividend::new(/* ... */)));
/// let market_price = Rc::new(RefCell::new(MarketPrice::new(/* ... */)));
///
/// eval_date.add_dividend_observer(dividend);
/// eval_date.add_marketprice_observer(market_price);
///
/// // Changing the date will notify all observers
/// eval_date.set_date(OffsetDateTime::now_utc());
/// ```
///
/// ## Comparison Operations
///
/// `EvaluationDate` can be compared with `OffsetDateTime` and `Date` types:
///
/// ```rust
/// use rustmetrics::EvaluationDate;
/// use time::{OffsetDateTime, Date};
///
/// let eval_date = EvaluationDate::new(OffsetDateTime::now_utc());
/// let other_date = OffsetDateTime::now_utc();
/// let date = Date::from_calendar_date(2023, time::Month::January, 1).unwrap();
///
/// assert!(eval_date == other_date);
/// assert!(eval_date >= date);
/// ```
///
/// ## Serialization
///
/// `EvaluationDate` supports serialization and deserialization, but the observer lists are
/// skipped during this process to avoid circular references.
///
/// ## Testing
///
/// The `test_shared_evaluation_date` function in the test module demonstrates the following:
///
/// 1. Creation of a shared `EvaluationDate`
/// 2. Construction of `ZeroCurve` and `DiscreteRatioDividend` objects using the shared date
/// 3. Testing the impact of changing the evaluation date on these financial objects
/// 4. Verifying that changing the date back restores the original values
///
/// This test ensures that the `EvaluationDate` correctly notifies its observers and that
/// financial calculations respond appropriately to date changes.
/// # Arguments
/// * `date` - The `OffsetDateTime` representing the evaluation date and time
/// * `marketprice_observers` - A list of `MarketPrice` observers
/// * `dividend_observers` - A list of `DiscreteRatioDividend` observers
#[derive(Clone, Serialize, Deserialize)]
pub struct EvaluationDate {
    date: OffsetDateTime,
    #[serde(skip)]
    marketprice_observers: Vec<Rc<RefCell<MarketPrice>>>,
    #[serde(skip)]
    dividend_observers: Vec<Rc<RefCell<DiscreteRatioDividend>>>,
}

impl PartialEq<OffsetDateTime> for EvaluationDate {
    fn eq(&self, other: &OffsetDateTime) -> bool {
        self.date == *other
    }
}

impl PartialOrd<OffsetDateTime> for EvaluationDate {
    fn partial_cmp(&self, other: &OffsetDateTime) -> Option<Ordering> {
        self.date.partial_cmp(other)
    }
}

impl PartialEq<Date> for EvaluationDate {
    fn eq(&self, other: &Date) -> bool {
        self.date() == *other
    }
}

impl PartialOrd<Date> for EvaluationDate {
    fn partial_cmp(&self, other: &Date) -> Option<Ordering> {
        self.date().partial_cmp(other)
    }
}

impl Debug for EvaluationDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EvaluationDate")
            .field("date", &self.date)
            .finish()
    }
}

impl Default for EvaluationDate {
    fn default() -> EvaluationDate {
        EvaluationDate {
            date: OffsetDateTime::now_utc(),
            marketprice_observers: vec![],
            dividend_observers: vec![],
        }
    }
}

impl EvaluationDate {
    /// Creates a new `EvaluationDate` with the specified `OffsetDateTime`.
    /// The observer lists are initialized as empty.
    /// # Example
    /// ```rust
    /// use rustmetrics::evaluation_date::EvaluationDate;
    /// use time::OffsetDateTime;
    /// let eval_date = EvaluationDate::new(OffsetDateTime::now_utc());
    /// ```
    /// # Argument
    /// * `date` - The `OffsetDateTime` representing the evaluation date and time.
    pub fn new(date: OffsetDateTime) -> EvaluationDate {
        EvaluationDate {
            date,
            marketprice_observers: vec![],
            dividend_observers: vec![],
        }
    }

    pub fn date(&self) -> Date {
        self.date.date()
    }

    pub fn get_date_clone(&self) -> OffsetDateTime {
        self.date
    }

    pub fn get_date(&self) -> &OffsetDateTime {
        &self.date
    }

    pub fn set_date(&mut self, date: OffsetDateTime) {
        self.date = date;
        self.notify_observers();
    }

    /// Adds a `DiscreteRatioDividend` observer to the `EvaluationDate`.
    /// # Example
    /// ```rust
    /// use rustmetrics::evaluation_date::EvaluationDate;
    /// use rustmetrics::parameters::discrete_ratio_dividend::DiscreteRatioDividend;
    /// use std::rc::Rc;
    /// use std::cell::RefCell;
    /// use time::OffsetDateTime;
    /// 
    /// let mut eval_date = EvaluationDate::new(OffsetDateTime::now_utc());
    /// let dividend = Rc::new(RefCell::new(DiscreteRatioDividend::new(/* ... */).expect("")));
    ///
    /// eval_date.add_dividend_observer(dividend);
    /// ```
    /// # Argument
    /// * `observer` - A reference-counted `RefCell` containing the `DiscreteRatioDividend` observer.
    /// # Panics
    /// Panics if the observer cannot be added to the list.
    /// This can occur if the observer is already present in the list.
    /// # Note
    /// The observer will be notified when the evaluation date changes.
    pub fn add_dividend_observer(&mut self, observer: Rc<RefCell<DiscreteRatioDividend>>) {
        self.dividend_observers.push(observer);
    }

    /// Adds a `MarketPrice` observer to the `EvaluationDate`.
    /// # Example
    /// ```rust
    /// use rustmetrics::{EvaluationDate, MarketPrice};
    /// use std::rc::Rc;
    /// use std::cell::RefCell;
    /// let mut eval_date = EvaluationDate::new(OffsetDateTime::now_utc());
    /// let market_price = Rc::new(RefCell::new(MarketPrice::new(/* ... */)));
    /// eval_date.add_marketprice_observer(market_price);
    /// ```
    /// # Argument
    /// * `observer` - A reference-counted `RefCell` containing the `MarketPrice` observer.
    pub fn add_marketprice_observer(&mut self, observer: Rc<RefCell<MarketPrice>>) {
        self.marketprice_observers.push(observer);
    }

    fn notify_observers(&mut self) {
        for marketprice_observer in self.marketprice_observers.iter() {
            {
                marketprice_observer
                    .borrow_mut()
                    .update_evaluation_date(self)
                    .expect("Failed to update market price observer");
            }
        }

        for dividend_observer in self.dividend_observers.iter() {
            {
                dividend_observer
                    .borrow_mut()
                    .update_evaluation_date(self)
                    .expect("Failed to update dividend observer");
            }
        }
    }

    pub fn display_observers(&self) {
        println!("Market Price Observers:");
        for observer in self.marketprice_observers.iter() {
            println!("{:?}", observer.borrow().get_name());
        }

        println!("Dividend Observers:");
        for observer in self.dividend_observers.iter() {
            println!("{:?}", observer.borrow().get_name());
        }
    }
}

impl AddAssign<&str> for EvaluationDate {
    fn add_assign(&mut self, rhs: &str) {
        self.date = add_period(&self.date, rhs);
        self.notify_observers();
    }
}

impl SubAssign<&str> for EvaluationDate {
    fn sub_assign(&mut self, rhs: &str) {
        self.date = sub_period(&self.date, rhs);
        self.notify_observers();
    }
}

impl Add<&str> for EvaluationDate {
    type Output = OffsetDateTime;

    fn add(self, rhs: &str) -> OffsetDateTime {
        add_period(&self.date, rhs)
    }
}

impl Sub<&str> for EvaluationDate {
    type Output = OffsetDateTime;

    fn sub(self, rhs: &str) -> OffsetDateTime {
        sub_period(&self.date, rhs)
    }
}
