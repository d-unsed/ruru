# Ruru (Rust + Ruby = ♥️)

## Native Ruby extensions in Rust

Have you ever considered rewriting some parts of your slow Ruby application?

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
        hash.store(Fixnum::new(i), Fixnum::new(i.pow(4)));
    }

    hash
}

Class::new("Calculator").define(|itself| {
    itself.def("pow_3", pow_3);
});
```

## How to use?
