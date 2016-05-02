use types::{c_char, Value};

extern "C" {
    pub fn ruby_init();
    pub fn rb_require(name: *const c_char) -> Value;
}
