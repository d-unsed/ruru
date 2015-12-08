use unsafe_binding::vm;

pub fn init() {
    unsafe {
        vm::ruby_init();
    }
}
