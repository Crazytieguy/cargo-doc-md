use std::error::Error as StdError;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum CustomError {
    NotFound,
    InvalidInput { field: String, reason: String },
    Io(io::Error),
    Parse(String),
    Multiple(Vec<CustomError>),
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CustomError::NotFound => write!(f, "Item not found"),
            CustomError::InvalidInput { field, reason } => {
                write!(f, "Invalid input in field '{}': {}", field, reason)
            }
            CustomError::Io(e) => write!(f, "IO error: {}", e),
            CustomError::Parse(msg) => write!(f, "Parse error: {}", msg),
            CustomError::Multiple(errors) => {
                write!(f, "Multiple errors: {} errors occurred", errors.len())
            }
        }
    }
}

impl StdError for CustomError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            CustomError::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<io::Error> for CustomError {
    fn from(error: io::Error) -> Self {
        CustomError::Io(error)
    }
}

impl From<String> for CustomError {
    fn from(error: String) -> Self {
        CustomError::Parse(error)
    }
}

pub type Result<T> = std::result::Result<T, CustomError>;

pub fn fallible_operation() -> Result<String> {
    Ok("success".to_string())
}

pub fn operation_with_context(value: i32) -> Result<String> {
    if value < 0 {
        Err(CustomError::InvalidInput {
            field: "value".to_string(),
            reason: "must be non-negative".to_string(),
        })
    } else {
        Ok(format!("Value: {}", value))
    }
}

pub fn chain_errors() -> Result<String> {
    let _data = std::fs::read_to_string("nonexistent.txt")?;
    Ok("read successfully".to_string())
}

#[derive(Debug)]
pub struct ErrorContext {
    pub error: CustomError,
    pub context: String,
}

impl fmt::Display for ErrorContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.context, self.error)
    }
}

impl StdError for ErrorContext {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        Some(&self.error)
    }
}

pub trait IntoContext<T> {
    fn context(self, context: impl Into<String>) -> std::result::Result<T, ErrorContext>;
}

impl<T> IntoContext<T> for Result<T> {
    fn context(self, context: impl Into<String>) -> std::result::Result<T, ErrorContext> {
        self.map_err(|error| ErrorContext {
            error,
            context: context.into(),
        })
    }
}
