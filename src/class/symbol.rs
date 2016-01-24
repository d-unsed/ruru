use std::convert::From;

use binding::symbol::{id_to_name, id_to_sym, sym_to_id};
use binding::util::internal_id;
use types::Value;

use super::traits::Object;

/// `Symbol`
#[derive(Debug, PartialEq)]
pub struct Symbol {
    value: Value
}

impl Symbol {
    /// Creates a new instance of Ruby `Symbol`
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ruru::{Symbol, VM};
    /// # VM::init();
    ///
    /// let symbol = Symbol::new("hello");
    ///
    /// assert_eq!(symbol.to_string(), "hello");
    pub fn new(string: &str) -> Self {
        Symbol {
            value: id_to_sym(internal_id(string))
        }
    }

    /// Retrieves the Rust `String` corresponding to `Symbol` object (Ruby `Symbol#to_s`)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ruru::{Symbol, VM};
    /// # VM::init();
    ///
    /// let symbol = Symbol::new("hello");
    ///
    /// assert_eq!(symbol.to_string(), "hello");
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
