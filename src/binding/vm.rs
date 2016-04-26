use ruby_sys::vm::ruby_init;

pub fn init() {
    unsafe {
        ruby_init();
    }
}
