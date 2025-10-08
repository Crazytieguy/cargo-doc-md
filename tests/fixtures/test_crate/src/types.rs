//! Type definitions and containers.
//!
//! This module demonstrates various type definitions including:
//! - Generic containers
//! - Enums with multiple variants
//! - Type aliases
//! - Structs with lifetime parameters

use std::collections::HashMap;

/// A generic container for items of type `T`.
///
/// # Examples
///
/// ```
/// use test_crate::types::Container;
///
/// let mut container = Container::<i32>::new();
/// container.add(42);
/// assert_eq!(container.len(), 1);
/// ```
pub struct Container<T> {
    pub items: Vec<T>,
}

impl<T> Container<T> {
    /// Creates a new empty container.
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    /// Adds an item to the container.
    pub fn add(&mut self, item: T) {
        self.items.push(item);
    }

    /// Returns the number of items in the container.
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Returns `true` if the container is empty.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Returns an iterator over the items.
    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.items.iter()
    }
}

impl<T> Default for Container<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> FromIterator<T> for Container<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// A struct with a lifetime parameter.
///
/// Demonstrates borrowing data with an explicit lifetime.
pub struct RefStruct<'a> {
    pub data: &'a str,
}

impl<'a> RefStruct<'a> {
    /// Creates a new `RefStruct` from borrowed data.
    pub fn new(data: &'a str) -> Self {
        Self { data }
    }

    /// Returns the borrowed data.
    pub fn get(&self) -> &'a str {
        self.data
    }
}

/// Represents the status of an operation.
///
/// This enum demonstrates:
/// - Unit variants
/// - Struct variants with named fields
/// - Multiple variant types in one enum
#[derive(Debug, Clone, PartialEq)]
pub enum Status {
    /// The operation is idle and waiting to start.
    Idle,
    /// The operation is running with progress information.
    Running { progress: f32 },
    /// The operation completed successfully.
    Completed,
    /// The operation failed with an error message.
    Failed { error: String },
}

impl Status {
    /// Returns `true` if the status is `Running`.
    pub fn is_running(&self) -> bool {
        matches!(self, Status::Running { .. })
    }

    /// Returns `true` if the status is `Completed`.
    pub fn is_completed(&self) -> bool {
        matches!(self, Status::Completed)
    }

    /// Returns the progress if the status is `Running`.
    pub fn progress(&self) -> Option<f32> {
        match self {
            Status::Running { progress } => Some(*progress),
            _ => None,
        }
    }
}

impl Default for Status {
    fn default() -> Self {
        Status::Idle
    }
}

/// A type alias for a string-to-string map.
///
/// Commonly used for configuration and metadata.
pub type StringMap = HashMap<String, String>;

/// A type alias for a generic key-value map.
pub type Map<K, V> = HashMap<K, V>;

/// The default capacity for containers.
pub const DEFAULT_CAPACITY: usize = 10;

/// The maximum number of retries.
pub const MAX_RETRIES: u32 = 3;

/// A pair of related values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pair<T, U> {
    pub first: T,
    pub second: U,
}

impl<T, U> Pair<T, U> {
    /// Creates a new pair.
    pub fn new(first: T, second: U) -> Self {
        Self { first, second }
    }

    /// Swaps the values in the pair.
    pub fn swap(self) -> Pair<U, T> {
        Pair {
            first: self.second,
            second: self.first,
        }
    }
}

impl<T, U> From<(T, U)> for Pair<T, U> {
    fn from((first, second): (T, U)) -> Self {
        Self::new(first, second)
    }
}

impl<T, U> From<Pair<T, U>> for (T, U) {
    fn from(pair: Pair<T, U>) -> Self {
        (pair.first, pair.second)
    }
}
