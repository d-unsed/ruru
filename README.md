# Ruru (Rust + Ruby = :heart:)

## Native Ruby extensions in Rust

[![](http://meritbadge.herokuapp.com/ruru)](https://crates.io/crates/ruru)

**[Documentation](http://d-unseductable.github.io/ruru/ruru/index.html)**

Have you ever considered rewriting some parts of your ~~slow~~ Ruby application?

Just rewrite your Ruby application with Rust method by method, class by class. It does not require
to change interface of your classes or to change any other Ruby code.

As simple as Ruby, as efficient as Rust.

## Examples

### The famous `String#blank?` method

```rust
#[no_mangle]
pub extern fn is_blank(_: Argc, _: *const AnyObject, itself: RString) -> Boolean {
    Boolean::new(itself.to_string().chars().all(|c| c.is_whitespace()))
}

fn main() {
    Class::from_existing("String").define(|itself| {
        itself.def("blank?", is_blank);
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
# ... somewhere in the application ...
Calculator.new.pow_3(5) #=> { 1 => 1, 2 => 8, 3 => 27, 4 => 64, 5 => 125 }
```

You found it's very slow to call `pow_3` for big number and decided to replace the whole class
with Rust.

```rust
#[no_mangle]
pub extern fn pow_3(argc: Argc, argv: *const AnyObject, itself: Fixnum) -> Hash {
    let argv = VM::parse_arguments(argc, argv);
    let num = argv[0].as_fixnum().to_i64();

    let mut hash = Hash::new();

    for i in (1..num + 1) {
        hash.store(Fixnum::new(i), Fixnum::new(i.pow(3)));
    }

    hash
}

Class::new("Calculator").define(|itself| {
    itself.def("pow_3", pow_3);
});
```

Ruby:

```ruby
# No Calculator class in Ruby anymore

# ... somewhere in the application ...
Calculator.new.pow_3(5) #=> { 1 => 1, 2 => 8, 3 => 27, 4 => 64, 5 => 125 }
```

So nothing has changed in the API of class thus no need to change code elsewhere in the app.

### Replacing only several methods instead of the whole class

If the `Calculator` class from the example above has more methods Ruby methods, but we want to
replace only `pow_3`, use `Class::from_existing()`

```rust
Class::from_existing("Calculator").define(|itself| {
    itself.def("pow_3", pow_3);
});
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
        .send("find_by", vec![conditions.as_any_object()])
        .send("account_balance", vec![])
        .as_fixnum()
        .to_i64();
```

Check out **[Documentation](http://d-unseductable.github.io/ruru/ruru/index.html)** for more
examples!

## ... and why **FFI** is not enough?

 - No support of native Ruby types;

 - No way to create a standalone application to run Ruby VM separately;

 - No way to call your Ruby code from Rust;

## How to use?

Ruru requires `rbenv` to be installed. Building and linking process is automatic.

There are two ways of using Ruru:

 - Standalone application - Rust is run first as a compiled executable file and then it calls Ruby
   code (see docs for `VM::init()`)

 - Running Rust code from Ruby application

The second way requires additional steps (to be improved):

 1. Add Ruru to `Cargo.toml`

   ```toml
   [dependencies]
   ruru = ">= 0.5.0"
   ```

 2. Compile your library as `dylib`

   ```toml
   [lib]
   crate-type = ["dylib"]
   ```

 3. Create a function which will initialize

   ```rust
   #[no_mangle]
   pub extern fn initialize_my_app() {
       Class::new("SomeClass");

       /// ... etc
   }
   ```

 4. Open the library and call function from Ruby

   ```ruby
   require 'fiddle'

   library = Fiddle::dlopen('libmy_library.dylib')

   Fiddle::Function.new(library['init_my_app'], [], Fiddle::TYPE_VOIDP).call
   ```

 5. Ruru is ready :heart:

# Contributors are welcome!
