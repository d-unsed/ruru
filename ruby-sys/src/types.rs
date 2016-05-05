pub use libc::{c_char, c_int, c_long};

use libc::uintptr_t;
use libc::intptr_t;
use libc::c_void;

pub type Value = uintptr_t;
pub type SignedValue = intptr_t;
pub type Id = uintptr_t;

#[derive(Debug, PartialEq)]
#[link_name = "ruby_value_type"]
#[repr(C)]
pub enum ValueType {
    None = 0x00,
    Object = 0x01,
    Class = 0x02,
    Module = 0x03,
    Float = 0x04,
    RString = 0x05,
    Regexp = 0x06,
    Array = 0x07,
    Hash = 0x08,
    Struct = 0x09,
    Bignum = 0x0a,
    File = 0x0b,
    Data = 0x0c,
    Match = 0x0d,
    Complex = 0x0e,
    Rational = 0x0f,
    Nil = 0x11,
    True = 0x12,
    False = 0x13,
    Symbol = 0x14,
    Fixnum = 0x15,
    Undef = 0x1b,
    Node = 0x1c,
    IClass = 0x1d,
    Zombie = 0x1e,
    Mask = 0x1f,
}

pub type Argc = c_int;

pub type CallbackPtr = *const c_void;
