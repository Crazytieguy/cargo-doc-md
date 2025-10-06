pub mod types;
pub mod functions;
pub mod nested;

pub const MAX_SIZE: usize = 100;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    message: String,
}

pub struct UnitStruct;

pub struct TupleStruct(pub String, pub i32);

#[derive(Debug, Clone)]
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

    pub fn get_value(&self) -> i32 {
        self.value
    }
}

pub struct GenericStruct<T, U> {
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
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SimpleEnum {
    VariantA,
    VariantB,
    VariantC,
}

pub enum ComplexEnum {
    Unit,
    Tuple(String, i32),
    Struct { name: String, age: u32 },
}

pub enum GenericEnum<T> {
    Some(T),
    None,
}

pub trait MyTrait {
    fn required_method(&self) -> String;

    fn provided_method(&self) -> i32 {
        42
    }
}

impl MyTrait for PlainStruct {
    fn required_method(&self) -> String {
        self.name.clone()
    }
}

pub fn simple_function() {
    println!("Hello, world!");
}

pub fn function_with_args(name: &str, value: i32) -> String {
    format!("{}: {}", name, value)
}

pub fn generic_function<T: std::fmt::Display>(item: T) -> String {
    format!("Item: {}", item)
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
