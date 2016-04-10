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

## How to use?
