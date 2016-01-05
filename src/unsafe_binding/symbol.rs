use types::{c_char, Id, rb_value};

#[link(name = "ruby")]
extern "C" {
    pub fn rb_id2sym(id: Id) -> rb_value;
    pub fn rb_id2name(id: Id) -> *const c_char;
    pub fn rb_sym2id(id: rb_value) -> Id;
}
