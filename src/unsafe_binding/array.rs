use types;

#[link(name = "ruby")]
extern "C" {
    pub fn rb_ary_new() -> types::rb_value;
    pub fn rb_ary_entry(array: types::rb_value, offset: types::c_long) -> types::rb_value;
    pub fn rb_ary_join(array: types::rb_value, separator: types::rb_value) -> types::rb_value;
    pub fn rb_ary_push(array: types::rb_value, item: types::rb_value) -> types::rb_value;

    pub fn rb_ary_store(array: types::rb_value,
                        index: types::c_long,
                        item: types::rb_value) -> types::rb_value;
}
