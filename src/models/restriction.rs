use regex::Regex;

/// ReSearchType indicates if the regex should be made according to the fact they match the
/// given regexs, or not.
#[derive(Clone)]
pub enum ReSearchType {
    /// If the regex is matched, the value will be true
    Inclusive,
    /// If the regex is not matched, the value will be true
    Exclusive
}

/// Transform a string with easy regex contents to a more complex regex
///
/// # Arguments
///
/// * `input` - The string to transform into regex.
fn str_to_regex(input : &str) -> Result<regex::Regex, regex::Error> {
    Regex::new(format!("^{}$", input.replace('*', ".*")).as_str())
}

/// A restriction represents a condition to render or not the given table.
#[derive(Clone)]
pub struct Restriction {
    /// The list of regexs
    regexs : Vec<Regex>,
    /// The type of restriction to apply
    re_search_type : ReSearchType
}

impl Restriction {

        /// Creates new restrictions
        ///
        /// # Arguments
        ///
        /// * `re_string` - The trivial regexs as string
        /// * `re_search_type` - How the regexs should be matches
        fn new(re_string : Vec<String>, re_search_type : ReSearchType) -> Restriction {
            let mut regexs : Vec<Regex> = Vec::new();
            re_string.iter().for_each(|element| if let Ok(value) = str_to_regex(element) { regexs.push(value) });
            Restriction {regexs, re_search_type}
        }

        /// Creates a new restriction in inclusive mode
        ///
        /// # Arguments
        ///
        /// * `re_string` - The trivial regexs as string
        pub fn new_inclusion(re_string : Vec<String>) -> Restriction {
            Restriction::new(re_string, ReSearchType::Inclusive)
        }

        /// Creates a new restriction in exclusive mode
        ///
        /// # Arguments
        ///
        /// * `re_string` - The trivial regexs as string
        pub fn new_exclusion(re_string : Vec<String>) -> Restriction {
            Restriction::new(re_string, ReSearchType::Exclusive)
        }

        /// Checks if the given inputs matches the restriction
        ///
        /// # Arguments
        ///
        /// * `table_name` - The table to verify with the given restriction
        pub fn verify_table_name(self, table_name : &str) -> bool {
            match self.re_search_type {
                ReSearchType::Inclusive => self.regexs.iter().any(|e| e.is_match(table_name)),
                ReSearchType::Exclusive => !self.regexs.iter().all(|e| e.is_match(table_name))
            }
        }
}
