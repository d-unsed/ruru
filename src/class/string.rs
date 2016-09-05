use std::convert::From;

use binding::string;
use types::{Value, ValueType};

use {Object, VerifiedObject};

/// `String`
#[derive(Debug, PartialEq)]
pub struct RString {
    value: Value,
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
        RString { value: string::new(string) }
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
        string::from_value(self.value)
    }

    /// Retrieves underlying Rust `String` from Ruby `String` object.
    ///
    /// Unlike `to_string()` it does not perform any checks for internal null-bytes.
    ///
    /// This function may be used to safely get binary data from Ruby.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ruru::{RString, VM};
    /// # VM::init();
    ///
    /// let string = RString::new("Hello,\0World!");
    ///
    /// assert_eq!(string.to_string_unchecked(), "Hello,\0World!".to_string());
    /// ```
    ///
    /// Ruby:
    ///
    /// ```ruby
    /// str = 'Hello,\0World!'
    ///
    /// str == 'Hello,\0World!'
    /// ```
    pub fn to_string_unchecked(&self) -> String {
        string::from_value_unchecked(self.value)
    }

    /// Returns the length of the string in bytes
    ///
    /// # Examples
    ///
    /// ```
    /// use ruru::{RString, VM};
    /// # VM::init();
    ///
    /// let string = RString::new("Hello, World!");
    /// let utf8_string = RString::new("⓯");
    ///
    /// assert_eq!(string.bytesize(), 13);
    /// assert_eq!(utf8_string.bytesize(), 3);
    /// ```
    ///
    /// Ruby:
    ///
    /// ```ruby
    /// string = 'Hello, World!'
    /// utf8_string = '⓯'
    ///
    /// string.bytesize == 13
    /// utf8_string.bytesize == 3
    /// ```
    pub fn bytesize(&self) -> i64 {
        string::bytesize(self.value)
    }
}

impl From<Value> for RString {
    fn from(value: Value) -> Self {
        RString { value: value }
    }
}

impl Object for RString {
    fn value(&self) -> Value {
        self.value
    }
}

impl VerifiedObject for RString {
    fn is_correct_type<T: Object>(object: &T) -> bool {
        object.value().ty() == ValueType::RString
    }

    fn error_message() -> String {
        "Error converting to String".to_string()
    }
}
