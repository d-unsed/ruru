use ruby_sys::types;

use class::any_object::AnyObject;

pub use libc::{c_char, c_int, c_long};

pub type Value = types::Value;
pub type SignedValue = types::SignedValue;
pub type Id = types::Id;

pub type Argc = types::Argc;

pub type Callback<I, O> = extern "C" fn(Argc, *const AnyObject, I) -> O;
pub type CallbackPtr = types::CallbackPtr;
