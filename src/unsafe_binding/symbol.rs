use types;

#[link(name = "ruby")]
extern "C" {
    pub fn rb_id2sym(id: types::rb_id) -> types::rb_value;
    pub fn rb_id2name(id: types::rb_id) -> *const types::c_char;
    pub fn rb_sym2id(id: types::rb_value) -> types::rb_id;
}
