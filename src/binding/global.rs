use types;

#[link(name = "ruby")]
extern "C" {
    pub static rb_cObject: types::rb_value;
}

pub enum RubySpecialConsts {
    False = 0,
    True = 0x14,
    Nil = 0x08,
    Undef = 0x34
}
