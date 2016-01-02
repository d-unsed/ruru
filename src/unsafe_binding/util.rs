use types::{argc, c_char, rb_id, rb_value};

#[link(name = "ruby")]
extern "C" {
    pub fn rb_const_get(klass: rb_value, id: rb_value) -> rb_value;
    pub fn rb_intern(name: *const c_char) -> rb_id;

    pub fn rb_funcallv(receiver: rb_value,
                       method: rb_value,
                       argc: argc,
                       argv: *const rb_value) -> rb_value;
}
