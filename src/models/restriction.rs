use regex::Regex;

/// ReSearchType indicates if the regex should be made according to the fact they match the
/// given regexs, or not.
#[derive(Clone)]
pub enum ReSearchType {
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

/// A restriction represents a condition to render or not the given table.
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
    pub fn new_inclusion(re_string: Vec<String>) -> Restriction {
        Restriction::new(re_string, ReSearchType::Inclusive)
    }

    /// Creates a new restriction in exclusive mode
    ///
    /// # Arguments
    ///
    /// * `re_string` - The trivial regexs as string
    pub fn new_exclusion(re_string: Vec<String>) -> Restriction {
        Restriction::new(re_string, ReSearchType::Exclusive)
    }

    /// Checks if the given inputs matches the restriction
    ///
    /// # Arguments
    ///
    /// * `table_name` - The table to verify with the given restriction
    pub fn verify_table_name(self, table_name: &str) -> bool {
        if !self.regexs.is_empty() {
            match self.re_search_type {
                ReSearchType::Inclusive => self.regexs.iter().any(|e| e.is_match(table_name)),
                ReSearchType::Exclusive => !self.regexs.iter().all(|e| e.is_match(table_name)),
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
                let rest = Restriction::new_exclusion(vec![String::from("hell*")]);
                vec!["hell", "helloe$", "helloa", "hell"]
                    .iter()
                    .all(|e| rest.clone().verify_table_name(e))
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
