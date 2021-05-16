use regex::Regex;

pub enum ReSearchType {
    Inclusive,
    Exclusive
}

///Trim whitespaces.
pub trait Trim {
    ///Trim leading and trailing whitespaces.
    fn trim_leading_trailing(&self) -> String;
}

///Replace characters that can set issues.
pub trait Replacable {
    fn replace_specials(&self) -> String;
}

pub trait ReSearch {
    fn regex_search(&self, regex_list : &[Regex], re_search_type : &ReSearchType) -> bool;
}


impl Trim for String {
    fn trim_leading_trailing(&self) -> String {
        self.trim_start().trim_end().to_string()
    }
}

impl Trim for str {
    fn trim_leading_trailing(&self) -> String {
        self.trim_start().trim_end().to_string()
    }
}

impl Replacable for str {
    fn replace_specials(&self) -> String {
        self.chars().filter(|c| c.is_ascii_alphanumeric() || c.is_ascii_whitespace()).collect::<String>()
    }
}

impl Replacable for String {
    fn replace_specials(&self) -> String {
        self.chars().filter(|c| c.is_ascii_alphanumeric() || c.is_ascii_whitespace()).collect::<String>()
    }
}

impl ReSearch for &str {
    fn regex_search(&self, regex_list : &[Regex], re_search_type : &ReSearchType) -> bool {
        match re_search_type {
            ReSearchType::Inclusive => regex_list.iter().any(|e| e.is_match(self)),
            ReSearchType::Exclusive => !regex_list.iter().all(|e| e.is_match(self))
        }
    }
}

impl ReSearch for String {
    fn regex_search(&self, regex_list : &[Regex], re_search_type : &ReSearchType) -> bool {
        match re_search_type {
            ReSearchType::Inclusive => regex_list.iter().any(|e| e.is_match(self)),
            ReSearchType::Exclusive => !regex_list.iter().any(|e| e.is_match(self))
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn trim_trailing() {
        assert_eq!(" hello".trim_leading_trailing(), "hello");
        assert_eq!(String::from(" hello").trim_leading_trailing(), String::from("hello"));
    }

    #[test]
    fn trim_leading() {
        assert_eq!("hello ".trim_leading_trailing(), "hello");
        assert_eq!(String::from("hello ").trim_leading_trailing(), String::from("hello"));
    }

    #[test]
    fn trim_leading_trailing() {
        assert_eq!("  hello ".trim_leading_trailing(), "hello");
        assert_eq!(String::from("  hello ").trim_leading_trailing(), String::from("hello"));
    }

    #[test]
    fn dont_trim_in() {
        assert_eq!("  he llo ".trim_leading_trailing(), "he llo");
        assert_eq!(String::from("  he llo ").trim_leading_trailing(), String::from("he llo"));
    }

    #[test]
    fn trim_other_seqs() {
        assert_eq!(" \n\t\n he llo \t\n".trim_leading_trailing(), "he llo");
        assert_eq!(String::from(" \t\n\n he llo \t\n").trim_leading_trailing(), String::from("he llo"));
    }

    #[test]
    fn replace_specials() {
        assert_eq!("h*ù$$âe🔎,;:!)l&²l<o".replace_specials(), "hello");
    }

    #[test]
    fn replace_specials_preserves_whites_spaces() {
        assert_eq!("\n\th ell o\t\n".replace_specials(), "\n\th ell o\t\n");
    }
}
