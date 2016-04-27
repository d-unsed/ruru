use std::convert::From;

use binding::hash::{aset, aref, new};
use types::Value;

use super::any_object::AnyObject;
use super::traits::Object;

/// `Hash`
pub struct Hash {
    value: Value,
}

impl Hash {
    /// Creates a new instance of empty `Hash`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ruru::{Hash, VM};
    /// # VM::init();
    ///
    /// Hash::new();
    /// ```
    ///
    /// Ruby:
    ///
    /// ```ruby
    /// {}
    /// ```
    pub fn new() -> Self {
        Hash { value: new() }
    }

    /// Retrieves an `AnyObject` from element stored at `key` key.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ruru::{Fixnum, Hash, Symbol, VM};
    /// # VM::init();
    ///
    /// let mut hash = Hash::new();
    ///
    /// hash.store(Symbol::new("key"), Fixnum::new(1));
    ///
    /// assert_eq!(hash.at(Symbol::new("key")).to::<Fixnum>(), Fixnum::new(1));
    /// ```
    ///
    /// Ruby:
    ///
    /// ```ruby
    /// hash = {}
    /// hash[:key] = 1
    ///
    /// hash[:key] == 1
    /// ```
    pub fn at<T: Object>(&self, key: T) -> AnyObject {
        let value = aref(self.value(), key.value());

        AnyObject::from(value)
    }

    /// Associates the `value` with the `key`.
    ///
    /// Both `key` and `value` must be types which implement `Object` trait.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ruru::{Fixnum, Hash, Symbol, VM};
    /// # VM::init();
    ///
    /// let mut hash = Hash::new();
    ///
    /// hash.store(Symbol::new("key"), Fixnum::new(1));
    ///
    /// assert_eq!(hash.at(Symbol::new("key")).to::<Fixnum>(), Fixnum::new(1));
    /// ```
    ///
    /// Ruby:
    ///
    /// ```ruby
    /// hash = {}
    /// hash[:key] = 1
    ///
    /// hash[:key] == 1
    /// ```
    pub fn store<K: Object, V: Object>(&mut self, key: K, value: V) -> AnyObject {
        let value = aset(self.value(), key.value(), value.value());

        AnyObject::from(value)
    }
}

impl From<Value> for Hash {
    fn from(value: Value) -> Self {
        Hash { value: value }
    }
}

impl Object for Hash {
    fn value(&self) -> Value {
        self.value
    }
}
