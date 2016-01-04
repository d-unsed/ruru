use std::convert::From;

use binding::symbol::{id_to_name, id_to_sym, sym_to_id};
use binding::util::internal_id;
use types::rb_value;

use super::traits::RawObject;

pub struct Symbol {
    value: rb_value
}

impl Symbol {
    pub fn new(string: &str) -> Self {
        Symbol {
            value: id_to_sym(internal_id(string))
        }
    }

    pub fn to_string(&self) -> String {
        id_to_name(sym_to_id(self.value()))
    }
}

impl From<rb_value> for Symbol {
    fn from(value: rb_value) -> Self {
        Symbol {
            value: value
        }
    }
}

impl RawObject for Symbol {
    fn value(&self) -> rb_value {
        self.value
    }
}
