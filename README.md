# Ruru (Rust + Ruby)

## Native Ruby extensions in Rust

[![](http://meritbadge.herokuapp.com/ruru)](https://crates.io/crates/ruru)
[![Documentation](https://docs.rs/ruru/badge.svg)](https://docs.rs/ruru)
[![Build Status](https://travis-ci.org/d-unseductable/ruru.svg?branch=master)](https://travis-ci.org/d-unseductable/ruru)
[![Build status](https://ci.appveyor.com/api/projects/status/2epyqhooimdu6u5l?svg=true)](https://ci.appveyor.com/project/d-unseductable/ruru)
[![Gitter](https://badges.gitter.im/rust-ruru/general.svg)](https://gitter.im/rust-ruru/general?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge)

<p align="center">
  <img src="http://this-week-in-ruru.org/assets/images/logo.png" width="350" height="350">
  <br>
  <b><a href="https://docs.rs/ruru">Documentation</a></b>
  <br>
  <b><a href="http://this-week-in-ruru.org">Website</a></b>
  <br>
</p>

Have you ever considered rewriting some parts of your ~~slow~~ Ruby application?

Just replace your Ruby application with Rust, method by method, class by class. It does not require you
to change the interface of your classes or to change any other Ruby code.

As simple as Ruby, as efficient as Rust.

## Contents

* [Examples](#examples)
  - [The famous `String#blank?` method](#the-famous-stringblank-method)
  - [Simple Sidekiq-compatible server](#simple-sidekiq-compatible-server)
  - [Safe conversions](#safe-conversions)
  - [Wrapping Rust data to Ruby objects](#wrapping-rust-data-to-ruby-objects)
  - [True parallelism](#true-parallelism)
  - [Defining a new class](#defining-a-new-class)
  - [Replacing only several methods instead of the whole class](#replacing-only-several-methods-instead-of-the-whole-class)
  - [Class definition DSL](#class-definition-dsl)
  - [Calling Ruby code from Rust](#calling-ruby-code-from-rust)
* [... and why is FFI not enough?](#-and-why-is-ffi-not-enough)
* [How do I use it?](#how-do-i-use-it)
* [Contributors are welcome!](#contributors-are-welcome)
* [License](#license)

## Examples

### The famous `String#blank?` method

The fast `String#blank?` implementation by Yehuda Katz

```rust,no_run
#[macro_use]
extern crate ruru;

use ruru::{Boolean, Class, Object, RString};

methods!(
   RString,
   itself,

   fn string_is_blank() -> Boolean {
       Boolean::new(itself.to_string().chars().all(|c| c.is_whitespace()))
   }
);

#[no_mangle]
pub extern fn initialize_string() {
    Class::from_existing("String").define(|itself| {
        itself.def("blank?", string_is_blank);
    });
}
```

### Simple Sidekiq-compatible server

[Link to the repository](https://github.com/d-unseductable/rust_sidekiq)

### Safe conversions

Since 0.8.0 safe conversions are available for built-in Ruby types and for custom types.

Let's imagine that we are writing an HTTP server. It should handle requests which are passed from
Ruby side.

Any object which responds to `#body` method is considered as a valid request.

```rust,no_run
#[macro_use]
extern crate ruru;

use std::error::Error;
use ruru::{Class, Object, RString, VerifiedObject, VM};

class!(Request);

impl VerifiedObject for Request {
    fn is_correct_type<T: Object>(object: &T) -> bool {
        object.respond_to("body")
    }

    fn error_message() -> &'static str {
        "Not a valid request"
    }
}

class!(Server);

methods!(
    Server,
    itself,

    fn process_request(request: Request) -> RString {
        let body = request
            .and_then(|request| request.send("body", vec![]).try_convert_to::<RString>())
            .map(|body| body.to_string());

        // Either request does not respond to `body` or `body` is not a String
        if let Err(ref error) = body {
            VM::raise(error.to_exception(), error.description());
        }

        let formatted_body = format!("[BODY] {}", body.unwrap());

        RString::new(&formatted_body)
    }
);

#[no_mangle]
pub extern fn initialize_server() {
    Class::new("Server", None).define(|itself| {
        itself.def("process_request", process_request);
    });
}
```

### Wrapping Rust data to Ruby objects

Wrap `Server`s to `RubyServer` objects

```rust,no_run
#[macro_use] extern crate ruru;
#[macro_use] extern crate lazy_static;

use ruru::{AnyObject, Class, Fixnum, Object, RString, VM};

// The structure which we want to wrap
pub struct Server {
    host: String,
    port: u16,
}

impl Server {
    fn new(host: String, port: u16) -> Self {
        Server {
            host: host,
            port: port,
        }
    }

    fn host(&self) -> &str {
        &self.host
    }

    fn port(&self) -> u16 {
        self.port
    }
}

wrappable_struct!(Server, ServerWrapper, SERVER_WRAPPER);

class!(RubyServer);

methods!(
    RubyServer,
    itself,

    fn ruby_server_new(host: RString, port: Fixnum) -> AnyObject {
        let server = Server::new(host.unwrap().to_string(),
                                 port.unwrap().to_i64() as u16);

        Class::from_existing("RubyServer").wrap_data(server, &*SERVER_WRAPPER)
    }

    fn ruby_server_host() -> RString {
        let host = itself.get_data(&*SERVER_WRAPPER).host();

        RString::new(host)
    }

    fn ruby_server_port() -> Fixnum {
        let port = itself.get_data(&*SERVER_WRAPPER).port();

        Fixnum::new(port as i64)
    }
);

fn main() {
    let data_class = Class::from_existing("Data");

    Class::new("RubyServer", Some(&data_class)).define(|itself| {
        itself.def_self("new", ruby_server_new);

        itself.def("host", ruby_server_host);
        itself.def("port", ruby_server_port);
    });
}
```

### True parallelism

Ruru provides a way to enable true parallelism for Ruby threads by releasing GVL (GIL).

It means that a thread with released GVL runs in parallel with other threads without
being interrupted by GVL.

Current example demonstrates a "heavy" computation (`2 * 2` for simplicity) run in parallel.

```rust,no_run
#[macro_use] extern crate ruru;

use ruru::{Class, Fixnum, Object, VM};

class!(Calculator);

methods!(
    Calculator,
    itself,

    fn heavy_computation() -> Fixnum {
        let computation = || { 2 * 2 };
        let unblocking_function = || {};

        // release GVL for current thread until `computation` is completed
        let result = VM::thread_call_without_gvl(
            computation,
            Some(unblocking_function)
        );

        Fixnum::new(result)
    }
);

fn main() {
    Class::new("Calculator", None).define(|itself| {
        itself.def("heavy_computation", heavy_computation);
    });
}
```

### Defining a new class

Let's say you have a `Calculator` class.

```ruby
class Calculator
  def pow_3(number)
    (1..number).each_with_object({}) do |index, hash|
      hash[index] = index ** 3
    end
  end
end

# ... somewhere in the application code ...
Calculator.new.pow_3(5) #=> { 1 => 1, 2 => 8, 3 => 27, 4 => 64, 5 => 125 }
```

You have found that it's very slow to call `pow_3` for big numbers and decided to replace the whole class
with Rust.

```rust,no_run
#[macro_use]
extern crate ruru;

use std::error::Error;
use ruru::{Class, Fixnum, Hash, Object, VM};

class!(Calculator);

methods!(
    Calculator,
    itself,

    fn pow_3(number: Fixnum) -> Hash {
        let mut result = Hash::new();

        // Raise an exception if `number` is not a Fixnum
        if let Err(ref error) = number {
            VM::raise(error.to_exception(), error.description());
        }

        for i in 1..number.unwrap().to_i64() + 1 {
            result.store(Fixnum::new(i), Fixnum::new(i.pow(3)));
        }

        result
    }
);

#[no_mangle]
pub extern fn initialize_calculator() {
    Class::new("Calculator", None).define(|itself| {
        itself.def("pow_3", pow_3);
    });
}
```

Ruby:

```ruby
# No Calculator class in Ruby anymore

# ... somewhere in the application ...
Calculator.new.pow_3(5) #=> { 1 => 1, 2 => 8, 3 => 27, 4 => 64, 5 => 125 }
```

Nothing has changed in the API of class, thus there is no need to change any code elsewhere in the app.

### Replacing only several methods instead of the whole class

If the `Calculator` class from the example above has more Ruby methods, but we want to
replace only `pow_3`, use `Class::from_existing()`

```rust,ignore
Class::from_existing("Calculator").define(|itself| {
    itself.def("pow_3", pow_3);
});
```

### Class definition DSL

```rust,no_run
Class::new("Hello", None).define(|itself| {
    itself.const_set("GREETING", &RString::new("Hello, World!").freeze());

    itself.attr_reader("reader");

    itself.def_self("greeting", greeting);
    itself.def("many_greetings", many_greetings);

    itself.define_nested_class("Nested", None).define(|itself| {
        itself.def_self("nested_greeting", nested_greeting);
    });
});
```

Which corresponds to the following Ruby code:

```ruby
class Hello
  GREETING = "Hello, World".freeze

  attr_reader :reader

  def self.greeting
    # ...
  end

  def many_greetings
    # ...
  end

  class Nested
    def self.nested_greeting
      # ...
    end
  end
end
```

See documentation for `Class` and `Object` for more information.

### Calling Ruby code from Rust

Getting an account balance of some `User` whose name is John and who is 18 or 19 years old.

```ruby
default_balance = 0

account_balance = User
  .find_by(age: [18, 19], name: 'John')
  .account_balance

account_balance = default_balance unless account_balance.is_a?(Fixnum)
```

```rust,no_run
#[macro_use]
extern crate ruru;

use ruru::{Array, Class, Fixnum, Hash, Object, RString, Symbol};

fn main() {
    let default_balance = 0;
    let mut conditions = Hash::new();

    conditions.store(
        Symbol::new("age"),
        Array::new().push(Fixnum::new(18)).push(Fixnum::new(19))
    );

    conditions.store(
        Symbol::new("name"),
        RString::new("John")
    );

    // Fetch user and his balance
    // and set it to 0 if balance is not a Fixnum (for example `nil`)
    let account_balance =
        Class::from_existing("User")
            .send("find_by", vec![conditions.to_any_object()])
            .send("account_balance", vec![])
            .try_convert_to::<Fixnum>()
            .map(|balance| balance.to_i64())
            .unwrap_or(default_balance);
}
```

**Check out [Documentation](https://docs.rs/ruru) for many more
examples!**

## ... and why is **FFI** not enough?

 - No support of native Ruby types;

 - No way to create a standalone application to run the Ruby VM separately;

 - No way to call your Ruby code from Rust;

## How do I use it?

Warning! The crate is a WIP.

It is recommended to use [Thermite](https://github.com/malept/thermite) gem,
a Rake-based helper for building and distributing Rust-based Ruby extensions.

To be able to use Ruru, make sure that your Ruby version is 2.3.0 or higher.

1. Your local MRI copy has to be built with the `--enable-shared` option. For
   example, using rbenv:

  ```bash
  CONFIGURE_OPTS=--enable-shared rbenv install 2.3.0
  ```

2. Add Ruru to `Cargo.toml`

  ```toml
  [dependencies]
  ruru = "0.9.0"
  ```

3. Compile your library as a `dylib`

  ```toml
  [lib]
  crate-type = ["dylib"]
  ```

4. Create a function which will initialize the extension

  ```rust,ignore
  #[no_mangle]
  pub extern fn initialize_my_app() {
      Class::new("SomeClass");

      // ... etc
  }
  ```

5. Build extension

  ```bash
  $ cargo build --release
  ```

  or using Thermite

  ```bash
  $ rake thermite:build
  ```

6. On the ruby side, open the compiled `dylib` and call the function to initialize extension

  ```ruby
  require 'fiddle'

  library = Fiddle::dlopen('path_to_dylib/libmy_library.dylib')

  Fiddle::Function.new(library['initialize_my_app'], [], Fiddle::TYPE_VOIDP).call
  ```

7. Ruru is ready :heart:

## Contributors are welcome!

If you have any questions, join Ruru on
[Gitter](https://gitter.im/rust-ruru/general?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge)

## License

MIT License

Copyright (c) 2015-2016 Dmitry Gritsay

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

Icon is designed by [Github](https://github.com).
