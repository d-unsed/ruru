use types;

#[link(name = "ruby")]
extern "C" {
    pub static rb_cObject: types::rb_value;
}
