use std::convert::From;

use types::Value;
use binding::fixnum::{int_to_num, num_to_int};

use super::traits::RawObject;

pub struct Fixnum {
    value: Value
}

impl Fixnum {
    pub fn new(num: i64) -> Self {
        Fixnum {
            value: int_to_num(num)
        }
    }

    pub fn to_i64(&self) -> i64 {
        num_to_int(self.value)
    }
}

impl From<Value> for Fixnum {
    fn from(value: Value) -> Self {
        Fixnum {
            value: value
        }
    }
}

impl RawObject for Fixnum {
    fn value(&self) -> Value {
        self.value
    }
}
