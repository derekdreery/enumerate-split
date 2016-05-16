# enumerate-split

Like enumerate, but splits into blocks based on some value (like `'\n'` for
lines) and gives the block number and the block position as `(usize, usize)`

[documentation](https://derekdreery.github.io/enumerate-split/enumerate_split/index.html)

## Usage
```toml
[dependencies]
enumerate-split = "0.0.1"
```

## Example
```rust
use enumerate_split::enumerate_split;

let mut input = enumerate_split("Some \n\nstring with a newline".chars(), '\n');
assert_eq!(input.next(), Some(('S', (0, 0))));
assert_eq!(input.next(), Some(('o', (0, 1))));
assert_eq!(input.next(), Some(('m', (0, 2))));
assert_eq!(input.next(), Some(('e', (0, 3))));
assert_eq!(input.next(), Some((' ', (0, 4))));
assert_eq!(input.next(), Some(('\n', (0, 5))));
assert_eq!(input.next(), Some(('\n', (1, 0))));
assert_eq!(input.next(), Some(('s', (2, 0))))
```
