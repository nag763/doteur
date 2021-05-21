use regex::Regex;

#[derive(Clone)]
pub enum ReSearchType {
    Inclusive,
    Exclusive
}

///From a String makes a regex.
fn str_to_regex(input : &str) -> Result<regex::Regex, regex::Error> {
    Regex::new(format!("^{}$", input.replace('*', ".*")).as_str())
}

#[derive(Clone)]
pub struct Restriction {
    regexs : Vec<Regex>,
    re_search_type : ReSearchType
}

impl Restriction {
        fn new(re_string : Vec<String>, re_search_type : ReSearchType) -> Restriction {
            let mut regexs : Vec<Regex> = Vec::new();
            re_string.iter().for_each(|element| if let Ok(value) = str_to_regex(element) { regexs.push(value) });

            Restriction {regexs, re_search_type}
        }

        pub fn new_inclusion(re_string : Vec<String>) -> Restriction {
            Restriction::new(re_string, ReSearchType::Inclusive)
        }

        pub fn new_exclusion(re_string : Vec<String>) -> Restriction {
            Restriction::new(re_string, ReSearchType::Exclusive)
        }

        pub fn verify_table_name(self, table_name : &str) -> bool {
            match self.re_search_type {
                ReSearchType::Inclusive => self.regexs.iter().any(|e| e.is_match(table_name)),
                ReSearchType::Exclusive => !self.regexs.iter().all(|e| e.is_match(table_name))
            }
        }
}
