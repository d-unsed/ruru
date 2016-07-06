use std::convert::From;

use binding::fixnum;
use types::Value;

use super::traits::Object;

/// `Fixnum`
#[derive(Debug, PartialEq)]
pub struct Fixnum {
    value: Value,
}

impl Fixnum {
    /// Creates a new `Fixnum`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ruru::{Fixnum, VM};
    /// # VM::init();
    ///
    /// let fixnum = Fixnum::new(1);
    ///
    /// assert_eq!(fixnum.to_i64(), 1);
    /// ```
    ///
    /// Ruby:
    ///
    /// ```ruby
    /// 1 == 1
    /// ```
    pub fn new(num: i64) -> Self {
        Fixnum { value: fixnum::int_to_num(num) }
    }

    /// Retrieves an `i64` value from `Fixnum`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ruru::{Fixnum, VM};
    /// # VM::init();
    ///
    /// let fixnum = Fixnum::new(1);
    ///
    /// assert_eq!(fixnum.to_i64(), 1);
    /// ```
    ///
    /// Ruby:
    ///
    /// ```ruby
    /// 1 == 1
    /// ```
    pub fn to_i64(&self) -> i64 {
        fixnum::num_to_int(self.value)
    }
}

impl From<Value> for Fixnum {
    fn from(value: Value) -> Self {
        Fixnum { value: value }
    }
}

impl Object for Fixnum {
    fn value(&self) -> Value {
        self.value
    }
}
