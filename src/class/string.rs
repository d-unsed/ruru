use std::convert::From;

use binding::string::{from_value, new};
use types::Value;

use super::traits::Object;

/// `String`
pub struct RString {
    value: Value
}

impl RString {
    /// Creates a new instance of Ruby `String` containing given `string`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ruru::{RString, VM};
    /// # VM::init();
    ///
    /// let string = RString::new("Hello, World!");
    ///
    /// assert_eq!(string.to_string(), "Hello, World!".to_string());
    /// ```
    ///
    /// Ruby:
    ///
    /// ```ruby
    /// str = 'Hello, World!'
    ///
    /// str == 'Hello, World!'
    /// ```
    pub fn new(string: &str) -> Self {
        RString {
            value: new(string)
        }
    }

    /// Retrieves underlying Rust `String` from Ruby `String` object.
    ///
    /// # Examples
    ///
    /// ```
    /// use ruru::{RString, VM};
    /// # VM::init();
    ///
    /// let string = RString::new("Hello, World!");
    ///
    /// assert_eq!(string.to_string(), "Hello, World!".to_string());
    /// ```
    ///
    /// Ruby:
    ///
    /// ```ruby
    /// str = 'Hello, World!'
    ///
    /// str == 'Hello, World!'
    /// ```
    pub fn to_string(&self) -> String {
        from_value(self.value)
    }
}

impl From<Value> for RString {
    fn from(value: Value) -> Self {
        RString {
            value: value
        }
    }
}

impl Object for RString {
    fn value(&self) -> Value {
        self.value
    }
}
