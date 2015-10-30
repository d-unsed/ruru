extern crate libc;

mod bindings;
mod types;
mod unsafe_bindings;
mod util;

#[test]
fn it_works() {
    bindings::class::define_module("NewModule");
}
