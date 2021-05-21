use regex::Regex;
use std::fs;

pub const POSSIBLE_DOTS_OUTPUT : [&str; 54] = ["bmp", "canon", "gv", "xdot", "xdot1.2", "xdot1.4",
                                            "cgimage", "cmap", "eps", "eps", "exr", "fig", "gd",
                                            "gd2" , "gif", "gtk", "ico", "imap", "cmapx", "imap_np",
                                            "cmapx_np", "ismap", "jp2", "jpg", "jpeg", "jpe", "json",
                                            "json0", "dot_json", "xdot_json", "pct", "pict","pdf",
                                            "pic", "plain", "plain-ext", "png", "pov", "ps2", "psd",
                                            "sgi", "svg", "svgz", "tga", "tif", "tiff", "tk", "vml",
                                            "vmlz", "vrml", "wbmp", "webp", "xlib", "x11"];

///From a String makes a regex.
fn str_to_regex(input : &str) -> Result<regex::Regex, regex::Error> {
    Regex::new(format!("^{}$", input.replace('*', ".*")).as_str())
}

#[derive(Clone)]
pub struct Args {
    pub filename: String,
    pub filecontent: String,
    pub output_filename: String,
    pub restrictions: Option<(Vec<Regex>, ReSearchType)>,
}

impl Args {
    pub fn new(filename: String) -> Args {
        Args {
            filename: filename.clone(),
            filecontent: fs::read_to_string(filename.as_str()).expect("Something went wrong while reading the file"),
            output_filename: String::from("output.dot"),
            restrictions: None
        }
    }

    pub fn get_filename_without_specials(&self) -> String {
        self.filename.chars().filter(|c| c.is_ascii_alphanumeric() || c.is_ascii_whitespace()).collect::<String>()
    }

    pub fn get_file_ext(&self) -> &str {
        std::path::Path::new(self.output_filename.as_str()).extension().unwrap_or_default().to_str().unwrap_or_default()
    }

    pub fn ext_supported(&self) -> bool {
        let file_ext : &str = self.get_file_ext();
        POSSIBLE_DOTS_OUTPUT.iter().any(|&i| i == file_ext)
    }

    pub fn get_filecontent(&self) -> &str {
        self.filecontent.as_str()
    }

    pub fn get_output_filename(&self) -> &str {
        self.output_filename.as_str()
    }

    pub fn set_output_filename(&mut self, output_filename : String) {
        self.output_filename = output_filename;
    }

    pub fn get_restrictions(&self) -> Option<(Vec<Regex>, ReSearchType)> {
        match &self.restrictions {
            Some(value) => Some((value.0.clone(), value.1.clone())),
            None => None
        }
    }

    pub fn set_inclusions(&mut self, inclusions : Vec<String>) {
        self.restrictions = Some(
            (inclusions.iter()
                       .map(|element| str_to_regex(element).unwrap_or_else(|_| Regex::new("").unwrap()))
                       .filter(|element| element.as_str() != "")
                       .collect::<Vec<Regex>>(),
             ReSearchType::Inclusive)
         );
    }


    pub fn set_exclusions(&mut self, exclusions : Vec<String>) {
        self.restrictions = Some(
            (exclusions.iter()
                       .map(|element| str_to_regex(element).unwrap_or_else(|_| Regex::new("").unwrap()))
                       .filter(|element| element.as_str() != "")
                       .collect::<Vec<Regex>>(),
            ReSearchType::Exclusive)
        );
    }

}

#[derive(Clone)]
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
