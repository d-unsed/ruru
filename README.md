# Ruru (Rust + Ruby = :heart:)

## Native Ruby extensions in Rust

[![](http://meritbadge.herokuapp.com/ruru)](https://crates.io/crates/ruru) [![Build Status](https://travis-ci.org/d-unseductable/ruru.svg?branch=master)](https://travis-ci.org/d-unseductable/ruru)

**[Documentation](http://d-unseductable.github.io/ruru/ruru/index.html)**

Have you ever considered rewriting some parts of your ~~slow~~ Ruby application?

Just replace your Ruby application with Rust, method by method, class by class. It does not require you
to change the interface of your classes or to change any other Ruby code.

As simple as Ruby, as efficient as Rust.

## Examples

### The famous `String#blank?` method

```rust
methods!(
   RString,
   itself,

   string_is_blank() -> Boolean {
       Boolean::new(itself.to_string().chars().all(|c| c.is_whitespace()))
   }
);

fn main() {
    Class::from_existing("String").define(|itself| {
        itself.def("blank?", string_is_blank);
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

```rust
class!(Calculator);

methods!(
    Calculator,
    itself,

    pow_3(num: Fixnum) -> Hash {
        let mut hash = Hash::new();

        for i in 1..num.to_i64() + 1 {
            hash.store(Fixnum::new(i), Fixnum::new(i.pow(3)));
        }

        hash
    }
);

#[no_mangle]
pub extern fn initialize_my_app() {
    Class::new("Calculator").define(|itself| {
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

```rust
Class::from_existing("Calculator").define(|itself| {
    itself.def("pow_3", pow_3);
});
```

### Rack middleware

Set the `X-RUST` header to `Hello from Rust!`

```rust
class!(RustMiddleware);

methods!(
    RustMiddleware,
    itself,

    initialize(app: AnyObject) -> RustMiddleware {
        itself.instance_variable_set("@app", app);

        itself
    }

    call(env: Hash) -> Array {
        let app_call = itself
            .instance_variable_get("@app")
            .send("call", vec![env.to_any_object()])
            .to::<Array>();

        let status = app_call.at(0);
        let mut headers = app_call.at(1).clone().to::<Hash>();
        let response = app_call.at(2);

        headers.store(RString::new("X-RUST"), RString::new("Hello from Rust!"));

        Array::new().push(status).push(headers).push(response)
    }
);

#[no_mangle]
pub extern fn initialize_middleware() {
    Class::new("RustMiddleware").define(|itself| {
        itself.def("initialize", initialize);
        itself.def("call", call);
    });
}
```

Ruby:

```
use RustMiddleware
```

### Calling Ruby code from Rust

Getting an account balance of some `User` whose name is John and who is 18 or 19 years old.

```ruby
User
  .find_by(age: [18, 19], name: 'John')
  .account_balance
```

```rust
let mut conditions = Hash::new();

conditions.store(
    Symbol::new("age"),
    Array::new().push(Fixnum::new(18)).push(Fixnum::new(19))
);

conditions.store(
    Symbol::new("name"),
    RString::new("John")
);

let account_balance =
    Class::from_existing("User")
        .send("find_by", vec![conditions.to_any_object()])
        .send("account_balance", vec![])
        .to::<Fixnum>()
        .to_i64();
```

Check out **[Documentation](http://d-unseductable.github.io/ruru/ruru/index.html)** for more
examples!

## ... and why is **FFI** not enough?

 - No support of native Ruby types;

 - No way to create a standalone application to run the Ruby VM separately;

 - No way to call your Ruby code from Rust;

## How do I use it?

Warning! The crate is a WIP.

There are two ways of using Ruru:

 - Standalone application - Rust is run first as a compiled executable file and
   then it calls Ruby code (see docs for `VM::init()`)

 - Running Rust code from a Ruby application

The second way requires additional steps (to be improved):

1. Your local MRI copy has to be built with the `--enable-shared` option. For
   example, using rbenv:

  ```bash
  CONFIGURE_OPTS=--enable-shared rbenv install 2.3.0
  ```

2. Add Ruru to `Cargo.toml`

  ```toml
  [dependencies]
  ruru = ">= 0.5.0"
  ```

3. Compile your library as a `dylib`

  ```toml
  [lib]
  crate-type = ["dylib"]
  ```

4. Create a function which will initialize the extension

  ```rust
  #[no_mangle]
  pub extern fn initialize_my_app() {
      Class::new("SomeClass");

      /// ... etc
  }
  ```

5. Open the library and call the function from Ruby

  ```ruby
  require 'fiddle'

  library = Fiddle::dlopen('libmy_library.dylib')

  Fiddle::Function.new(library['initialize_my_app'], [], Fiddle::TYPE_VOIDP).call
  ```

6. Ruru is ready :heart:

# Contributors are welcome!
