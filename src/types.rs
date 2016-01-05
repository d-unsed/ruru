extern crate libc;

use class::object::Object;
use class::traits::RawObject;

pub use libc::{c_char, c_int, c_long, c_void};

pub type Value = libc::uintptr_t;
pub type SignedValue = libc::intptr_t;
pub type Id = libc::uintptr_t;

pub type Argc = libc::c_int;

pub type Callback<T: RawObject> = extern fn(Argc, *const Object, Object) -> T;
pub type CallbackPtr = *const c_void;
