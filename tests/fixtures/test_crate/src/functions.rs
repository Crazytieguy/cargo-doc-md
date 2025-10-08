//! Function examples demonstrating various signatures and patterns.
//!
//! This module shows:
//! - Simple functions
//! - Generic functions with trait bounds
//! - Async functions
//! - Unsafe functions
//! - Const functions
//! - Higher-order functions

use std::ops::Mul;

/// Adds two numbers together.
///
/// # Examples
///
/// ```
/// use test_crate::functions::add;
///
/// assert_eq!(add(2, 3), 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Multiplies two values that implement `Mul`.
///
/// This is a generic function that works with any type implementing
/// the multiplication operator.
///
/// # Type Parameters
///
/// * `T` - A type that implements `Mul` and `Copy`
pub fn multiply<T: Mul<Output = T> + Copy>(a: T, b: T) -> T {
    a * b
}

/// Processes a byte slice and returns a new vector.
///
/// # Arguments
///
/// * `data` - The input byte slice to process
///
/// # Returns
///
/// A new `Vec<u8>` containing a copy of the input data.
pub fn process_slice(data: &[u8]) -> Vec<u8> {
    data.to_vec()
}

/// Mutates a byte slice in place.
///
/// Each byte is incremented by 1 (with wrapping).
///
/// # Arguments
///
/// * `data` - A mutable reference to the byte slice
pub fn process_mut_slice(data: &mut [u8]) {
    for byte in data {
        *byte = byte.wrapping_add(1);
    }
}

/// An async function that simulates fetching data.
///
/// # Arguments
///
/// * `url` - The URL to fetch from
///
/// # Returns
///
/// A `Result` containing the fetched string or an error message.
///
/// # Examples
///
/// ```no_run
/// # async fn example() {
/// use test_crate::functions::async_function;
///
/// let result = async_function("https://example.com").await;
/// assert!(result.is_ok());
/// # }
/// ```
pub async fn async_function(url: &str) -> Result<String, String> {
    Ok(format!("Fetched: {}", url))
}

/// A higher-order function that applies a function to a value.
///
/// # Arguments
///
/// * `f` - A function that takes an `i32` and returns an `i32`
///
/// # Returns
///
/// The result of calling `f(42)`.
pub fn higher_order_function<F>(f: F) -> i32
where
    F: Fn(i32) -> i32,
{
    f(42)
}

/// An unsafe function that dereferences a raw pointer.
///
/// # Safety
///
/// The caller must ensure that `ptr` is valid and properly aligned.
///
/// # Arguments
///
/// * `ptr` - A raw pointer to a `u8`
pub unsafe fn unsafe_function(ptr: *const u8) -> u8 {
    *ptr
}

/// A const function that can be evaluated at compile time.
///
/// # Examples
///
/// ```
/// use test_crate::functions::const_function;
///
/// const VALUE: i32 = const_function(21);
/// assert_eq!(VALUE, 42);
/// ```
pub const fn const_function(x: i32) -> i32 {
    x * 2
}

/// Applies a closure to each element in a slice.
///
/// # Type Parameters
///
/// * `T` - The type of elements in the slice
/// * `F` - The closure type
pub fn for_each<T, F>(slice: &[T], mut f: F)
where
    F: FnMut(&T),
{
    for item in slice {
        f(item);
    }
}

/// Maps a slice to a new vector using a closure.
pub fn map<T, U, F>(slice: &[T], f: F) -> Vec<U>
where
    F: Fn(&T) -> U,
{
    slice.iter().map(f).collect()
}

/// Filters a slice based on a predicate.
pub fn filter<T, F>(slice: &[T], predicate: F) -> Vec<&T>
where
    F: Fn(&T) -> bool,
{
    slice.iter().filter(|x| predicate(x)).collect()
}

/// A function that takes multiple generic parameters with different bounds.
pub fn complex_generics<T, U, V>(t: T, u: U, _v: V) -> String
where
    T: std::fmt::Display,
    U: std::fmt::Debug,
    V: Clone + PartialEq,
{
    format!("t: {}, u: {:?}", t, u)
}
