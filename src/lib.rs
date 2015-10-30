extern crate libc;

mod types;
mod unsafe_bindings;
mod bindings;

#[test]
fn it_works() {
    bindings::class::define_module("NewModule");
}
