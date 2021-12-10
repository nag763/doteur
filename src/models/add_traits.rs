/// Trait to facilitate the triming of white spaces
pub trait Trim {
    /// Trim leading and trailing whitespace.
    fn trim_leading_trailing(&self) -> String;
}

/// Replace characters that can set issues for the dot file.
pub trait Replacable {
    /// Remove all non ascii chars or digits.
    fn replace_specials(&self) -> String;
    /// Remove all backquotes.
    fn replace_bq(&self) -> String;
}

/// Gets the last char
pub trait LastChar {
    /// Returns the last char
    fn get_last_char(&self) -> char;
}

/// Splits a string or str with a given vec
pub trait SplitVec {
    /// Split the vec of usize
    fn split_vec(&self, indexes: Vec<usize>) -> Vec<&str>;
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
    fn replace_bq(&self) -> String {
        str::replace(self, "`", "")
    }
}

impl Replacable for String {
    fn replace_specials(&self) -> String {
        self.chars().filter(|c| c.is_ascii_alphanumeric() || c.is_ascii_whitespace()).collect::<String>()
    }
    fn replace_bq(&self) -> String {
        str::replace(self, "`", "")
    }
}

impl LastChar for String {
    fn get_last_char(&self) -> char {
        self.chars().last().unwrap()
    }
}

impl SplitVec for String {
    fn split_vec(&self, indexes : Vec<usize>) -> Vec<&str> {
        if indexes.is_empty() {
            vec![self]
        } else {
            let mut ret : Vec<&str> = Vec::new();
            indexes.iter().enumerate().for_each(|(i, x)| {
                match i {
                    0 => ret.push(&self[0..*x]),
                    _ => ret.push(&self[indexes[i-1]+1..*x])

                }
            });
            // We push the rest
            ret.push(&self[indexes[indexes.len()-1]+1..]);
            ret
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
       assert_eq!("\n\th ell o\t\n".replace_specials(), "\n\th ell o\t\n", "white spaces are preserved");
    }

    #[test]
    fn replace_backquotes() {
        assert_eq!("\n\th ell o\t\n".replace_bq(), "\n\th ell o\t\n", "no bq no rmval");
        assert_eq!("\n`\th `ell ``o\t`\n".replace_bq(), "\n\th ell o\t\n", "bq are removed");
        assert_eq!("\n`\th \"ell \"\"''``'o\t`\n".replace_bq(), "\n\th \"ell \"\"'''o\t\n", "other quotes aren't removed");
    }
}
