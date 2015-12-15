use types;

#[link(name = "ruby")]
extern "C" {
    pub fn rb_int2inum(num: types::rb_signed_value) -> types::rb_value;
    pub fn rb_num2int(num: types::rb_value) -> types::rb_signed_value;
}
