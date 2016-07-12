use std::convert::From;

use binding::global::RubySpecialConsts;
use types::{InternalValue, Value};

use traits::Object;

/// `NilClass`
pub struct NilClass {
    value: Value,
}

impl NilClass {
    /// Creates a new instance of `NilClass` (`nil`).
    ///
    /// # Examples
    ///
    /// ```
    /// use ruru::{NilClass, VM};
    /// use ruru::traits::Object;
    /// # VM::init();
    ///
    /// assert!(NilClass::new().value().is_nil());
    /// ```
    ///
    /// Ruby:
    ///
    /// ```ruby
    /// nil.nil? == true
    /// ```
    pub fn new() -> Self {
        NilClass { value: Value::from(RubySpecialConsts::Nil as InternalValue) }
    }
}

impl From<Value> for NilClass {
    fn from(value: Value) -> Self {
        NilClass { value: value }
    }
}

impl Object for NilClass {
    fn value(&self) -> Value {
        self.value
    }
}
