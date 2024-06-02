// Copyright ⓒ 2021-2024 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/doteur/blob/main/LICENCE.MD).

use std::error::Error;
use std::fmt;

#[derive(Debug)]
enum DoteurCliErrorType {
    DotExeNotInPath,
    ExtensionNotSupported,
    NoTableFound,
    NoInput,
    BadInput,
}

#[derive(Debug)]
pub struct DoteurCliError {
    message: Option<String>,
    r#type: DoteurCliErrorType,
}

impl fmt::Display for DoteurCliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let err_msg : String = match self.r#type {
            DoteurCliErrorType::DotExeNotInPath => "The dot exe isn't in your path, we couldn't write the output.If you work on linux, use your package manager to download graphviz.If you work on windows, refer to the tutorial or download the tool via the official graphviz site.Graphviz official download page : https://graphviz.org/download/.".to_string(),
            DoteurCliErrorType::ExtensionNotSupported => format!("The given extension isn't supported. Please verify it is one of the following :\n\n{}", self.message.as_ref().unwrap()),
            DoteurCliErrorType::NoTableFound => "No table found for the given input".to_string(),
            DoteurCliErrorType::NoInput => "Please precise at least one argument as input".to_string(),
            DoteurCliErrorType::BadInput => self.message.as_ref().unwrap().to_string()
       };
        write!(f, "{}", err_msg)
    }
}

impl Error for DoteurCliError {}

impl DoteurCliError {
    pub fn dot_exe_not_in_path() -> DoteurCliError {
        DoteurCliError {
            message: None,
            r#type: DoteurCliErrorType::DotExeNotInPath,
        }
    }

    pub fn ext_not_supported(message: &str) -> DoteurCliError {
        DoteurCliError {
            message: Some(message.to_string()),
            r#type: DoteurCliErrorType::ExtensionNotSupported,
        }
    }
    pub fn no_table_found() -> DoteurCliError {
        DoteurCliError {
            message: None,
            r#type: DoteurCliErrorType::NoTableFound,
        }
    }

    pub fn no_input() -> DoteurCliError {
        DoteurCliError {
            message: None,
            r#type: DoteurCliErrorType::NoInput,
        }
    }
    // Allow as it depends from the feature it's compiled with
    #[allow(dead_code)]
    pub fn bad_input(message: &str) -> DoteurCliError {
        DoteurCliError {
            message: Some(message.to_string()),
            r#type: DoteurCliErrorType::BadInput,
        }
    }
}
