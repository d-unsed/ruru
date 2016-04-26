extern crate libc;

pub mod array;
pub mod class;
pub mod fixnum;
pub mod types;
pub mod hash;
pub mod string;
pub mod symbol;
pub mod util;
pub mod vm;

use types::Value;

extern "C" {
    pub static rb_cObject: Value;
}
