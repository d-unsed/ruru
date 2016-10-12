# Changes by Version

## Unreleased

* `Array::concat()`
* `Array::dup()`
* `Array::pop()`
* `Array::reverse()`
* `Array::shift()`
* `Array::sort_bang()`
* `Array::sort()`
* `Array::to_s()`
* `Array::unshift(item)`

## [0.8.1] - 2016-09-25

### Changed

* `VM` thread functions to receive `FnOnce` instead of `FnMut`

## [0.8.0] - 2016-09-18

See [wiki page for upgrading from 0.7.x to 0.8.0](https://github.com/d-unseductable/ruru/wiki/Upgrading-from-0.7-to-0.8).

### Added

* `VerifiedObject` trait for safe conversions between types
* `Object::try_convert_to()`
* `Error` enum convertible to exceptions
* `Result<T: Object, Error>` as a result of type conversion
* `unsafe_methods!` macro
* `Class::define_nested_class()`
* `Object::respond_to()`
* `Class::superclass()`
* `Class::ancestors()`
* `Hash::length()`
* `Object::singleton_class()`
* `attr_reader`, `attr_writer`, `attr_accessor` to `Class`
* GVL-related functions to `VM` (#34)
* Checking for presence of method arguments
* Derive `Debug`, `PartialEq` for all Ruby types
* `VM::raise()`

### Changed

* `AnyObject::to()` marked as `unsafe`
* `AnyObject::to()` moved to `Object::to()`
* `AnyObject::ty()` moved to `Object::ty()`
* `methods!` macro sets arguments to `Result<T: Object, Error>` using `try_convert_to()`
* Traits from `ruru::traits::*` module exported to top-level `ruru::*` module
* `Class::new()` receives optional superclass
* `define`, `define_method`, `define_singleton_method` moved from `Class` to `Object` trait
* `Hash::each()` yields keys and values as `AnyObject` instead of `Object` to allow safe conversions

### Fixed

* `x86` build

## [0.7.8] - 2016-07-09

### Added

* `Iterator` for `Array` (#30)
* `FromIterator` for `Array`

## [0.7.7] - 2016-07-07

### Added

* `Hash::each()`

### Fixed

* Windows build

### Removed

* Direct `libc` crate dependency

## [0.7.6] - 2016-07-05

### Added

* `RString::bytesize()`
* `RString::to_string_unchecked()`

### Changed

* Reduce the number of identifiers needed to import when using the `class!` macro

## [0.7.5] - 2016-06-30

### Added

* `VM::block_proc()`

## [0.7.4] - 2016-06-29

### Added

* `Proc` class

### Changed

* Reduce the number of identifiers needed to import when using macros

### Fixed

* Windows build (#25)

## [0.7.3] - 2016-05-07

### Added

* `NilClass`
* `Object.is_nil()`

### Changed

* Move the `ruby-sys` crate to its own repository (#17)

### Fixed

* Warnings from the `methods!` macro

## [0.7.2] - 2016-05-05

### Changed

* Refactor `Value` usage (#15)

## [0.7.1] - 2016-05-05

### Added

* `AnyObject.ty` (#13)

### Removed

* `ruru.unsafe_binding` module

## [0.7.0] - 2016-05-02

### Added

* `VM::require()` (Ruby's `require` statement)

### Changed

* `fn` required when declaring methods in `methods!` macro

----

For versions prior to 0.7.0, please see `git log`.

[0.8.1]: https://github.com/d-unseductable/ruru/compare/v0.8.0...v0.8.1
[0.8.0]: https://github.com/d-unseductable/ruru/compare/v0.7.8...v0.8.0
[0.7.8]: https://github.com/d-unseductable/ruru/compare/v0.7.7...v0.7.8
[0.7.7]: https://github.com/d-unseductable/ruru/compare/v0.7.6...v0.7.7
[0.7.6]: https://github.com/d-unseductable/ruru/compare/v0.7.5...v0.7.6
[0.7.5]: https://github.com/d-unseductable/ruru/compare/v0.7.4...v0.7.5
[0.7.4]: https://github.com/d-unseductable/ruru/compare/v0.7.3...v0.7.4
[0.7.3]: https://github.com/d-unseductable/ruru/compare/v0.7.2...v0.7.3
[0.7.2]: https://github.com/d-unseductable/ruru/compare/v0.7.1...v0.7.2
[0.7.1]: https://github.com/d-unseductable/ruru/compare/v0.7.0...v0.7.1
[0.7.0]: https://github.com/d-unseductable/ruru/compare/v0.6.0...v0.7.0
