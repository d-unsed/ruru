use std::convert::From;

use binding::hash;
use types::{Value, ValueType};

use AnyObject;
use traits::{Object, VerifiedObject};

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
        Hash { value: hash::new() }
    }

    /// Retrieves an `AnyObject` from element stored at `key` key.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ruru::{Fixnum, Hash, Symbol, VM};
    /// use ruru::traits::Object;
    /// # VM::init();
    ///
    /// let mut hash = Hash::new();
    ///
    /// hash.store(Symbol::new("key"), Fixnum::new(1));
    ///
    /// assert_eq!(hash.at(Symbol::new("key")).try_convert_to::<Fixnum>(), Ok(Fixnum::new(1)));
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
        let result = hash::aref(self.value(), key.value());

        AnyObject::from(result)
    }

    /// Associates the `value` with the `key`.
    ///
    /// Both `key` and `value` must be types which implement `Object` trait.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ruru::{Fixnum, Hash, Symbol, VM};
    /// use ruru::traits::Object;
    /// # VM::init();
    ///
    /// let mut hash = Hash::new();
    ///
    /// hash.store(Symbol::new("key"), Fixnum::new(1));
    ///
    /// assert_eq!(hash.at(Symbol::new("key")).try_convert_to::<Fixnum>(), Ok(Fixnum::new(1)));
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
        let result = hash::aset(self.value(), key.value(), value.value());

        AnyObject::from(result)
    }

    /// Runs a closure for each `key` and `value` pair.
    ///
    /// You can specify types for each object if they are known and are the same for each key and
    /// each value. This way is faster, but not safe, because it does not check if the objects
    /// have correct type.
    ///
    /// ```
    /// # use ruru::{Fixnum, Hash, Symbol, VM};
    /// # VM::init();
    /// # let mut hash = Hash::new();
    ///
    /// hash.each(|key: Symbol, value: Fixnum| {
    ///     // ...
    /// });
    /// ```
    ///
    /// If the types are unknown or may vary, use `AnyObject` type and try to safely convert them.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ruru::{Fixnum, Hash, Symbol, VM};
    /// # VM::init();
    ///
    /// let mut hash = Hash::new();
    ///
    /// hash.store(Symbol::new("first_key"), Fixnum::new(1));
    /// hash.store(Symbol::new("second_key"), Fixnum::new(2));
    ///
    /// let mut doubled_values: Vec<i64> = Vec::new();
    ///
    /// hash.each(|_key: Symbol, value: Fixnum| {
    ///     doubled_values.push(value.to_i64() * 2);
    /// });
    ///
    /// assert_eq!(doubled_values, vec![2, 4]);
    /// ```
    ///
    /// Ruby:
    ///
    /// ```ruby
    /// hash = {
    ///   first_key: 1,
    ///   second_key: 2
    /// }
    ///
    /// doubled_values = []
    ///
    /// hash.each do |_key, value|
    ///   doubled_values << [value * 2]
    /// end
    ///
    /// doubled_values == [2, 4]
    /// ```
    pub fn each<K, V, F>(&self, closure: F)
        where K: Object,
              V: Object,
              F: FnMut(K, V)
    {
        hash::each(self.value, closure);
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

impl VerifiedObject for Hash {
    fn is_correct_type<T: Object>(object: &T) -> bool {
        object.value().ty() == ValueType::Hash
    }

    fn error_message() -> String {
        "Error converting to Hash".to_string()
    }
}
