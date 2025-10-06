pub mod inner {
    pub struct InnerStruct {
        pub value: i32,
    }

    pub fn inner_function() -> &'static str {
        "inner"
    }

    pub mod deep {
        pub struct DeepStruct {
            pub data: String,
        }

        pub fn deep_function() -> i32 {
            42
        }
    }
}

pub use inner::InnerStruct;

pub struct OuterStruct {
    pub inner: inner::InnerStruct,
}
