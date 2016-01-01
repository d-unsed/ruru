extern crate libc;

use class::object;
use class::traits::RawObject;

pub use libc::{c_char, c_int, c_long, c_void};

pub type rb_value = libc::uintptr_t;
pub type rb_signed_value = libc::intptr_t;
pub type rb_id = libc::uintptr_t;

pub type argc = libc::c_int;

pub type callback<T: RawObject> = extern fn(argc, *const object::Object, object::Object) -> T;
pub type callback_ptr = *const c_void;
