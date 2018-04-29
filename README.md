# Non-Allocation String Iterators

This library provides basic non-allocating string iterators for both substring and character functions.

## Examples

1) Character Functions

```rust
use str_iter::Func;

fn main() {
  "Hello 😎 Dennis! 😀"
      .func_iter(|c: char| c < '\u{1F600}' || c > '\u{1F64F}')
      .for_each(|v| println!("{}", v));
}
```

## TODO

Implement the multitude of useful iterator traits.
