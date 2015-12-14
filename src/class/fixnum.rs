use std::convert::From;

use types;
use binding::util;

use super::traits::RawObject;

pub struct Fixnum {
    value: types::rb_value
}

impl Fixnum {
    pub fn new(num: i64) -> Self {
        Fixnum {
            value: util::int_to_num(num)
        }
    }

    pub fn to_i64(&self) -> i64 {
        util::num_to_int(self.value)
    }
}

impl From<types::rb_value> for Fixnum {
    fn from(value: types::rb_value) -> Self {
        Fixnum {
            value: value
        }
    }
}

impl RawObject for Fixnum {
    fn value(&self) -> types::rb_value {
        self.value
    }
}
