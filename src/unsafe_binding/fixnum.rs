use types::{c_long, rb_signed_value, rb_value};

#[link(name = "ruby")]
extern "C" {
    pub fn rb_int2inum(num: rb_signed_value) -> rb_value;
    pub fn rb_num2int(num: rb_value) -> c_long;
}
