use ruby_sys::vm::{rb_block_proc, rb_require, ruby_init};
use types::Value;
use util::str_to_cstring;

pub fn block_proc() -> Value {
    unsafe { rb_block_proc() }
}

pub fn init() {
    unsafe {
        ruby_init();
    }
}

pub fn require(name: &str) {
    unsafe {
        rb_require(str_to_cstring(name).as_ptr());
    }
}
