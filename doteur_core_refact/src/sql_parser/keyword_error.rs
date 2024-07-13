use std::fmt;

// Define an error type for invalid keyword conversion
#[derive(Debug)]
pub struct KeywordError(pub String);

impl fmt::Display for KeywordError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid keyword: {}", self.0)
    }
}

impl std::error::Error for KeywordError {}
