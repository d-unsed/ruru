pub use ruby_sys::rb_cObject;

pub enum RubySpecialConsts {
    False = 0,
    True = 0x14,
    Nil = 0x08,
    Undef = 0x34
}
