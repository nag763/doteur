// Copyright ⓒ 2021-2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/doteur/blob/main/LICENCE.MD).

use regex::Regex;

/// ReSearchType indicates if the regex should be made according to the fact they match the
/// given regexs, or not.
#[derive(Clone)]
enum ReSearchType {
    /// If the regex is matched, the value will be true
    Inclusive,
    /// If the regex is not matched, the value will be true
    Exclusive,
}

/// Transform a string with easy regex contents to a more complex regex
///
/// # Arguments
///
/// * `input` - The string to transform into regex.
fn str_to_regex(input: &str) -> Result<regex::Regex, regex::Error> {
    if input.is_empty() {
        return Err(regex::Error::Syntax(
            "Can't create an empty regex".to_string(),
        ));
    }
    let regex: Regex = Regex::new(format!("^{}$", input.replace('*', ".*")).as_str())?;
    Ok(regex)
}

#[macro_export]
macro_rules! matches_optionable_restriction {
        ($restrict:ident, $($to_check:expr),+) => {
            if let Some(restriction) = $restrict {
                true $(&& restriction.clone().verify_table_name($to_check))*
            } else {
                true
            }
    }
}

/// A restriction represents a condition to render or not the given table.
///
/// A restriction can either be inclusive (will match only what matches the expressions)
/// or exclusive (will only match what doesn't match the expressions)
#[derive(Clone)]
pub struct Restriction {
    /// The list of regexs
    regexs: Vec<Regex>,
    /// The type of restriction to apply
    re_search_type: ReSearchType,
}

impl Restriction {
    /// Creates new restrictions
    ///
    /// # Arguments
    ///
    /// * `re_string` - The trivial regexs as string
    /// * `re_search_type` - How the regexs should be matches
    fn new(re_string: Vec<String>, re_search_type: ReSearchType) -> Restriction {
        let mut regexs: Vec<Regex> = Vec::new();
        re_string.iter().for_each(|element| {
            if let Ok(value) = str_to_regex(element) {
                regexs.push(value)
            }
        });
        Restriction {
            regexs,
            re_search_type,
        }
    }

    /// Creates a new restriction in inclusive mode
    ///
    /// # Arguments
    ///
    /// * `re_string` - The trivial regexs as string
    ///
    /// # Example
    ///
    /// ```
    /// use doteur_core::restriction::Restriction;
    /// Restriction::new_inclusion(vec![String::from(".*hell")]);
    /// ```
    pub fn new_inclusion(re_string: Vec<String>) -> Restriction {
        Restriction::new(re_string, ReSearchType::Inclusive)
    }

    /// Creates a new restriction in exclusive mode
    ///
    /// # Arguments
    ///
    /// * `re_string` - The trivial regexs as string
    ///
    /// # Example
    ///
    /// ```
    /// use doteur_core::restriction::Restriction;
    /// Restriction::new_exclusion(vec![String::from(".*hell")]);
    /// ```
    pub fn new_exclusion(re_string: Vec<String>) -> Restriction {
        Restriction::new(re_string, ReSearchType::Exclusive)
    }

