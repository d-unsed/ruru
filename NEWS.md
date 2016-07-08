# Changes by Version

## Unreleased

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

[0.7.7]: https://github.com/d-unseductable/ruru/compare/v0.7.6...v0.7.7
[0.7.6]: https://github.com/d-unseductable/ruru/compare/v0.7.5...v0.7.6
[0.7.5]: https://github.com/d-unseductable/ruru/compare/v0.7.4...v0.7.5
[0.7.4]: https://github.com/d-unseductable/ruru/compare/v0.7.3...v0.7.4
[0.7.3]: https://github.com/d-unseductable/ruru/compare/v0.7.2...v0.7.3
[0.7.2]: https://github.com/d-unseductable/ruru/compare/v0.7.1...v0.7.2
[0.7.1]: https://github.com/d-unseductable/ruru/compare/v0.7.0...v0.7.1
[0.7.0]: https://github.com/d-unseductable/ruru/compare/v0.6.0...v0.7.0
