use std::convert::From;

use binding::thread;
use types::{RawFd, Value};

use {Class, Object, VerifiedObject};

/// `Thread`
#[derive(Debug, PartialEq)]
pub struct Thread {
    value: Value,
}

impl Thread {
    /// Creates a new green thread.
    ///
    /// The returning value of the closure will be available as `#value` of the thread
    ///
    /// # Examples
    ///
    /// ```
    /// use ruru::{Fixnum, Thread, VM};
    /// # VM::init();
    ///
    /// Thread::new(|| {
    ///     let computation_result = 1 + 2;
    ///
    ///     Fixnum::new(computation_result)
    /// });
    /// ```
    ///
    /// Ruby:
    ///
    /// ```ruby
    /// Thread.new do
    ///   computation_result = 1 + 2
    ///
    ///   computation_result
    /// end
    /// ```
    pub fn new<F, R>(func: F) -> Self
        where F: FnOnce() -> R,
              R: Object
    {
        Self::from(thread::create(func))
    }

    /// Tells scheduler to switch to other threads while current thread is waiting for a
    /// readable event on the given file descriptor.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use ruru::{Thread, VM};
    /// # VM::init();
    ///
    /// // let fd = ...;
    ///
    /// Thread::wait_fd(fd);
    /// ```
    pub fn wait_fd(fd: RawFd) {
        thread::wait_fd(fd);
    }
}

impl From<Value> for Thread {
    fn from(value: Value) -> Self {
        Thread { value: value }
    }
}

impl Object for Thread {
    #[inline]
    fn value(&self) -> Value {
        self.value
    }
}

impl VerifiedObject for Thread {
    fn is_correct_type<T: Object>(object: &T) -> bool {
        object.class() == Class::from_existing("Thread")
    }

    fn error_message() -> &'static str {
        "Error converting to Thread"
    }
}
