# up_set

This crate allows you to write functions that either **set** the value or **update** it using a closure.

```rust
use up_set::UpSet;

#[derive(Debug, Eq, PartialEq)]
struct Point {
    x: u32,
    y: u32,
}

// `M` is a generic marker to guide Rust's type inference system to choose
// the correct implementation of `UpSet` for either `u32` or `Fn(u32) -> u32`
impl Point {
    fn x<M, U: UpSet<u32, M>>(mut self, x: U) -> Self {
        self.x = x.up_set(self.x);
        self
    }
    fn y<M, U: UpSet<u32, M>>(mut self, y: U) -> Self {
        self.y = y.up_set(self.y);
        self
    }
}

assert_eq!(
    Point { x: 10, y: 10 }.x(20).y(|y| y + 50),
    Point {
        // `x` was set to a specific value
        x: 20,
        // `width` was updated
        y: 60,
    }
);

assert_eq!(
    Point { x: 10, y: 10 }.x(|x| x + 20).y(50),
    Point {
        // `x` was updated
        x: 30,
        // `y` was set to a specific value
        y: 50,
    }
);
```
