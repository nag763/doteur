pub mod add_trait;

use std::path::Path;
use std::ffi::OsStr;
use regex::Regex;
use std::fs;

use add_trait::{Trim, ReSearchType, ReSearch};

#[macro_use] extern crate lazy_static;


lazy_static! {
    ///Look after table defs.
    static ref RE_TABLE_DEFS : Regex = Regex::new(r"(?i)\s*CREATE\s*TABLE[^;]*.").unwrap();
    ///Get table name.
    static ref RE_TABLE_NAME : Regex = Regex::new(r"(?i)\s*CREATE\s*TABLE\s*(?:IF\s*NOT\s*EXISTS)?\s*[`]?(\w*).").unwrap();
    ///Check if foreign key exists.
    static ref RE_FK : Regex = Regex::new(r"(?i)\s*FOREIGN\s*KEY").unwrap();
    ///Check for the content in parenthesis.
    static ref RE_IN_PARENTHESES : Regex = Regex::new(r"[`]?(\w*)[`]?\s*(?:\(`?([^()`]+)`?\))").unwrap();
    ///Split on coma.
    static ref RE_SEP_COMA : Regex = Regex::new(r",\s").unwrap();
    ///Look after alter table statements.
    static ref RE_ALTERED_TABLE : Regex = Regex::new(r"\s*(?i)ALTER\s*TABLE\s*`?(\w*)`?\s*([^;]*)").unwrap();
}

///Get the tables from the input.
fn get_tables(input: &str) -> Vec<&str> {
    RE_TABLE_DEFS.find_iter(input)
            .map(|element| element.as_str())
            .collect::<Vec<&str>>()
}

///Check if the given input has declared tables.
pub fn contains_tables(input: &str) -> bool {
    !get_tables(input).is_empty()
}


///Convert sql table to dot output.
fn convert_sql_to_dot(input: &str, restrictive_regex : Option<&(Vec<Regex>, ReSearchType)>) -> (String, String) {
    let table_name = RE_TABLE_NAME.captures(input)
                                  .unwrap()
                                  .get(1)
                                  .unwrap()
                                  .as_str()
                                  .trim_leading_trailing();

    if let Some(restriction) = restrictive_regex {
        if !table_name.regex_search(&restriction.0, &restriction.1) {
            return (String::from(""), String::from(""));
        }
    }

    let table_header : String = generate_table_header(table_name.as_str());

    let begin_dec : usize;
    let end_dec : usize;
    // If the table is empty
    match input.find('('){
        Some(v) => begin_dec = v,
        None => return (close_table(table_header.as_str()), "".to_string())
    }
    match input.rfind(')') {
        Some(v) => end_dec = v,
        None => return (close_table(table_header.as_str()), "".to_string())
    }
    //
    let lines : Vec<String> = RE_SEP_COMA
        .split(input
            .chars()
            .take(end_dec)
            .skip(begin_dec+1)
            .collect::<String>()
            .as_str())
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let generated : Vec<(String, Option<String>)> = lines
        .iter()
        .map(|s| (generate_attributes(s), generate_relations(&table_name, s, restrictive_regex)))
        .collect::<Vec<(String, Option<String>)>>();

    let body_content : String = generated
        .iter()
        .map(|s| s.0.as_str())
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>()
        .join("\n\n");

    let relations : String = generated
        .iter()
        .map(|s| s.1.as_deref().unwrap_or_default())
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>()
        .join("\n");

    (close_table([table_header, body_content].join("\n").as_str()), relations)
}


