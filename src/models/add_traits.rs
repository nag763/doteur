///Trim whitespaces.
pub trait Trim {
    ///Trim leading and trailing whitespaces.
    fn trim_leading_trailing(&self) -> String;
}

///Replace characters that can set issues.
pub trait Replacable {
    fn replace_specials(&self) -> String;
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
