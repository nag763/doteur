//! # General information
//! <p align="center"><img src="https://raw.githubusercontent.com/nag763/doteur/main/.github/assets/logo.png"></img></p>
//! <h2 align="center">Doteur</h2>
//! <h4 align="center">A simple tool to draw your mysql relations from exports.</h4>
//! <p align="center"><img height ="480" width="640" src="https://raw.githubusercontent.com/nag763/doteur/main/.github/assets/sample.jpeg"></img></p>
//! <u>Warning :</u> It is highly recommended to install <a href="https://graphviz.org/download/">graphviz</a> prior using this tool
//! For more information, please refer to either :
//! <ul>
//! <li><a href="https://github.com/nag763/doteur"/>Github</a></li>
//! <li><a href="https://doteur.net">The offical website</a></li>
//! <li><a href="https://docker.com/nag763/doteur">The docker repo</a></li>
//! </ul>

pub mod models;

use regex::{Regex, Captures};
use std::fs;

use models::args::{Args};
use models::restriction::{Restriction};
use models::add_traits::{Trim, LastChar, SplitVec};

use models::dot_structs::dot_table::{DotTable};
use models::dot_structs::dot_file::{DotFile};

#[macro_use] extern crate lazy_static;

lazy_static! {
    ///Look after table defs.
    static ref RE_TABLE_DEFS : Regex = Regex::new(r"(?i)\s*CREATE\s*TABLE[^;]*.").unwrap();
    ///Get table name.
    static ref RE_TABLE_NAME : Regex = Regex::new(r"(?i)\s*CREATE\s*TABLE\s*(?:IF\s*NOT\s*EXISTS)?\s*[`]?(\w*)[`]?\s*\(([^;]*)\)").unwrap();
    ///Check if foreign key exists.
    static ref RE_FK : Regex = Regex::new(r"(?i)\s*FOREIGN\s*KEY").unwrap();
    ///Check for the content in parenthesis.
    static ref RE_FK_DEF : Regex = Regex::new(r####"(?i)FOREIGN\s*KEY\s*\(["`']?(?P<table_key>\w*)["`']?\)\s*REFERENCES\s*[`"']?(?P<distant_table>\w*)[`"']?\s*\([`'"]?(?P<distant_key>\w*)[`"']?\)\s*(?:ON\s*DELETE\s*SET\s*)?(?P<on_delete>\w*)"####).unwrap();
    ///Look after alter table statements.
    static ref RE_ALTERED_TABLE : Regex = Regex::new(r"\s*(?i)ALTER\s*TABLE\s*`?(\w*)`?\s*([^;]*)").unwrap();
}


/// Detect comas in a String
///
/// # Arguments
///
/// * `content` - content to detect comas in
fn detect_comas(content : &str) -> Result<Vec<usize>, Vec<&str>> {
    let mut indexes : Vec<usize> = Vec::new();
    let mut buffer : String = String::new();
    let mut errors : Vec<&str> = Vec::new();
    content.chars().enumerate().for_each(|(i, c)|{
        match c {
            '(' => {
                // If the parenthesis isn't inside a string
                if buffer.is_empty() || buffer.get_last_char() != '`' {
                    buffer.push(c);
                }
            },
            ')' => {
                if !buffer.is_empty() {
                    let last_char : char = buffer.get_last_char();
                    if last_char == '(' {
                            buffer.pop();
                    } else if last_char != '`' {
                        errors.push("Parenthesis don't match");
                    }
                } else {
                    errors.push("Closing parenthesis without opening parenthesis");
                }
            },
            '`' => {
                if !buffer.is_empty() {
                    let last_char : char = buffer.get_last_char();
                    if last_char == '`' {
                        buffer.pop();
                    } else if last_char == '(' {
                        buffer.push(c);
                    // If a back tick is neither a closure nor a declaration
                    } else {
                        errors.push("Malformed, single backtick");
                    }
                } else {
                    buffer.push(c)
                }
            },
            ',' => {
                if buffer.is_empty() {
                    indexes.push(i);
                }
            },
            _ => ()
        }
    } );
    match errors.is_empty() {
        true => Ok(indexes),
        false => Err(errors)
    }
}

/// Get the tables from the input
///
/// # Arguments
///
/// * `input` - The content where sql table are stored
fn get_tables(input: &str) -> Vec<&str> {
    RE_TABLE_DEFS.find_iter(input)
            .map(|element| element.as_str())
            .collect::<Vec<&str>>()
}


/// Check if the given input contains sql tables
///
/// # Arguments
///
/// * `input` - The content where sql table are stored
pub fn contains_tables(input: &str) -> bool {
    !get_tables(input).is_empty()
}

/// Convert a sql table to a dot table and store it in the given dot file
///
/// # Arguments
///
/// * `dot_file` - A mutable dot file
/// * `input` - The content to convert
/// * `restrictions` - The restriction to apply on the table
/// * `dark_mode` - Changes the rendering of the output file
fn convert_sql_to_dot(dot_file : &mut DotFile, input: &str, restrictions : Option<&Restriction>, dark_mode: bool) -> Result<&'static str, &'static str> {

    let captures : Captures = RE_TABLE_NAME.captures(input).unwrap();
    let table_name : String = captures
                                  .get(1)
                                  .unwrap()
                                  .as_str()
                                  .trim_leading_trailing();


    // TODO : first depth et si relations pas nulles
    if let Some(restriction) = restrictions {
        if !restriction.clone().verify_table_name(table_name.as_str()) {
            return Err("Table doesn't match the restrictions");
        }
    }

    let mut dot_table : DotTable = DotTable::new(table_name.as_str(), dark_mode);

    let attr_defs : String = captures.get(2).unwrap().as_str().trim_leading_trailing();
    let lines : Vec<&str>;

    match detect_comas(attr_defs.as_str()) {
        Ok(v) => lines = attr_defs.split_vec(v),
        Err(_) => {dot_file.add_table(dot_table); return Err("Attributes malformed");},
    }

    lines.iter().for_each(|s| {let _ = generate_attributes(&mut dot_table, s); let _ = generate_relations(dot_file, &table_name, s, restrictions);});
    dot_file.add_table(dot_table);
    Ok("Attributes")
}

/// Write the output to the given file
///
/// # Arguments
///
/// * `content` - The content to write
/// * `filename` - The output file
pub fn write_output_to_file(content: &str, filename: &str) -> std::io::Result<()>{
    fs::write(filename ,content)?;
    Ok(())
}

/// Generate the attributes and write them into the dot_table
///
/// # Arguments
///
/// * `dot_table` - A mutable DotTable object where the attributes will be written
/// * `attr` - The attributes as string
fn generate_attributes(dot_table : &mut DotTable, attr: &str) -> Result<&'static str, &'static str>{
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
        dot_table.add_attribute(title.as_str(), rest.as_str());
        Ok("Attribute")
    } else if RE_FK.find_iter(attr).count() != 0 {
        let captures : Captures = RE_FK_DEF.captures(attr).unwrap();
        dot_table.add_attribute_fk(
            captures.name("table_key").unwrap().as_str(), 
            captures.name("distant_table").unwrap().as_str(), 
            captures.name("distant_key").unwrap().as_str()
        );
        Ok("FK Attribute")
    } else {
        Err("Not an attribute")
    }
}

