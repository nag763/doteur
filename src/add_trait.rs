///Trim whitespaces.
pub trait Trim {
    ///Trim leading and trailing whitespaces.
    fn trim_leading_trailing(&self) -> String;
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