///Create dot file header.
fn init_dot(filename: &str) -> String {
    format!("//This file has been generated with sqltodot, enjoy!
digraph {} {{\n
    node [\n
        shape = \"plaintext\"
    ]\n\n", Path::new(filename).file_stem().unwrap_or_else(|| OsStr::new("sql")).to_str().unwrap_or("sql"))
}

///Close dot file properly.
fn close_dot(opened_dot: &str) -> String {
    format!("{}\n}}", opened_dot)
}


///Write the output in the given file.
pub fn write_output_to_file(content: &str, filename: &str) -> std::io::Result<()>{
    fs::write(filename ,content)?;
    Ok(())
}


///Generate the .dot table header.
fn generate_table_header(name: &str) -> String {
    format!("
    {0} [label=<
        <TABLE BGCOLOR=\"white\" BORDER=\"1\" CELLBORDER=\"0\" CELLSPACING=\"0\">

        <TR><TD COLSPAN=\"2\" CELLPADDING=\"5\" ALIGN=\"CENTER\" BGCOLOR=\"blue\">
        <FONT FACE=\"Roboto\" COLOR=\"white\" POINT-SIZE=\"10\">
        <B>{0}</B>
        </FONT></TD></TR>", name)
}


///Close a .dot table.
fn close_table(table: &str) -> String {
    format!("{}\n\n\t</TABLE> >]\n", table)
}


///Generate the .dot attributes for the given input.
fn generate_attributes(attr: &str) -> String {
    //If the attribute is not a key.
    if !attr.to_lowercase().contains("key") {
        let title : String;
        let rest : String;
        let trimed : String = attr.trim_leading_trailing();
        //If it contains back coma, remove it.
        if trimed.chars().collect::<Vec<char>>()[0] == '`' {
            let splitted = trimed
                .split('`')
                .map(|s| s.to_string())
                .collect::<Vec<String>>();
            title = splitted[1].to_string();
            rest = splitted[2].trim_leading_trailing();
        } else {
            let mut splitted = trimed
                .split(' ')
                .map(|s| s.to_string())
                .collect::<Vec<String>>();
            title = splitted.remove(0);
            rest = splitted.join(" ");
        }
        format!("
        <TR><TD ALIGN=\"LEFT\" BORDER=\"0\">
        <FONT FACE=\"Roboto\"><B>{0}</B></FONT>
        </TD><TD ALIGN=\"LEFT\">
        <FONT FACE=\"Roboto\">{1}</FONT>
        </TD></TR>", title.trim_leading_trailing(), rest.trim_leading_trailing()
        )
    } else {
        let is_fk : bool = RE_FK.find_iter(attr).count() != 0;
        // If the key is a foreign key, write it.
        if is_fk {
            let captures : Vec<(&str, &str)> = RE_IN_PARENTHESES.captures_iter(attr)
                                                    .map(|matched| (matched.get(1).unwrap().as_str(), matched.get(2).unwrap().as_str()))
                                                    .collect::<Vec<(&str, &str)>>();
            // The tabs here are for the output file.
            format!("
        <TR><TD ALIGN=\"LEFT\" BORDER=\"0\">
        <FONT FACE=\"Roboto\"><B>[FK] {0}</B></FONT>
        </TD><TD ALIGN=\"LEFT\">
        <FONT FACE=\"Roboto\">Refers to <I>{1}[{2}]</I></FONT>
        </TD></TR>", captures[0].1, captures[1].0, captures[1].1
            )
        //If not, write an empty string.
        } else {
            "".to_string()
        }
    }
}


///Generate relations from the given inputs.
fn generate_relations(table_name : &str, input: &str, restrictive_regex : Option<&(Vec<Regex>, ReSearchType)>) -> Option<String> {
    let is_fk : bool = RE_FK.find_iter(input).count() != 0;
    // No PK support yet.
    if is_fk {
        let captures : Vec<(&str, &str)> = RE_IN_PARENTHESES.captures_iter(input)
                                                .map(|matched| (matched.get(1).unwrap().as_str(), matched.get(2).unwrap().as_str()))
                                                .collect::<Vec<(&str, &str)>>();
        if captures.len() == 2 {
            let table_end : &str = captures[1].0;
            if let Some(restriction) = restrictive_regex {
                if vec![table_name ,table_end].iter().all(|element| element.regex_search(&restriction.0, &restriction.1)) {
                    Some(format!("\t{0} -> {1} [label=\"Key {2} refers {3}\", arrowhead = \"dot\"]", table_name, table_end, captures[0].1, captures[1].1))
                } else {
                    None
                }
            } else {
                Some(format!("\t{0} -> {1} [label=\"Key {2} refers {3}\", arrowhead = \"dot\"]", table_name, table_end, captures[0].1, captures[1].1))
            }
        } else {
            None
        }
    } else {
        None
    }
}


///Process the given filename and content to generate a
///.dot file.
pub fn process_file(filename: &str, content: &str, restrictions : Option<(Vec<&str>, ReSearchType)>) -> String {

    let restrictive_regex : Option<(Vec<Regex>, ReSearchType)>;

    if let Some(rest) = restrictions {
        restrictive_regex = Some((
            rest.0.iter()
                  .map(|element| str_to_regex(element).unwrap_or_else(|_| Regex::new("").unwrap()))
                  .filter(|element| element.as_str() != "")
                  .collect::<Vec<Regex>>(),
            rest.1
        ));
    } else {
        restrictive_regex = None;
    }
    // Generate content from the declared tables.
    let generated_content : Vec<(String, String)> = get_tables(content).iter()
                                                                       .map(|element| convert_sql_to_dot(element, restrictive_regex.as_ref()))
                                                                       .filter(|(element1 , _)| !element1.is_empty())
                                                                       .collect::<Vec<(String, String)>>();

    // Look after the other fks, declared on alter table statements.
    let other_relations : Vec<String> = RE_ALTERED_TABLE.captures_iter(content)
                                                        .map(|element|
                                                            generate_relations(
                                                                element.get(1).unwrap().as_str(),
                                                                element.get(2).unwrap().as_str(),
                                                                restrictive_regex.as_ref()
                                                            ).unwrap_or_default()
                                                        )
                                                        .filter(|s| !s.is_empty())
                                                        .collect::<Vec<String>>();

    // Returns the content generated
    close_dot(
        [
            init_dot(filename),
            generated_content.iter()
                             .map(|element| element.0.as_str())
                             .collect::<Vec<&str>>()
                             .join("\n"),
            generated_content.iter()
                             .map(|element| element.1.as_str())
                             .chain(other_relations.iter().map(|element| element.as_str()))
                             .collect::<Vec<&str>>()
                             .join("\n"),
        ].concat().as_str()
    )
}

///From a String makes a regex.
fn str_to_regex(input : &str) -> Result<regex::Regex, regex::Error> {
    Regex::new(input.replace('*', ".*").as_str())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_re_table_defs() {
        assert_ne!(RE_TABLE_DEFS.find_iter("\nCREATE TABLE HELLO();").count(), 0, "with leading");
        assert_ne!(RE_TABLE_DEFS.find_iter("\n\tCREATE TABLE HELLO();").count(), 0, "with leading");
        assert_ne!(RE_TABLE_DEFS.find_iter("\nCREATE TABLE `HELLO`();").count(), 0, "with backquotes");
        assert_ne!(RE_TABLE_DEFS.find_iter("\n\tCReaTe TabLe HELLO();").count(), 0, "non capital letters");
        assert_ne!(RE_TABLE_DEFS.find_iter("CREATE TABLE   \t HELLO();").count(), 0, "several spaces between");
        assert_ne!(RE_TABLE_DEFS.find_iter("\tCREATE\t\t TABLE   \t HELLO();").count(), 0, "several spaces between");
        assert_ne!(RE_TABLE_DEFS.find_iter("CREATE \n\tTABLE \n \t HELLO();").count(), 0, "several backline between");
        assert_ne!(RE_TABLE_DEFS.find_iter("CREATE \n\tTABLE \n \t HELLO();").count(), 0, "several backline between");
        assert_ne!(RE_TABLE_DEFS.find_iter("CREATE TABLE IF NOT EXISTS HELLO();").count(), 0, "if not exists");

        assert_eq!(RE_TABLE_DEFS.find_iter("CREATE TABL HELLO();").count(), 0, "typo");
        assert_eq!(RE_TABLE_DEFS.find_iter("CRATE TABLE HELLO();").count(), 0, "typo");
        assert_eq!(RE_TABLE_DEFS.find_iter("CREATE OR TABLE HELLO();").count(), 0, "wrong keyword");
        assert_eq!(RE_TABLE_DEFS.find_iter("CREATE DATABASE HELLO();").count(), 0, "wrong keyword");
        assert_eq!(RE_TABLE_DEFS.find_iter("DROP TABLE HELLO();").count(), 0, "wrong keyword");
        assert_eq!(RE_TABLE_DEFS.find_iter("ALTER TABLE HELLO();").count(), 0, "wrong keyword");
    }

    #[test]
    fn test_re_table_name() {
        assert_eq!(RE_TABLE_NAME.captures("CREATE TABLE HELLO();").unwrap().get(1).unwrap().as_str(), "HELLO", "normal");
        assert_eq!(RE_TABLE_NAME.captures("CREATE TABLE `HELLO`();").unwrap().get(1).unwrap().as_str(), "HELLO", "with backquotes");
        assert_eq!(RE_TABLE_NAME.captures("CREATE TABLE IF NOT EXISTS `HELLO`();").unwrap().get(1).unwrap().as_str(), "HELLO", "with backquotes");
        assert_eq!(RE_TABLE_NAME.captures("CREATE TABLE If NoT EXIsTS HELLO();").unwrap().get(1).unwrap().as_str(), "HELLO", "with backquotes and mixed");
        assert_eq!(RE_TABLE_NAME.captures("\t\nCREATE\t\n TABLE\t\n `HELLO`\t();").unwrap().get(1).unwrap().as_str(), "HELLO", "with separative sequences");
        assert_eq!(RE_TABLE_NAME.captures("\t\nCreATE\t\n TaBle\t\n `HeLlO`();").unwrap().get(1).unwrap().as_str(), "HeLlO", "mixed");
    }

    #[test]
    fn test_re_fk() {
        assert_eq!(RE_FK.find_iter("ADD FOREIGN KEY (PersonID) REFERENCES Persons(PersonID)").count(), 1, "normal");
        assert_eq!(RE_FK.find_iter("ADD FOREIGN KEY (`PersonID`) REFERENCES `Persons`(`PersonID`)").count(), 1, "normal");
        assert_eq!(RE_FK.find_iter("FOREIGN KEY (PersonID) REFERENCES Persons(PersonID);").count(), 1, "normal");
        assert_eq!(RE_FK.find_iter("FOREIGN KEY (`PersonID`) REFERENCES `Persons`(`PersonID`)").count(), 1, "normal");
        assert_eq!(RE_FK.find_iter("\n\tFOREIGN\t\n \t\nKEY \n\t(`PersonID`) REFERENCES `Persons`(`PersonID`)").count(), 1, "with spaces");
        assert_eq!(RE_FK.find_iter("\n\tForeIgN\t\n \t\nkeY (`PersonID`) REFERENCES `Persons`(`PersonID`)").count(), 1, "mixed");

        assert_ne!(RE_FK.find_iter("ADD PRIMARY KEY (PersonID) REFERENCES Persons(PersonID)").count(), 1, "wrong key");
        assert_ne!(RE_FK.find_iter("ADD UNIQUE KEY (PersonID) REFERENCES Persons(PersonID)").count(), 1, "wrong key");
    }

    #[test]
    fn test_re_in_parenthesis() {
        assert_eq!(RE_IN_PARENTHESES.find_iter("FOREIGN KEY (PersonID) REFERENCES Persons(PersonID)").count(), 2, "normal");
        let matches : Vec<&str> = RE_IN_PARENTHESES.find_iter("FOREIGN KEY (PersonID) REFERENCES Persons(PersonID)").map(|s| s.as_str()).collect();
        assert_eq!(matches.get(0).unwrap(), &"KEY (PersonID)", "normal");
        assert_eq!(matches.get(1).unwrap(), &"Persons(PersonID)", "normal");

        assert_eq!(RE_IN_PARENTHESES.find_iter("FOREIGN KEY (`PersonID`) REFERENCES `Persons`(`PersonID`)").count(), 2, "normal with backquotes");
        let matches2 : Vec<&str> = RE_IN_PARENTHESES.find_iter("FOREIGN KEY (`PersonID`) REFERENCES `Persons`(`PersonID`)").map(|s| s.as_str()).collect();
        assert_eq!(matches2.get(0).unwrap(), &"KEY (`PersonID`)", "normal with backquotes");
        assert_eq!(matches2.get(1).unwrap(), &"`Persons`(`PersonID`)", "normal with backquotes");

        let captures = RE_IN_PARENTHESES.captures_iter("FOREIGN KEY (`PersonID`) REFERENCES `Persons`(`PersonID`)").map(|matched| (matched.get(1).unwrap().as_str(), matched.get(2).unwrap().as_str())).collect::<Vec<(&str, &str)>>();
        assert_eq!(captures[0].0, "KEY", "normal with backquotes");
        assert_eq!(captures[0].1, "PersonID", "normal with backquotes");
        assert_eq!(captures[1].0, "Persons", "normal with backquotes");
        assert_eq!(captures[1].1, "PersonID", "normal with backquotes");
    }

    #[test]
    fn test_re_alter_table() {
        assert_eq!(RE_ALTERED_TABLE.find_iter("ALTER TABLE HELLO ADD FOREIGN KEY (PersonID) REFERENCES artists (id) ;").count(), 1, "normal");
        let captures = RE_ALTERED_TABLE.captures("ALTER \t\nTABLE HELLO ADD FOREIGN KEY (PersonID) REFERENCES artists (id) ;").unwrap();
        assert_eq!(captures.get(1).unwrap().as_str(), "HELLO", "normal");
        assert_eq!(captures.get(2).unwrap().as_str(), "ADD FOREIGN KEY (PersonID) REFERENCES artists (id) ", "normal");

        assert_eq!(RE_ALTERED_TABLE.find_iter("ALTER TABLE `HELLO` ADD FOREIGN KEY (`PersonID`) REFERENCES `artists` (`id`) ;").count(), 1, "normal");
        let captures2 = RE_ALTERED_TABLE.captures("ALTER TABLE `HELLO` ADD FOREIGN KEY (`PersonID`) REFERENCES `artists` (`id`) ;").unwrap();
        assert_eq!(captures2.get(1).unwrap().as_str(), "HELLO", "normal");
        assert_eq!(captures2.get(2).unwrap().as_str(), "ADD FOREIGN KEY (`PersonID`) REFERENCES `artists` (`id`) ", "normal");

    }
}
