use types::{c_char, rb_id, rb_value};

#[link(name = "ruby")]
extern "C" {
    pub fn rb_id2sym(id: rb_id) -> rb_value;
    pub fn rb_id2name(id: rb_id) -> *const c_char;
    pub fn rb_sym2id(id: rb_value) -> rb_id;
}
