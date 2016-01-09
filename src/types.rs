extern crate libc;

use libc::c_void;

use class::any_object::AnyObject;
use class::traits::Object;

pub use libc::{c_char, c_int, c_long};

pub type Value = libc::uintptr_t;
pub type SignedValue = libc::intptr_t;
pub type Id = libc::uintptr_t;

pub type Argc = libc::c_int;

pub type Callback<I: Object, O: Object> = extern fn(Argc, *const AnyObject, I) -> O;
pub type CallbackPtr = *const c_void;
