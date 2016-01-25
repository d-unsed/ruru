use std::slice;

use binding::vm::init;
use class::any_object::AnyObject;
use types::{Argc, Value};

/// Virtual Machine and helpers
pub struct VM;

impl VM {
    /// Initializes Ruby virtual machine.
    ///
    /// This function should **ONLY** be used if you write a standalone application which calls
    /// Ruby itself (e.g. Rack web server which is run first and then passes requests
    /// to middleware).
    ///
    /// In this case it should be called before any interaction with Ruby.
    ///
    /// If you write a library which is connected to Ruby in runtime (e.g. some gem), this
    /// function should not be used.
    ///
    /// # Examples
    ///
    /// ```
    /// use ruru::{Class, VM};
    ///
    /// VM::init();
    ///
    /// // VM started, able to use Ruby now
    /// // ...
    ///
    /// Class::new("SomeClass"); // etc
    /// ```
    pub fn init() {
        init();
    }

    // TODO: Move to other struct
    /// Converts a pointer `AnyObject` array to `Vec<AnyObject>`.
    ///
    /// This function is a helper for callbacks.
    ///
    /// Later it will be moved to other struct, because it is not related to VM itself.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ruru::types::Argc;
    /// use ruru::{AnyObject, Boolean, Class, RString, VM};
    ///
    /// #[no_mangle]
    /// pub extern fn string_eq(argc: Argc, argv: *const AnyObject, itself: RString) -> Boolean {
    ///     let argv = VM::parse_arguments(argc, argv);
    ///     let other_string = argv[0].as_string();
    ///
    ///     Boolean::new(itself.to_string() == other_string.to_string())
    /// }
    ///
    /// fn main() {
    ///     Class::from_existing("String").define_method("==", string_eq);
    /// }
    /// ```
    pub fn parse_arguments(argc: Argc, arguments: *const AnyObject) -> Vec<AnyObject> {
        unsafe {
            slice::from_raw_parts(arguments, argc as usize).to_vec()
        }
    }
}