    /// Checks if the given inputs matches the restriction
    ///
    /// # Arguments
    ///
    /// * `table_name` - The table to verify with the given restriction
    ///
    /// # Example
    ///
    /// ```
    /// use doteur_core::restriction::Restriction;
    /// assert!(
    ///     {
    ///         let rest = Restriction::new_inclusion(vec![String::from("hell")]);
    ///         rest.verify_table_name("hell")
    ///     },
    ///     "Exact match"
    /// );
    /// assert!(
    ///     !{
    ///         let rest = Restriction::new_exclusion(vec![String::from("hell")]);
    ///         rest.verify_table_name("hell")
    ///     },
    ///     "Exact match"
    /// );
    /// ```
    pub fn verify_table_name(self, table_name: &str) -> bool {
        if !self.regexs.is_empty() {
            match self.re_search_type {
                ReSearchType::Inclusive => self.regexs.iter().any(|e| e.is_match(table_name)),
                ReSearchType::Exclusive => !self.regexs.iter().any(|e| e.is_match(table_name)),
            }
        } else {
            // If the array is empty
            true
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_str_to_regex() {
        assert!(str_to_regex("").is_err(), "empty regexs not allowed");
        assert_eq!(
            str_to_regex("*hell*").unwrap().as_str(),
            Regex::new("^.*hell.*$").unwrap().as_str(),
            "normal use case"
        );
        assert_eq!(
            str_to_regex("*helO1*").unwrap().as_str(),
            Regex::new("^.*helO1.*$").unwrap().as_str(),
            "normal use case"
        );
        assert_eq!(
            str_to_regex("*el").unwrap().as_str(),
            Regex::new("^.*el$").unwrap().as_str(),
            "normal use case"
        );
        assert_eq!(
            str_to_regex("he*").unwrap().as_str(),
            Regex::new("^he.*$").unwrap().as_str(),
            "normal use case"
        );
        assert_eq!(
            str_to_regex("dollar$*").unwrap().as_str(),
            Regex::new("^dollar$.*$").unwrap().as_str(),
            "edgy normal use case"
        );
    }

    #[test]
    fn test_restrictive_type() {
        assert!(
            matches!(
                {
                    let rest = Restriction::new_inclusion(vec![String::from(".*hell")]);
                    rest.re_search_type
                },
                ReSearchType::Inclusive
            ),
            "normal use case"
        );
        assert!(
            matches!(
                {
                    let rest = Restriction::new_exclusion(vec![String::from(".*hell")]);
                    rest.re_search_type
                },
                ReSearchType::Exclusive
            ),
            "normal use case"
        );
    }

    #[test]
    fn test_verify_table_name_inclusive() {
        assert!(
            {
                let rest = Restriction::new_inclusion(vec![String::from("hell")]);
                rest.verify_table_name("hell")
            },
            "Exact match"
        );
        assert!(
            {
                let rest = Restriction::new_inclusion(vec![String::from("hell*")]);
                vec!["hell", "helloe$", "helloa", "hell"]
                    .iter()
                    .all(|e| rest.clone().verify_table_name(e))
            },
            "Exact match"
        );
        assert!(
            !{
                let rest = Restriction::new_inclusion(vec![String::from("*ll*")]);
                vec!["hel", "heloe$", "heloa", "helel"]
                    .iter()
                    .all(|e| rest.clone().verify_table_name(e))
            },
            "Shouldn't match"
        );
        assert!(
            {
                let rest =
                    Restriction::new_inclusion(vec![String::from("*ll*"), String::from("he*")]);
                vec!["hey", "heloe$", "heloa", "helell", "llorn"]
                    .iter()
                    .all(|e| rest.clone().verify_table_name(e))
            },
            "Multiple match and regex"
        );
        assert!(
            {
                let rest = Restriction::new_inclusion(vec![]);
                vec!["hey", "heloe$", "heloa", "helell", "llorn"]
                    .iter()
                    .all(|e| rest.clone().verify_table_name(e))
            },
            "No regex"
        );
    }

    #[test]
    fn test_verify_table_name_exclusive() {
        assert!(
            !{
                let rest = Restriction::new_exclusion(vec![String::from("hell")]);
                rest.verify_table_name("hell")
            },
            "Exact match"
        );
        assert!(
            !{
                let rest = Some(Restriction::new_exclusion(vec![String::from("hell*")]));
                matches_optionable_restriction!(rest, "hell", "helloe$", "helloa", "hell")
            },
            "Exact match"
        );
        assert!(
            {
                let rest = Restriction::new_exclusion(vec![String::from("*ll*")]);
                vec!["hel", "heloe$", "heloa", "helel"]
                    .iter()
                    .all(|e| rest.clone().verify_table_name(e))
            },
            "Shouldn't match"
        );
        assert!(
            !{
                let rest =
                    Restriction::new_exclusion(vec![String::from("*ll*"), String::from("he*")]);
                vec!["hey", "heloe$", "heloa", "helell", "llorn"]
                    .iter()
                    .all(|e| rest.clone().verify_table_name(e))
            },
            "Multiple match and regex"
        );
        assert!(
            {
                let rest = Restriction::new_exclusion(vec![]);
                vec!["hey", "heloe$", "heloa", "helell", "llorn"]
                    .iter()
                    .all(|e| rest.clone().verify_table_name(e))
            },
            "No regex"
        );
    }
}
