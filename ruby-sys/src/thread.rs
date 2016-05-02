use types::{CallbackPtr, Value};

extern "C" {
    pub fn rb_thread_create(id: Id) -> Value;
}
