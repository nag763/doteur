// Copyright ⓒ 2021-2024 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/doteur/blob/main/LICENCE.MD).

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
    file: Option<String>,
    line: Option<u32>,
}

impl Error for DoteurCoreError {}

impl fmt::Display for DoteurCoreError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let err_msg: String = match self.r#type {
            DoteurCoreErrorType::UserInputMalformed => {
                format!("User input is malformed : {}", self.message)
            }
            DoteurCoreErrorType::RegexError => {
                format!(
                    "Regex error ({}:{}) : {}",
                    self.file.as_ref().unwrap(),
                    self.line.unwrap(),
                    self.message
                )
            }
            DoteurCoreErrorType::LogicError => {
                format!(
                    "Logic error ({}:{}) : {}",
                    self.file.as_ref().unwrap(),
                    self.line.unwrap(),
                    self.message
                )
            }
        };
        write!(f, "{}", err_msg)
    }
}

impl DoteurCoreError {
    pub fn user_input_malformed(message: &str) -> DoteurCoreError {
        DoteurCoreError {
            r#type: DoteurCoreErrorType::UserInputMalformed,
            message: message.to_string(),
            file: None,
            line: None,
        }
    }

    pub fn regex_error(message: &str, file: &str, line: u32) -> DoteurCoreError {
        DoteurCoreError {
            r#type: DoteurCoreErrorType::RegexError,
            message: message.to_string(),
            file: Some(file.to_string()),
            line: Some(line),
        }
    }

    pub fn logic_error(message: &str, file: &str, line: u32) -> DoteurCoreError {
        DoteurCoreError {
            r#type: DoteurCoreErrorType::LogicError,
            message: message.to_string(),
            file: Some(file.to_string()),
            line: Some(line),
        }
    }
}
