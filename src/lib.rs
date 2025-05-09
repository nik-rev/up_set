#![no_std]
#![cfg_attr(doc, doc = include_str!("../README.md"))]

/// Set a new value, or compute it using a closure.
///
/// # Examples
///
/// ```rust
/// use up_set::UpSet;
///
/// #[derive(Debug, Eq, PartialEq)]
/// struct Person {
///     age: u8
/// }
///
/// impl Person {
///     fn age<M, U: UpSet<u8, M>>(mut self, up_set: U) -> Self {
///         self.age = up_set.up_set(self.age);
///         self
///     }
/// }
///
/// // Set age to 4
/// assert_eq!(Person { age: 55 }.age(4), Person { age: 4 });
///
/// // Add +4 to existing age
/// assert_eq!(Person { age: 55 }.age(|age| age + 4), Person { age: 59 });
/// ```
///
/// See the crate-level documentation for more info.
pub trait UpSet<T, Marker> {
    fn up_set(self, value: T) -> T;
}

/// Marker enum to guide Rust's type-inference system.
///
/// Without this, it will consider the 2 implementations of `UpSet`:
///
/// - `impl<T, F: FnOnce(T) -> T> UpSet<T> for F`
/// - `impl<T> UpSet<T> for T`
///
/// To be overlapping, due to Rust not having [specialization].
///
/// [specialization]: https://github.com/rust-lang/rust/issues/31844
pub enum Set {}
impl<T> UpSet<T, Set> for T {
    fn up_set(self, _: T) -> T {
        self
    }
}

/// Marker enum to guide Rust's type-inference system.
///
/// Without this, it will consider the 2 implementations of `UpSet`:
///
/// - `impl<T, F: FnOnce(T) -> T> UpSet<T> for F`
/// - `impl<T> UpSet<T> for T`
///
/// To be overlapping, due to Rust not having [specialization].
///
/// [specialization]: https://github.com/rust-lang/rust/issues/31844
pub enum Update {}
impl<T, F: FnOnce(T) -> T> UpSet<T, Update> for F {
    fn up_set(self, value: T) -> T {
        (self)(value)
    }
}