/// Generates the relations and write them into the DotFile
///
/// # Arguments
///
/// * `dot_file` - Where the content should be written in
/// * `table_name` - The name of the table where the relations originates
/// * `input` - Where the relations are written
/// * `restrictive_regex` - The restrictions to apply
fn generate_relations(dot_file : &mut DotFile, table_name : &str, input: &str, restrictive_regex : Option<&Restriction>) -> Result<&'static str, &'static str> {
    if RE_FK.find_iter(input).count() != 0 {
        let captures : Captures = RE_FK_DEF.captures(input).unwrap();
        if captures.len() == 5 {
            let table_end : &str = captures.name("distant_table").unwrap().as_str();
            if let Some(restriction) = restrictive_regex {
                if vec![table_name ,table_end].iter().all(|element| restriction.clone().verify_table_name(element)){
                    dot_file.add_relation(
                        table_name, 
                        table_end, 
                        captures.name("table_key").unwrap().as_str(), 
                        captures.name("distant_key").unwrap().as_str()
                    );
                    return Ok("Match restrictions, relations added");
                } else {
                    return Err("Doesn't match restrictions");
                }
            } else {
                    dot_file.add_relation(table_name, table_end, captures.name("table_key").unwrap().as_str(), captures.name("distant_key").unwrap().as_str());
                return Ok("Relation added");
            }
        }
    }
    Err("Not a relation")
}

/// Process the given file and return the output dot string
///
/// # Arguments
///
/// * `args` - The CLI args
pub fn process_file(args : Args) -> String {

    let mut dot_file : DotFile = DotFile::new(args.get_filename_without_specials().as_str(), args.get_dark_mode());

    // Generate content from the declared tables.
    get_tables(args.get_filecontent()).iter().for_each(|element| {let _ = convert_sql_to_dot(&mut dot_file, element, args.get_restrictions(), args.get_dark_mode());});

    // Look after the other fks, declared on alter table statements.
    RE_ALTERED_TABLE.captures_iter(args.get_filecontent())
                    .for_each(|element|
                        {
                            let _ = generate_relations(
                                &mut dot_file,
                                element.get(1).unwrap().as_str(),
                                element.get(2).unwrap().as_str(),
                                args.get_restrictions()
                            );
                        }
                    );

    // Returns the content generated
    dot_file.to_string()
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
        assert_eq!(RE_FK_DEF.find_iter("FOREIGN KEY (PersonID) REFERENCES Persons(PersonID)").count(), 2, "normal");
        let matches : Vec<&str> = RE_FK_DEF.find_iter("FOREIGN KEY (PersonID) REFERENCES Persons(PersonID)").map(|s| s.as_str()).collect();
        assert_eq!(matches.get(0).unwrap(), &"KEY (PersonID)", "normal");
        assert_eq!(matches.get(1).unwrap(), &"Persons(PersonID)", "normal");

        assert_eq!(RE_FK_DEF.find_iter("FOREIGN KEY (`PersonID`) REFERENCES `Persons`(`PersonID`)").count(), 2, "normal with backquotes");
        let matches2 : Vec<&str> = RE_FK_DEF.find_iter("FOREIGN KEY (`PersonID`) REFERENCES `Persons`(`PersonID`)").map(|s| s.as_str()).collect();
        assert_eq!(matches2.get(0).unwrap(), &"KEY (`PersonID`)", "normal with backquotes");
        assert_eq!(matches2.get(1).unwrap(), &"`Persons`(`PersonID`)", "normal with backquotes");

        let captures = RE_FK_DEF.captures_iter("FOREIGN KEY (`PersonID`) REFERENCES `Persons`(`PersonID`)").map(|matched| (matched.get(1).unwrap().as_str(), matched.get(2).unwrap().as_str())).collect::<Vec<(&str, &str)>>();
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
