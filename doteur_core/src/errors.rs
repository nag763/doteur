use std::error::Error;
use std::fmt;

#[derive(Debug)]
enum DoteurCoreErrorType {
    UserInputMalformed,
    RegexError,
    LogicError,
}

#[derive(Debug)]
pub struct DoteurCoreError {
    r#type: DoteurCoreErrorType,
    message: String,
}

impl Error for DoteurCoreError {}

impl fmt::Display for DoteurCoreError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let err_msg: String = match self.r#type {
            DoteurCoreErrorType::UserInputMalformed => {
                format!("User input is malformed : {}", self.message)
            }
            DoteurCoreErrorType::RegexError => {
                format!("Regex error ({}:{}) : {}", file!(), line!(), self.message)
            }
            DoteurCoreErrorType::LogicError => {
                format!("Logic error ({}:{}) : {}", file!(), line!(), self.message)
            }
        };
        write!(f, "{}", err_msg)
    }
}

impl DoteurCoreError {
    pub fn user_input_malformed(message: &str) -> DoteurCoreError {
        DoteurCoreError {
            r#type: DoteurCoreErrorType::UserInputMalformed,
            message: String::from(message),
        }
    }

    pub fn regex_error(message: &str) -> DoteurCoreError {
        DoteurCoreError {
            r#type: DoteurCoreErrorType::RegexError,
            message: String::from(message),
        }
    }

    pub fn logic_error(message: &str) -> DoteurCoreError {
        DoteurCoreError {
            r#type: DoteurCoreErrorType::LogicError,
            message: String::from(message),
        }
    }
}
