use std::convert::From;

use types::rb_value;
use binding::fixnum::{int_to_num, num_to_int};

use super::traits::RawObject;

pub struct Fixnum {
    value: rb_value
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

impl From<rb_value> for Fixnum {
    fn from(value: rb_value) -> Self {
        Fixnum {
            value: value
        }
    }
}

impl RawObject for Fixnum {
    fn value(&self) -> rb_value {
        self.value
    }
}
