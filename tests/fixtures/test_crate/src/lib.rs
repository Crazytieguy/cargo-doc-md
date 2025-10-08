//! # Test Crate
//!
//! A comprehensive test crate demonstrating all Rust documentation features.
//!
//! This crate serves as a test fixture for `cargo-doc-md` and demonstrates:
//!
//! - **Structs**: Plain, tuple, unit, generic, with methods
//! - **Enums**: Simple and complex variants
//! - **Traits**: With associated types, constants, default implementations
//! - **Functions**: Including async, generic, const, and unsafe
//! - **Lifetimes**: Explicit lifetime parameters and bounds
//! - **Patterns**: Builder, newtype, typestate, visitor
//! - **Error handling**: Custom error types with `std::error::Error`
//! - **Documentation**: Rich markdown with examples, links, and code blocks
//!
//! ## Quick Start
//!
//! ```rust
//! use test_crate::{PlainStruct, Builder};
//!
//! let item = PlainStruct::new("example".to_string(), 42);
//! assert_eq!(item.get_value(), 42);
//!
//! let built = Builder::new()
//!     .name("test".to_string())
//!     .value(100)
//!     .build()
//!     .unwrap();
//! ```
//!
//! ## Module Organization
//!
//! - [`types`] - Type definitions including containers and enums
//! - [`functions`] - Various function signatures and examples
//! - [`traits`] - Advanced trait definitions with associated types
//! - [`lifetimes`] - Lifetime parameter examples
//! - [`patterns`] - Common Rust design patterns
//! - [`async_example`] - Async/await functionality
//! - [`errors`] - Error handling patterns
//! - [`nested`] - Nested module hierarchy example
//!
//! ## Feature Flags
//!
//! This crate has no feature flags but demonstrates documentation of them.
//!
//! ## Safety
//!
//! This crate contains `unsafe` code examples for documentation purposes only.

pub mod types;
pub mod functions;
pub mod nested;
pub mod traits;
pub mod lifetimes;
pub mod patterns;
pub mod async_example;
pub mod errors;

use std::fmt;

pub const MAX_SIZE: usize = 100;

pub const MIN_SIZE: usize = 0;

pub const VERSION: &str = "0.1.0";

pub type Result<T> = std::result::Result<T, Error>;

pub type GenericResult<T, E = Error> = std::result::Result<T, E>;

pub use patterns::Builder;
pub use patterns::Newtype;
pub use errors::CustomError;

#[derive(Debug)]
pub struct Error {
    message: String,
}

impl Error {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for Error {}

pub struct UnitStruct;

pub struct TupleStruct(pub String, pub i32);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlainStruct {
    pub name: String,
    pub value: i32,
    private_field: bool,
}

impl PlainStruct {
    pub fn new(name: String, value: i32) -> Self {
        Self {
            name,
            value,
            private_field: false,
        }
    }

    pub fn with_private(name: String, value: i32, private_field: bool) -> Self {
        Self {
            name,
            value,
            private_field,
        }
    }

    pub fn get_value(&self) -> i32 {
        self.value
    }

    pub fn set_value(&mut self, value: i32) {
        self.value = value;
    }
}

impl Default for PlainStruct {
    fn default() -> Self {
        Self::new(String::new(), 0)
    }
}

#[derive(Debug, Clone)]
pub struct GenericStruct<T, U = String> {
    pub first: T,
    pub second: U,
}

impl<T, U> GenericStruct<T, U> {
    pub fn new(first: T, second: U) -> Self {
        Self { first, second }
    }

    pub fn swap(self) -> GenericStruct<U, T> {
        GenericStruct {
            first: self.second,
            second: self.first,
        }
    }

    pub fn map_first<F, R>(self, f: F) -> GenericStruct<R, U>
    where
        F: FnOnce(T) -> R,
    {
        GenericStruct {
            first: f(self.first),
            second: self.second,
        }
    }
}

impl<T: Clone, U: Clone> GenericStruct<T, U> {
    pub fn duplicate(&self) -> (T, U) {
        (self.first.clone(), self.second.clone())
    }
}

pub struct BoundedGeneric<T>
where
    T: Clone + fmt::Debug + Send + Sync + 'static,
{
    pub data: T,
}

impl<T> BoundedGeneric<T>
where
    T: Clone + fmt::Debug + Send + Sync + 'static,
{
    pub fn new(data: T) -> Self {
        Self { data }
    }

    pub fn clone_data(&self) -> T {
        self.data.clone()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SimpleEnum {
    VariantA,
    VariantB,
    VariantC,
}

impl SimpleEnum {
    pub const fn default_variant() -> Self {
        SimpleEnum::VariantA
    }

    pub fn is_variant_a(&self) -> bool {
        matches!(self, SimpleEnum::VariantA)
    }
}

pub enum ComplexEnum {
    Unit,
    Tuple(String, i32),
    Struct { name: String, age: u32 },
}

impl ComplexEnum {
    pub fn name(&self) -> Option<&str> {
        match self {
            ComplexEnum::Tuple(name, _) | ComplexEnum::Struct { name, .. } => Some(name),
            ComplexEnum::Unit => None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum GenericEnum<T, E = String> {
    Ok(T),
    Err(E),
    None,
}

impl<T, E> GenericEnum<T, E> {
    pub fn is_ok(&self) -> bool {
        matches!(self, GenericEnum::Ok(_))
    }

    pub fn is_err(&self) -> bool {
        matches!(self, GenericEnum::Err(_))
    }

    pub fn ok(self) -> Option<T> {
        match self {
            GenericEnum::Ok(t) => Some(t),
            _ => None,
        }
    }
}

pub trait MyTrait {
    fn required_method(&self) -> String;

    fn provided_method(&self) -> i32 {
        42
    }

    fn another_provided(&self) -> bool {
        true
    }
}

impl MyTrait for PlainStruct {
    fn required_method(&self) -> String {
        self.name.clone()
    }

    fn provided_method(&self) -> i32 {
        self.value
    }
}

impl MyTrait for String {
    fn required_method(&self) -> String {
        self.clone()
    }
}

pub trait DisplayDebug: fmt::Display + fmt::Debug {
    fn format_both(&self) -> String {
        format!("Display: {}, Debug: {:?}", self, self)
    }
}

pub fn simple_function() {
    println!("Hello, world!");
}

pub fn function_with_args(name: &str, value: i32) -> String {
    format!("{}: {}", name, value)
}

pub fn generic_function<T: fmt::Display>(item: T) -> String {
    format!("Item: {}", item)
}

pub fn multiple_bounds<T>(item: T) -> String
where
    T: fmt::Display + fmt::Debug + Clone,
{
    format!("{:?}", item)
}

pub fn function_with_result(value: i32) -> Result<String> {
    if value > 0 {
        Ok(format!("Positive: {}", value))
    } else {
        Err(Error {
            message: "Value must be positive".to_string(),
        })
    }
}

pub const fn const_function(x: i32) -> i32 {
    x * 2
}

/// An unsafe function that dereferences a raw pointer.
///
/// # Safety
///
/// The caller must ensure that `ptr` is valid, properly aligned,
/// and points to initialized memory.
pub unsafe fn unsafe_function(ptr: *const u8) -> u8 {
    *ptr
}

#[macro_export]
macro_rules! create_struct {
    ($name:expr, $value:expr) => {
        PlainStruct::new($name.to_string(), $value)
    };
}

#[macro_export]
macro_rules! max {
    ($x:expr) => ($x);
    ($x:expr, $($y:expr),+) => {
        std::cmp::max($x, max!($($y),+))
    };
}
