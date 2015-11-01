use types;

pub trait RawObject {
    fn from_value(value: types::rb_value) -> Self;

    fn value(&self) -> types::rb_value;
}
