use std::convert::From;

use types::{Value, ValueType};
use util;

use {Object, VerifiedObject};

/// `TrueClass` and `FalseClass`
#[derive(Debug, PartialEq)]
pub struct Boolean {
    value: Value,
}

impl Boolean {
    /// Creates a new instance boolean value from `bool`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ruru::{Boolean, VM};
    /// # VM::init();
    ///
    /// assert_eq!(Boolean::new(true).to_bool(), true);
    /// ```
    ///
    /// Ruby:
    ///
    /// ```ruby
    /// true == true
    /// ```
    pub fn new(state: bool) -> Self {
        Self::from(util::bool_to_value(state))
    }

    /// Retrieves a `bool` value from `Boolean`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ruru::{Boolean, VM};
    /// # VM::init();
    ///
    /// assert_eq!(Boolean::new(true).to_bool(), true);
    /// ```
    ///
    /// Ruby:
    ///
    /// ```ruby
    /// true == true
    /// ```
    pub fn to_bool(&self) -> bool {
        self.value().is_true()
    }
}

impl From<Value> for Boolean {
    fn from(value: Value) -> Self {
        Boolean { value: value }
    }
}

impl Object for Boolean {
    #[inline]
    fn value(&self) -> Value {
        self.value
    }
}

impl VerifiedObject for Boolean {
    fn is_correct_type<T: Object>(object: &T) -> bool {
        let ty = object.value().ty();

        ty == ValueType::True || ty == ValueType::False
    }

    fn error_message() -> String {
        "Error converting to Boolean".to_string()
    }
}
