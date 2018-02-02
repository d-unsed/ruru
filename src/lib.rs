#[allow(unused_imports)]
#[macro_use]
extern crate lazy_static;

extern crate ruby_sys;

mod binding;
mod class;

#[macro_use]
pub mod dsl;

pub mod result;
pub mod typed_data;
pub mod types;
pub mod util;

pub use class::any_object::AnyObject;
pub use class::array::Array;
pub use class::boolean::Boolean;
pub use class::class::Class;
pub use class::fixnum::Fixnum;
pub use class::float::Float;
pub use class::gc::GC;
pub use class::hash::Hash;
pub use class::integer::Integer;
pub use class::nil_class::NilClass;
pub use class::module::Module;
pub use class::rproc::Proc;
pub use class::string::RString;
pub use class::symbol::Symbol;
pub use class::thread::Thread;
pub use class::vm::VM;

pub use class::traits::object::Object;
pub use class::traits::verified_object::VerifiedObject;

#[test]
fn it_works() {}
