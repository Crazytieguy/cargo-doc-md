//! Demonstrates nested module hierarchies.
//!
//! This shows how documentation is structured for deeply nested modules.

/// Inner module with its own types and functions.
pub mod inner {
    /// A struct defined in the inner module.
    pub struct InnerStruct {
        pub value: i32,
    }

    impl InnerStruct {
        /// Creates a new `InnerStruct`.
        pub fn new(value: i32) -> Self {
            Self { value }
        }

        /// Doubles the value.
        pub fn double(&mut self) {
            self.value *= 2;
        }
    }

    /// A function in the inner module.
    pub fn inner_function() -> &'static str {
        "inner"
    }

    /// Deeply nested module.
    pub mod deep {
        /// A struct in the deeply nested module.
        pub struct DeepStruct {
            pub data: String,
        }

        impl DeepStruct {
            /// Creates a new `DeepStruct`.
            pub fn new(data: String) -> Self {
                Self { data }
            }

            /// Returns the length of the data.
            pub fn len(&self) -> usize {
                self.data.len()
            }

            /// Returns `true` if the data is empty.
            pub fn is_empty(&self) -> bool {
                self.data.is_empty()
            }
        }

        /// A function in the deeply nested module.
        pub fn deep_function() -> i32 {
            42
        }

        /// Even deeper nesting.
        pub mod deeper {
            /// The deepest struct.
            pub struct DeeperStruct;

            impl DeeperStruct {
                /// Returns a greeting from the depths.
                pub fn greet() -> &'static str {
                    "Hello from the depths!"
                }
            }
        }
    }
}

pub use inner::InnerStruct;

/// An outer struct that contains an inner struct.
pub struct OuterStruct {
    pub inner: inner::InnerStruct,
}

impl OuterStruct {
    /// Creates a new `OuterStruct`.
    pub fn new(value: i32) -> Self {
        Self {
            inner: inner::InnerStruct::new(value),
        }
    }

    /// Gets the inner value.
    pub fn get_value(&self) -> i32 {
        self.inner.value
    }
}
