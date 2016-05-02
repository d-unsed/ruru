use ruby_sys::vm::{rb_require, ruby_init};
use util::str_to_cstring;

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
