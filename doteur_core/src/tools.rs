// Copyright ⓒ 2021-2024 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/doteur/blob/main/LICENCE.MD).

use crate::add_traits::LastChar;
use std::fs;

/// Detect unclosed comas in a String
///
/// This method is used to detect the unclosed comas in a string. It will return an error if
/// the comas aren't enclosed properly and in consequence the input is malformed.
///
/// # Arguments
///
/// * `content` - content to detect comas in
///
/// # Example
///
/// ```
/// use doteur_core::tools::detect_comas;
/// assert!(detect_comas("Mytext,`(My text),").is_err(), "malformed");
///
/// assert!(detect_comas("coma1, coma2, coma3").is_ok(), "comas");
///
/// assert_eq!(
///     detect_comas("coma1, coma2, coma3").unwrap(),
///     vec![5, 12],
///     "comas"
/// );
/// ```
pub fn detect_comas(content: &str) -> Result<Vec<usize>, &str> {
    let self_closables: Vec<char> = vec!['`', '"'];
    let pair_closables: Vec<char> = vec!['(', '['];
    if content.is_empty() {
        return Err("Empty input");
    }
    let mut indexes: Vec<usize> = Vec::new();
    let mut buffer: String = String::new();
    for (i, c) in content.chars().enumerate() {
        match c {
            '(' => {
                // If the parenthesis aren't inside a string
                if buffer.is_empty() || !self_closables.contains(&buffer.get_last_char()) {
                    buffer.push(c);
                }
            }
            ')' => {
                if !buffer.is_empty() {
                    let last_char: char = buffer.get_last_char();
                    // If last char of buffer is an open parenthesis, we pop it and continue
                    if last_char == '(' {
                        buffer.pop();
                    } else if !self_closables.contains(&buffer.get_last_char()) {
                        return Err("Opened parenthesis never closed");
                    }
                } else {
                    return Err("Parenthesis closed without being opened");
                }
            }
            '[' => {
                // If the parenthesis aren't inside a string
                if buffer.is_empty() || !self_closables.contains(&buffer.get_last_char()) {
                    buffer.push(c);
                }
            }
            ']' => {
                if !buffer.is_empty() {
                    let last_char: char = buffer.get_last_char();
                    // If last char of buffer is an open parenthesis, we pop it and continue
                    if last_char == '[' {
                        buffer.pop();
                    } else if !self_closables.contains(&buffer.get_last_char()) {
                        return Err("Opened hooks never closed");
                    }
                } else {
                    return Err("Hooks closed without being opened");
                }
            }
            '`' => {
                if !buffer.is_empty() {
                    let last_char: char = buffer.get_last_char();
                    if last_char == '`' {
                        buffer.pop();
                    } else if pair_closables.contains(&last_char) {
                        buffer.push(c);
                    // If a back tick is neither a closure nor a declaration
                    } else {
                        return Err("Malformed, single backtick");
                    }
                } else {
                    buffer.push(c)
                }
            }
            '"' => {
                if !buffer.is_empty() {
                    let last_char: char = buffer.get_last_char();
                    if last_char == '"' {
                        buffer.pop();
                    } else if pair_closables.contains(&last_char) {
                        buffer.push(c);
                    // If a back tick is neither a closure nor a declaration
                    } else {
                        return Err("Malformed, single backtick");
                    }
                } else {
                    buffer.push(c)
                }
            }
            ',' => {
                if buffer.is_empty() {
                    indexes.push(i);
                }
            }
            _ => (),
        }
    }
    match buffer.is_empty() {
        true => Ok(indexes),
        false => Err("Malformed, some symbols aren't closed properly"),
    }
}

/// Write the output to the given file
///
/// # Arguments
///
/// * `content` - The content to write
/// * `filename` - The output file
pub fn write_output_to_file(content: &str, filename: &str) -> std::io::Result<()> {
    fs::write(filename, content)?;
    Ok(())
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_detect_comas() {
        assert!(detect_comas("").is_err(), "empty input");
        assert!(detect_comas("`,)()").is_err(), "malformed");
        assert!(detect_comas("`,").is_err(), "malformed");
        assert!(detect_comas("(,))").is_err(), "malformed");
        assert!(detect_comas("\",)\"\"").is_err(), "malformed");
        assert!(detect_comas(".,())").is_err(), "malformed");
        assert!(detect_comas("```,").is_err(), "malformed");
        assert!(detect_comas("Mytext,`(My text),").is_err(), "malformed");

        assert!(detect_comas("coma1, coma2, coma3").is_ok(), "comas");
        assert!(detect_comas(" , ").is_ok(), "comas");
        assert!(detect_comas(",").is_ok(), "comas");
        assert!(detect_comas("`coma1` , coma2").is_ok(), "comas");
        assert!(detect_comas("(`coma1`) , coma2").is_ok(), "comas");
        assert!(detect_comas("`coma1` , (coma2)").is_ok(), "comas");

        assert_eq!(
            detect_comas("coma1, coma2, coma3").unwrap(),
            vec![5, 12],
            "comas"
        );
        assert_eq!(detect_comas(" , ").unwrap(), vec![1], "comas");
        assert_eq!(detect_comas(",").unwrap(), vec![0], "comas");
        assert_eq!(detect_comas("`coma1` , coma2").unwrap(), vec![8], "comas");
        assert_eq!(
            detect_comas("(`coma1`) , coma2").unwrap(),
            vec![10],
            "comas"
        );
        assert_eq!(detect_comas("`coma1` , (coma2)").unwrap(), vec![8], "comas");
    }
}
