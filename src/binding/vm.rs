use unsafe_binding::vm::ruby_init;

pub fn init() {
    unsafe {
        ruby_init();
    }
}
