extern crate libc;

pub use libc::{c_char, c_int, c_long};

pub type rb_value = libc::uintptr_t;
pub type rb_id = libc::uintptr_t;
