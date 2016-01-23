use std::convert::From;

use binding::symbol::{id_to_name, id_to_sym, sym_to_id};
use binding::util::internal_id;
use types::Value;

use super::traits::Object;

#[derive(Debug, PartialEq)]
pub struct Symbol {
    value: Value
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

impl From<Value> for Symbol {
    fn from(value: Value) -> Self {
        Symbol {
            value: value
        }
    }
}

impl Object for Symbol {
    fn value(&self) -> Value {
        self.value
    }
}
