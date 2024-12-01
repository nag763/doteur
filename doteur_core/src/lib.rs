//! Copyright ⓒ 2021-2024 LABEYE Loïc
//! This tool is distributed under the MIT License, check out [here](https://github.com/nag763/doteur/blob/main/LICENCE.MD).
//! # General information
//! <h2 align="center">Doteur Core</h2>
//! <h4 align="center">This library contains all the tools used to transform your SQL input into a DOT one.</h4>
//! <p align="justify">Doteur is a CLI (Command Line Interface) tool that has for purpose to render the SQL schemas into good looking graphs. This will help you to easily understand the structure of a large database and understand what happens behind the scenes of your project.</p>
//! Besides, you will be able to use the large panel of features to either sort the tables you want to visualize or render with a different color scheme for instance.
//! So far the tool handles both the MySQL and SQLite syntaxes, but it is planned to handle the Postgre one as soon as the formers will be considered as stable. The input of the tool can be either a sql file export, or given the version you downloaded, connect to either a MySQL running instance or an existing SQLite database.
//! The tool has been developed on Linux, but is also available for Windows 10 and 11 and macOS.
//! <br/>
//! <p>Useful links :</p>
//! <ul>
//! <li><a href="https://github.com/nag763/doteur"/>Github repository</a></li>
//! <li><a href="https://nag763.github.io/doteur"/>Official documentation</a></li>
//! <li><a href="https://docker.com/nag763/doteur">Docker tool</a></li>
//! </ul>

#[cfg(feature = "mysql_addons")]
/// Module used to connect to a remote MySQL running database instance
///
/// This module is only available with the `mysql_addons` feature
pub mod mysql_tools;
/// Module used to filter the tables to render
///
/// A restriction can either be inclusive, meaning that only the tables that matche the restriction
/// are rendered, or exclusive, meaning that only the tables that don't match the restrictions will be rendered
pub mod restriction;
#[cfg(feature = "sqlite_addons")]
/// Module used to connect to a SQLite database
///
/// This module is only available with the `sqlite_addons` feature
pub mod sqlite_tools;
/// Module containing different utilities
pub mod tools;

/// Module containing the additional traits
mod add_traits;
/// Module containing the different dot structures used in the code
mod dot_structs;
/// Module containing the errors thrown by the core libraries
mod errors;

use std::borrow::Cow;

use crate::add_traits::{Replacable, SplitVec, Trim};
use crate::errors::DoteurCoreError;
use crate::restriction::Restriction;
use crate::tools::detect_comas;

use dot_structs::dot_file::DotFile;
use dot_structs::dot_table::DotTable;
use dot_structs::relation::Relation;

use log::{debug, error, info, warn};
use regex::{Captures, Regex};

#[macro_use]
extern crate lazy_static;

macro_rules! unwrap_captures_name_as_str {
    ($captures:ident, $key:expr, $err:block) => {
        match $captures.name($key) {
            Some(v) => v.as_str(),
            None => $err,
        }
    };
    ($captures:ident, $key:expr, $err_label:expr) => {
        unwrap_captures_name_as_str!($captures, $key, {
            return Err(DoteurCoreError::regex_error($err_label, file!(), line!()));
        })
    };
    ($captures:ident, $key:expr) => {
        unwrap_captures_name_as_str!($captures, $key, {
            return Err(DoteurCoreError::regex_error(
                "Group not found in regex",
                file!(),
                line!(),
            ));
        })
    };
}

lazy_static! {
    ///Get table name.
    static ref RE_TABLE_NAME : Regex = Regex::new(r####"(?i)\s*CREATE\s*TABLE\s*(?:IF\s*NOT\s*EXISTS)?\s*(?:(?:\w+)\.)?(?P<table_name>(?:[`"\[]{1}[^`"\]]+[`"\]]{1})|(?:\w*))\s*\((?P<content>[^;]*)\)"####).unwrap();
    ///Get column type
    static ref RE_COL_TYPE : Regex = Regex::new(r####"(?i)\s*((?:FULLTEXT|SPATIAL)?\s+(?:INDEX|KEY|CHECK))|(?:CONSTRAINT\s*[`'"]\w*[`'"])?\s*(?P<key_type>UNIQUE|FOREIGN|PRIMARY)\s+"####).unwrap();
    ///Get columns definitioon
    static ref RE_COL_DEF : Regex = Regex::new(r####"(?i)\s*(?P<col_name>(?:[`"\[]{1}[^`"\]]+[`"\]]{1})|(?:\w*))\s*(?P<col_def>.*)"####).unwrap();
    ///Check if input is a primary key
    static ref RE_PK_DEF : Regex = Regex::new(r####"(?i)PRIMARY\s*KEY\s*["`]?(?:\w*)[`"]?\s*\((?P<col_name>[^\)]+)\)"####).unwrap();
    ///Check if a PK is declared in the line
    static ref RE_PK_IN_LINE : Regex = Regex::new(r####"(?i)\s*PRIMARY\s*KEY.*"####).unwrap();
    ///Check for the content in parenthesis.
    static ref RE_FK_DEF : Regex = Regex::new(r####"(?i)FOREIGN\s*KEY\s*(?:(?:public|private).)?\((?P<table_key>[^\)]+)\)\s*REFERENCES\s*(?:(?:public|private).)?[`"'\[]?(?P<distant_table>\w*)["`'\]]?\s*\((?P<distant_key>[^\)]+)\)\s*(?:(?:ON\s*UPDATE\s*(?:(?:SET\s*\w*|\w*))\s*)?(?:ON\s*DELETE\s*)?(?P<on_delete>(SET\s*NULL|CASCADE|RESTRICT|NO\s*ACTION|SET\s*DEFAULT)))?"####).unwrap();
    ///Look after alter table statements.
    static ref RE_ALTERED_TABLE : Regex = Regex::new(r####"\s*(?i)ALTER\s*TABLE\s*(?:ONLY)?\s*['`"\[]?(?:(?:public|private).)?(?P<table_name>\w*)[`"'\]]?\s*(?P<altered_content>[^;]*)"####).unwrap();
    ///Regex to remove comments
    static ref RE_COMMENTS : Regex = Regex::new(r####"(--.*|#.*|/\*[^*/]*\*/)"####).unwrap();
}

/// Get the tables from the input
///
/// This method will check the data given on input and will return all the tables
/// until the end of their declaration.
///
/// # Arguments
///
/// * `data` - The content where sql table are stored
fn get_tables(data: &str) -> Vec<&str> {
    RE_TABLE_NAME
        .find_iter(data)
        .map(|element| element.as_str())
        .collect::<Vec<&str>>()
}

/// Check if the given input contains sql tables
///
/// This function will with the given input ensure that the data passed contains at least one SQL table.
/// If no table is detected in the data, the function will return false, otherwise, it will return true.
///
/// # Arguments
///
/// * `data` - The content that we need to check the existence of SQL tables in.
///
/// # Example
/// ```
/// use doteur_core::contains_sql_tables;
/// // Normal use case
///assert!(contains_sql_tables("
/// CREATE TABLE foo ( bar );
///"));
///assert!(contains_sql_tables("
/// CREATE TABLE `foo` ( bar );
///"));
///assert!(contains_sql_tables("
/// CREATE TABLE `FOOBAR` ( bar );
///"));
///assert_eq!(contains_sql_tables("
/// My table ;
///"), false);
/// ```
///
pub fn contains_sql_tables(data: &str) -> bool {
    RE_TABLE_NAME.is_match(data)
}

/// Remove the SQL comments from an input
fn remove_sql_comments(data: &str) -> Cow<'_, str> {
    RE_COMMENTS.replace_all(data, "")
}

/// Convert a sql table to a dot table and store it in the given dot file
fn convert_sql_table_to_dot(
    input: &str,
    restrictions: Option<&Restriction>,
    dark_mode: bool,
) -> Result<Option<(String, DotTable, Vec<Relation>)>, DoteurCoreError> {
    let captures: Captures = RE_TABLE_NAME.captures(input).unwrap();

    let table_name: String = unwrap_captures_name_as_str!(
        captures,
        "table_name",
        "Regex error, the input is either not a sql table or isn't parsed properly by the process"
    )
    .replace_enclosing()
    .trim_leading_trailing();
    info!(
        "Starting to convert the SQL table {} into a DOT table",
        table_name
    );

    // Check restrictions, if some are present, early return if table doesn't match restrictions
    if !matches_optionable_restriction!(restrictions, &table_name) {
        return Ok(None);
    }

    let attr_defs: String = unwrap_captures_name_as_str!(
        captures,
        "content",
        "Regex error, the input is either not a sql table or isn't pared properly by the process"
    )
    .trim_leading_trailing();

    let lines: Vec<&str> = match detect_comas(attr_defs.as_str()) {
        Ok(v) => {
            debug!(
                "Table {} splitted correctly, {} unclosed comas found",
                table_name,
                v.len()
            );
            attr_defs.split_vec(v)
        }
        Err(e) => {
            error!("Error in comas parsing for table : {0}\n{1}", table_name, e);
            return Err(DoteurCoreError::user_input_malformed(
                "Attributes malformed",
            ));
        }
    };

    let mut dot_table: DotTable = DotTable::new(table_name.as_str(), dark_mode);
    let mut relations: Vec<Relation> = Vec::new();

    for line in lines {
        // If column type is common attribute
        if !RE_COL_TYPE.is_match(line) {
            debug!(
                "Line {} is an attribute definition",
                line.trim_leading_trailing()
            );
            match generate_attributes(&mut dot_table, line) {
                Ok(col_name) => info!(
                    "Attribute {} processed correctly and added to table {}",
                    col_name, table_name
                ),
                Err(e) => error!("An error happened while processing line : {}", e),
            }
        // If column type is a relation or an index
        } else {
            debug!(
                "Line {} has been deteceted has a relation definition",
                line.trim_leading_trailing()
            );
            let col_type: Captures = match RE_COL_TYPE.captures(line) {
                Some(v) => v,
                None => {
                    error!("Regex error for line - capture not succesfull");
                    continue;
                }
            };

            let key_type: String = unwrap_captures_name_as_str!(col_type, "key_type", {
                warn!("Key type isn't handled, line will be ignored");
                continue;
            })
            .to_uppercase();
            match key_type.as_str() {
                "FOREIGN" => {
                    debug!(
                        "Line {} has been found as a foreign key def",
                        line.trim_leading_trailing()
                    );
                    match generate_relations(table_name.as_str(), line, restrictions) {
                        Ok(v) => {
                            // If the relations matched the restrictions
                            if let Some(relation) = v {
                                // Add them to the function buffer
                                relations.push(relation.clone());
                                debug!("{} relations have been added following the processing of the line {}", relation.get_number_of_pairs_of_keys(), line.trim_leading_trailing());
                                // And add the FK nature to the attributes in table
                                for pair_key_refered in relation.get_pairs_of_keys() {
                                    match dot_table.add_fk_nature_to_attribute(
                                        pair_key_refered.0.as_str(),
                                        relation.get_refered_table(),
                                        pair_key_refered.1.as_str(),
                                    ) {
                                        Ok(_) => info!("Attribute {} of table {} has been detected as FK", pair_key_refered.0, table_name),
                                        Err(e) => error!("An error happened while adding the FK nature of attribute {} to the table {} : {}", pair_key_refered.0, table_name, e)
                                    };
                                }
                            }
                        }
                        Err(e) => {
                            error!("An error happened while processing the foreign key: {}", e)
                        }
                    }
                }
                "PRIMARY" => {
                    if !RE_PK_DEF.is_match(line) {
                        debug!(
                            "Line {} has been found as a primary key definition including an attribute definition",
                            line.trim_leading_trailing()
                        );
                        match generate_attributes(&mut dot_table, line) {
                            Ok(col_name) => info!(
                                "PK {} with attribute definition added to table {}",
                                col_name, table_name
                            ),
                            Err(e) => {
                                error!("An error happened while processing primary key : {}", e)
                            }
                        }
                    } else {
                        debug!(
                            "Line {} has been found as a primary key def",
                            line.trim_leading_trailing()
                        );
                        match generate_primary(&mut dot_table, line) {
                            Ok(m) => info!(
                                "PK(s) processed correctly {} and added to the table {}",
                                m, table_name
                            ),
                            Err(e) => {
                                error!("An error happened while processing primary key : {}", e)
                            }
                        }
                    }
                }
                _ => warn!("The line didn't match any known relation type"),
            }
        }
    }
    info!("The table {} has been processed with success", table_name);
    Ok(Some((table_name, dot_table, relations)))
}

/// Generate the attributes and write them into the dot_table
fn generate_attributes(dot_table: &mut DotTable, attr: &str) -> Result<String, DoteurCoreError> {
    let col_name: String;
    // If a PK is present in line, process attribute as pk
    if RE_PK_IN_LINE.is_match(attr) {
        let trimmed_line: &str = &RE_PK_IN_LINE.replace(attr, "");
        let captures: Captures = RE_COL_DEF.captures(trimmed_line).unwrap();
        col_name = unwrap_captures_name_as_str!(captures, "col_name")
            .replace_enclosing()
            .trim_leading_trailing();
        dot_table.add_attribute_pk(
            col_name.as_str(),
            unwrap_captures_name_as_str!(captures, "col_def"),
        );
        Ok(col_name)
    // Otherwise, process as atribute
    } else {
        let captures: Captures = RE_COL_DEF.captures(attr).unwrap();
        col_name = unwrap_captures_name_as_str!(captures, "col_name")
            .replace_enclosing()
            .trim_leading_trailing();
        dot_table.add_attribute(
            col_name.as_str(),
            unwrap_captures_name_as_str!(captures, "col_def"),
        );
        Ok(col_name)
    }
}

/// Generate the attributes as primary and write them into the table
fn generate_primary(dot_table: &mut DotTable, line: &str) -> Result<String, DoteurCoreError> {
    // Assert that the line matches regex and get the captures
    let captures: Captures = match RE_PK_DEF.captures(line) {
        Some(captures) => captures,
        None => {
            return Err(DoteurCoreError::regex_error(
                "Input error",
                file!(),
                line!(),
            ))
        }
    };
    // Check that the group column name has been captured, and detect the comas within
    let (col_name, comas_detected): (String, Result<Vec<usize>, &str>) =
        match captures.name("col_name") {
            Some(v) => (v.as_str().to_string(), detect_comas(v.as_str())),
            None => {
                return Err(DoteurCoreError::regex_error(
                    "Input is not a primary key",
                    file!(),
                    line!(),
                ))
            }
        };
    match comas_detected {
        //If severeal comas are detected
        Ok(comas_vec) if !comas_vec.is_empty() => {
            for attr in col_name.split_vec(comas_vec) {
                dot_table.add_pk_nature_to_attribute(
                    attr.replace_enclosing().trim_leading_trailing().as_str(),
                )?;
            }
        }
        // If no comas are detected
        _ => {
            dot_table.add_pk_nature_to_attribute(
                col_name
                    .replace_enclosing()
                    .trim_leading_trailing()
                    .as_str(),
            )?;
        }
    }
    Ok(col_name)
}

/// Returns the relations from an input
fn generate_relations(
    table_name: &str,
    line: &str,
    restrictive_regex: Option<&Restriction>,
) -> Result<Option<Relation>, DoteurCoreError> {
    // If the regex doesn't match the input, early return
    if !RE_FK_DEF.is_match(line) {
        return Err(DoteurCoreError::regex_error(
            "Input isn't a relation",
            file!(),
            line!(),
        ));
    }

    let captures: Captures = match RE_FK_DEF.captures(line) {
        Some(v) => v,
        None => {
            return Err(DoteurCoreError::regex_error(
                "Capture error",
                file!(),
                line!(),
            ))
        }
    };

    let distant_table: &str = unwrap_captures_name_as_str!(captures, "distant_table");

    // If one of the tables doesn't match any of the restrictions, early return
    if !matches_optionable_restriction!(restrictive_regex, table_name, distant_table) {
        info!("One of the two tables doesn't match the restrictions");
        return Ok(None);
    }

    // Bind the common variables used later
    let table_key: String = unwrap_captures_name_as_str!(captures, "table_key").replace_enclosing();
    let distant_key: String =
        unwrap_captures_name_as_str!(captures, "distant_key").replace_enclosing();
    let relation_type: &str = captures
        .name("on_delete")
        .map_or("RESTRICT", |m| m.as_str());

    // Process the input
    match detect_comas(table_key.as_str()) {
        // Case where attributes are separated by comas
        Ok(comas_vec) if !comas_vec.is_empty() => {
            match detect_comas(distant_key.as_str()) {
                // If both vec are the same size, then the nth key of vec1 matches nth key of vec2
                Ok(second_coma_vec)
                    if !second_coma_vec.is_empty() && second_coma_vec.len() == comas_vec.len() =>
                {
                    let mut relation: Relation = Relation::new(
                        table_name.to_string(),
                        distant_table.to_string(),
                        relation_type.to_string(),
                    );
                    let vec_table_key: Vec<&str> = table_key.split_vec(comas_vec.clone());
                    let vec_distant_key: Vec<&str> = distant_key.split_vec(second_coma_vec);
                    //If we have a table as input
                    for i in 0..comas_vec.len() {
                        relation.push_pair_of_keys(
                            vec_table_key
                                .get(i)
                                .unwrap()
                                .replace_enclosing()
                                .trim_leading_trailing(),
                            vec_distant_key
                                .get(i)
                                .unwrap()
                                .replace_enclosing()
                                .trim_leading_trailing(),
                        );
                    }
                    // If we don't
                    Ok(Some(relation))
                }
                // Size of vec doesn't match, error return
                _ => Err(DoteurCoreError::user_input_malformed(
                    "Error in file format",
                )),
            }
        }
        // Single key processing
        _ => Ok(Some(Relation::new_with_single_pair(
            table_name.to_string(),
            distant_table.to_string(),
            table_key.replace_enclosing().trim_leading_trailing(),
            distant_key.replace_enclosing().trim_leading_trailing(),
            relation_type.to_string(),
        ))),
    }
}

/// Process the given file and return the output as a string
///
/// This function takes a SQL table as data and returns it as a DOT output.
///
/// # Arguments
///
/// * `data` - The SQL content as a string
/// * `restrictions` - The list of filters we want to apply on the input
/// * `legend` - Whether we add a legend describing the types of relations at the end of the file or not.
/// * `dark_mode` - Whether the output needs to be rendered in dark mode or not.
pub fn process_data(
    data: &str,
    restrictions: Option<&Restriction>,
    legend: bool,
    dark_mode: bool,
) -> String {
    let mut dot_file: DotFile = DotFile::new(legend, dark_mode);

    let cleaned_content: &str = &remove_sql_comments(data);

    info!("Starting to process the tables for the given input");
    // Generate content from the declared tables.
    for table in get_tables(cleaned_content) {
        match convert_sql_table_to_dot(table, restrictions, dark_mode) {
            Ok(result) => {
                if let Some((table_name, dot_table, relations)) = result {
                    dot_file.add_table(dot_table);
                    for relation in relations {
                        dot_file.add_relation(relation);
                    }
                    info!("Table {} added to dot file", table_name);
                } else {
                    info!("The table hasn't been added as it wasn't matching the restrictions");
                }
            }
            Err(e) => error!("An error happened while processing a table : {}", e),
        };
    }

    // Look after the other fks, declared on alter table statements.
    for element in RE_ALTERED_TABLE.captures_iter(cleaned_content) {
        // Those errors shouldn't be thrown
        let table_name: &str = unwrap_captures_name_as_str!(element, "table_name", {
            panic!("Regex error");
        });
        let altered_content: &str = unwrap_captures_name_as_str!(element, "altered_content", {
            panic!("Regex error");
        });
        match generate_relations(table_name, altered_content, restrictions) {
            Ok(v) => {
                if let Some(relation) = v {
                    dot_file.add_relation(relation);
                    info!("New relation found and added for table : {}", table_name);
                } else {
                    info!(
                        "Relation for table : {} didn't match the restrictions",
                        table_name
                    );
                }
            }
            Err(e) => error!("Error while processing alter table : {}", e),
        };
    }

    info!("The data has been processed into the data file with sucess");
    // Returns the content generated
    dot_file.to_string()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_re_table_name() {
        assert!(
            RE_TABLE_NAME.is_match("\nCREATE TABLE HELLO();"),
            "with leading"
        );
        assert!(
            RE_TABLE_NAME.is_match("\n\tCREATE TABLE HELLO();"),
            "with leading"
        );
        assert!(
            RE_TABLE_NAME.is_match("\nCREATE TABLE `HELLO`();"),
            "with backquotes"
        );
        assert!(
            RE_TABLE_NAME.is_match("\n\tCReaTe TabLe HELLO();"),
            "non capital letters"
        );
        assert!(
            RE_TABLE_NAME.is_match("CREATE TABLE   \t HELLO();"),
            "several spaces between"
        );
        assert!(
            RE_TABLE_NAME.is_match("\tCREATE\t\t TABLE   \t HELLO();"),
            "several spaces between"
        );
        assert!(
            RE_TABLE_NAME.is_match("CREATE \n\tTABLE \n \t HELLO();"),
            "several backline between"
        );
        assert!(
            RE_TABLE_NAME.is_match("CREATE \n\tTABLE \n \t HELLO();"),
            "several backline between"
        );
        assert!(
            RE_TABLE_NAME.is_match("CREATE TABLE IF NOT EXISTS HELLO();"),
            "if not exists"
        );

        assert!(!RE_TABLE_NAME.is_match("CREATE TABL HELLO();"), "typo");
        assert!(!RE_TABLE_NAME.is_match("CRATE TABLE HELLO();"), "typo");
        assert!(
            !RE_TABLE_NAME.is_match("CREATE OR TABLE HELLO();"),
            "wrong keyword"
        );
        assert!(
            !RE_TABLE_NAME.is_match("CREATE DATABASE HELLO();"),
            "wrong keyword"
        );
        assert!(
            !RE_TABLE_NAME.is_match("DROP TABLE HELLO();"),
            "wrong keyword"
        );
        assert!(
            !RE_TABLE_NAME.is_match("ALTER TABLE HELLO();"),
            "wrong keyword"
        );

        assert_eq!(
            RE_TABLE_NAME
                .captures("CREATE TABLE HELLO();")
                .unwrap()
                .name("table_name")
                .unwrap()
                .as_str(),
            "HELLO",
            "normal"
        );
        assert_eq!(
            RE_TABLE_NAME
                .captures("CREATE TABLE `HELLO`();")
                .unwrap()
                .name("table_name")
                .unwrap()
                .as_str(),
            "`HELLO`",
            "with backquotes"
        );
        assert_eq!(
            RE_TABLE_NAME
                .captures("CREATE TABLE IF NOT EXISTS `HELLO`();")
                .unwrap()
                .name("table_name")
                .unwrap()
                .as_str(),
            "`HELLO`",
            "with backquotes"
        );
        assert_eq!(
            RE_TABLE_NAME
                .captures("CREATE TABLE If NoT EXIsTS HELLO();")
                .unwrap()
                .name("table_name")
                .unwrap()
                .as_str(),
            "HELLO",
            "with backquotes and mixed"
        );
        assert_eq!(
            RE_TABLE_NAME
                .captures("\t\nCREATE\t\n TABLE\t\n `HELLO`\t();")
                .unwrap()
                .name("table_name")
                .unwrap()
                .as_str(),
            "`HELLO`",
            "with separative sequences"
        );
        assert_eq!(
            RE_TABLE_NAME
                .captures("\t\nCreATE\t\n TaBle\t\n `HeLlO`();")
                .unwrap()
                .name("table_name")
                .unwrap()
                .as_str(),
            "`HeLlO`",
            "mixed"
        );
    }

    #[test]
    fn test_re_fk_def() {
        let captures: Captures = RE_FK_DEF
            .captures("FOREIGN KEY (PersonID) REFERENCES Persons(PersonID)")
            .unwrap();
        assert_eq!(
            captures.name("table_key").unwrap().as_str(),
            "PersonID",
            "single"
        );
        assert_eq!(
            captures.name("distant_table").unwrap().as_str(),
            "Persons",
            "single"
        );
        assert_eq!(
            captures.name("distant_key").unwrap().as_str(),
            "PersonID",
            "single"
        );

        let captures_with_bq: Captures = RE_FK_DEF
            .captures("FOREIGN KEY (`PersonID`) REFERENCES `Persons`(`PersonID`)")
            .unwrap();
        assert_eq!(
            captures_with_bq.name("table_key").unwrap().as_str(),
            "`PersonID`",
            "single with bq"
        );
        assert_eq!(
            captures_with_bq.name("distant_table").unwrap().as_str(),
            "Persons",
            "single with bq"
        );
        assert_eq!(
            captures_with_bq.name("distant_key").unwrap().as_str(),
            "`PersonID`",
            "single with bq"
        );

        let captures_several: Captures = RE_FK_DEF
            .captures("FOREIGN KEY (keyA, keyB) REFERENCES Persons(keyC, keyD)")
            .unwrap();
        assert_eq!(
            captures_several.name("table_key").unwrap().as_str(),
            "keyA, keyB",
            "several"
        );
        assert_eq!(
            captures_several.name("distant_table").unwrap().as_str(),
            "Persons",
            "several"
        );
        assert_eq!(
            captures_several.name("distant_key").unwrap().as_str(),
            "keyC, keyD",
            "several"
        );

        let captures_several_with_bq: Captures = RE_FK_DEF
            .captures("FOREIGN KEY (`keyA`, `keyB`) REFERENCES `Persons`(`keyC`, `keyD`)")
            .unwrap();
        assert_eq!(
            captures_several_with_bq.name("table_key").unwrap().as_str(),
            "`keyA`, `keyB`",
            "several with bq"
        );
        assert_eq!(
            captures_several_with_bq
                .name("distant_table")
                .unwrap()
                .as_str(),
            "Persons",
            "several with bq"
        );
        assert_eq!(
            captures_several_with_bq
                .name("distant_key")
                .unwrap()
                .as_str(),
            "`keyC`, `keyD`",
            "several with bq"
        );

        let captures_with_on_delete_set_null: Captures = RE_FK_DEF
            .captures(
                "FOREIGN KEY (`PersonID`) REFERENCES `Persons`(`PersonID`) ON DELETE SET NULL",
            )
            .unwrap();
        assert_eq!(
            captures_with_on_delete_set_null
                .name("on_delete")
                .unwrap()
                .as_str(),
            "SET NULL",
            "normal"
        );

        let captures_with_on_delete_cascade: Captures = RE_FK_DEF
            .captures("FOREIGN KEY (`PersonID`) REFERENCES `Persons`(`PersonID`) ON DELETE CASCADE")
            .unwrap();
        assert_eq!(
            captures_with_on_delete_cascade
                .name("on_delete")
                .unwrap()
                .as_str(),
            "CASCADE",
            "normal"
        );

        let captures_with_on_delete_restrict: Captures = RE_FK_DEF
            .captures(
                "FOREIGN KEY (`PersonID`) REFERENCES `Persons`(`PersonID`) ON DELETE RESTRICT",
            )
            .unwrap();
        assert_eq!(
            captures_with_on_delete_restrict
                .name("on_delete")
                .unwrap()
                .as_str(),
            "RESTRICT",
            "normal"
        );

        let captures_with_on_delete_restrict_and_leading_on_update : Captures = RE_FK_DEF.captures("FOREIGN KEY (`PersonID`) REFERENCES `Persons`(`PersonID`) ON UPDATE SET NULL ON DELETE RESTRICT").unwrap();
        assert_eq!(
            captures_with_on_delete_restrict_and_leading_on_update
                .name("on_delete")
                .unwrap()
                .as_str(),
            "RESTRICT",
            "normal"
        );

        let captures_with_on_delete_restrict_and_trailing_on_update : Captures = RE_FK_DEF.captures("FOREIGN KEY (`PersonID`) REFERENCES `Persons`(`PersonID`) ON DELETE RESTRICT ON UPDATE CASCADE").unwrap();
        assert_eq!(
            captures_with_on_delete_restrict_and_trailing_on_update
                .name("on_delete")
                .unwrap()
                .as_str(),
            "RESTRICT",
            "normal"
        );
    }

    #[test]
    fn test_pk_def() {
        assert!(RE_PK_DEF.is_match("PRIMARY KeY (A) UNIQUE INDEX"));
        assert!(RE_PK_DEF.is_match("PRIMARY KEY (MyPK, secondPK)"));
        assert!(RE_PK_DEF.is_match(
            "PRIMARY KEY
         (FOO) UNIQUE INDEX"
        ));
        assert!(RE_PK_DEF.is_match("PRIMARY KeY (FOO, BAR) UNIQUE INDEX"));
        assert!(RE_PK_DEF.is_match("PRIMARY KeY (FOO, BAR,  FOOBAR) UNIQUE INDEX"));

        assert!(!RE_PK_DEF.is_match("FOREIGN KEY (keyA, keyB) REFERENCES Persons(keyC, keyD)"));
        assert!(!RE_PK_DEF.is_match("FOO KeY VARCHAR UNIQUE"));
        assert!(!RE_PK_DEF.is_match("KEY (A) UNIQUE"));

        assert_eq!(
            RE_PK_DEF
                .captures("PRIMARY KeY (A) UNIQUE INDEX")
                .unwrap()
                .name("col_name")
                .unwrap()
                .as_str(),
            "A",
            "unique key"
        );
        assert_eq!(
            RE_PK_DEF
                .captures("PRIMARY KEY (MyPK, secondPK)")
                .unwrap()
                .name("col_name")
                .unwrap()
                .as_str(),
            "MyPK, secondPK",
            "several keys"
        );
        assert_eq!(
            RE_PK_DEF
                .captures(
                    "PRIMARY KEY
         (FOO) UNIQUE INDEX"
                )
                .unwrap()
                .name("col_name")
                .unwrap()
                .as_str(),
            "FOO",
            "unique key"
        );
        assert_eq!(
            RE_PK_DEF
                .captures("PRIMARY KeY (FOO, BAR) UNIQUE INDEX")
                .unwrap()
                .name("col_name")
                .unwrap()
                .as_str(),
            "FOO, BAR",
            "several keys"
        );
        assert_eq!(
            RE_PK_DEF
                .captures("PRIMARY KeY (`FOO, BAR,  FOOBAR`) UNIQUE INDEX")
                .unwrap()
                .name("col_name")
                .unwrap()
                .as_str(),
            "`FOO, BAR,  FOOBAR`",
            "several keys behind bq"
        );
    }

    #[test]
    fn test_re_alter_table() {
        assert!(
            RE_ALTERED_TABLE
                .is_match("ALTER TABLE HELLO ADD FOREIGN KEY (PersonID) REFERENCES artists (id) ;"),
            "normal"
        );
        let captures = RE_ALTERED_TABLE
            .captures("ALTER \t\nTABLE HELLO ADD FOREIGN KEY (PersonID) REFERENCES artists (id) ;")
            .unwrap();
        assert_eq!(
            captures.name("table_name").unwrap().as_str(),
            "HELLO",
            "normal"
        );
        assert_eq!(
            captures.name("altered_content").unwrap().as_str(),
            "ADD FOREIGN KEY (PersonID) REFERENCES artists (id) ",
            "normal"
        );

        assert!(
            RE_ALTERED_TABLE.is_match(
                "ALTER TABLE `HELLO` ADD FOREIGN KEY (`PersonID`) REFERENCES `artists` (`id`) ;"
            ),
            "normal"
        );
        let captures2 = RE_ALTERED_TABLE
            .captures(
                "ALTER TABLE `HELLO` ADD FOREIGN KEY (`PersonID`) REFERENCES `artists` (`id`) ;",
            )
            .unwrap();
        assert_eq!(
            captures2.name("table_name").unwrap().as_str(),
            "HELLO",
            "normal"
        );
        assert_eq!(
            captures2.name("altered_content").unwrap().as_str(),
            "ADD FOREIGN KEY (`PersonID`) REFERENCES `artists` (`id`) ",
            "normal"
        );
    }

    #[test]
    fn test_re_col_def() {
        assert!(RE_COL_DEF.is_match("foo INT(10) UNIQUE"), "normal key def");
        assert!(
            RE_COL_DEF.is_match("foo VARCHAR(10) UNIQUE"),
            "normal key def"
        );
        assert!(RE_COL_DEF.is_match("foo TEXT INDEX"), "normal key def");
        assert!(
            RE_COL_DEF.is_match("foo INT(10) UNIQUE"),
            "normal key def with pk"
        );
        assert!(
            RE_COL_DEF.is_match("foo VARCHAR(10) UNIQUE"),
            "normal key def with pk"
        );
        assert!(
            RE_COL_DEF.is_match("foo TEXT INDEX"),
            "normal key def with pk"
        );
        assert!(RE_COL_DEF.is_match("foo INT(10) UNIQUE"), "normal key def");

        assert_eq!(
            RE_COL_DEF
                .captures("foo VARCHAR(10) UNIQUE")
                .unwrap()
                .name("col_name")
                .unwrap()
                .as_str(),
            "foo",
            "normal key def"
        );
        assert_eq!(
            RE_COL_DEF
                .captures("foo VARCHAR(10) UNIQUE")
                .unwrap()
                .name("col_def")
                .unwrap()
                .as_str(),
            "VARCHAR(10) UNIQUE",
            "normal key def"
        );
        assert_eq!(
            RE_COL_DEF
                .captures("foo TEXT INDEX")
                .unwrap()
                .name("col_name")
                .unwrap()
                .as_str(),
            "foo",
            "normal key def"
        );
        assert_eq!(
            RE_COL_DEF
                .captures("foo TEXT INDEX")
                .unwrap()
                .name("col_def")
                .unwrap()
                .as_str(),
            "TEXT INDEX",
            "normal key def"
        );
        assert_eq!(
            RE_COL_DEF
                .captures("foo INT(10) UNIQUE")
                .unwrap()
                .name("col_name")
                .unwrap()
                .as_str(),
            "foo",
            "normal key def with pk"
        );
        assert_eq!(
            RE_COL_DEF
                .captures("foo INT(10) UNIQUE")
                .unwrap()
                .name("col_def")
                .unwrap()
                .as_str(),
            "INT(10) UNIQUE",
            "normal key def with pk"
        );
        assert_eq!(
            RE_COL_DEF
                .captures("foo VARCHAR(10) UNIQUE")
                .unwrap()
                .name("col_name")
                .unwrap()
                .as_str(),
            "foo",
            "normal key def with pk"
        );
        assert_eq!(
            RE_COL_DEF
                .captures("foo VARCHAR(10) UNIQUE")
                .unwrap()
                .name("col_def")
                .unwrap()
                .as_str(),
            "VARCHAR(10) UNIQUE",
            "normal key def with pk"
        );
        assert_eq!(
            RE_COL_DEF
                .captures("`foo` TEXT INDEX")
                .unwrap()
                .name("col_name")
                .unwrap()
                .as_str(),
            "`foo`",
            "normal key def with pk"
        );
        assert_eq!(
            RE_COL_DEF
                .captures("`foo`   TEXT INDEX")
                .unwrap()
                .name("col_def")
                .unwrap()
                .as_str(),
            "TEXT INDEX",
            "normal key def with pk"
        );
    }

    #[test]
    fn test_re_col_type() {
        assert!(
            RE_COL_TYPE.is_match(" FOREIGN KEY (PersonID) REFERENCES artists (id) "),
            "fk def"
        );
        assert!(
            RE_COL_TYPE.is_match(" FOREIGN KEY (PersonID) REFERENCES artists (id) "),
            "fk def"
        );
        assert!(
            RE_COL_TYPE.is_match(" FOREIGN KEY (`PersonID`) REFERENCES `artists` (`id`) "),
            "fk def with backcomas"
        );
        assert!(
            RE_COL_TYPE.is_match(" CONSTRAINT FOREIGN KEY (PersonID) REFERENCES artists (id) "),
            "fk def with constraint"
        );
        assert!(
            RE_COL_TYPE
                .is_match(" CONSTRAINT FOREIGN KEY (`PersonID`) REFERENCES `artists` (`id`) "),
            "fk def with constraint and backcomas"
        );
        assert!(RE_COL_TYPE.is_match(" \nPRIMARY KEY (PersonID)"), "primary");
        assert!(
            RE_COL_TYPE.is_match(" \nPRIMARY KEY (`PersonID`)"),
            "primary with backquotes"
        );
        assert!(
            RE_COL_TYPE.is_match(" \nCONSTRAINT  PRIMARY KEY (PersonID)"),
            "constraint with primary"
        );
        assert!(
            RE_COL_TYPE.is_match(" \nCONSTRAINT PRIMARY KEY (`PersonID`)"),
            "constraint with primary and backquotes"
        );
        assert!(RE_COL_TYPE.is_match(" \nUNIQUE KEY (PersonID)"), "unique");
        assert!(
            RE_COL_TYPE.is_match(" \nUNIQUE KEY (`PersonID`)"),
            "unique with backquotes"
        );
        assert!(
            RE_COL_TYPE.is_match(" \nCONSTRAINT  UNIQUE KEY (PersonID)"),
            "unique with constraint"
        );
        assert!(
            RE_COL_TYPE.is_match(" \nCONSTRAINT UNIQUE KEY (`PersonID`)"),
            "unique with constraint and backquotes"
        );
        assert!(
            RE_COL_TYPE.is_match(" \nKEY `productLine` (`productLine`),"),
            "key def"
        );
        assert!(
            RE_COL_TYPE.is_match(" \nKEY `productLine` (productLine),"),
            "key def with back quotes"
        );
        assert!(
            RE_COL_TYPE.is_match(" \nINDEX `productLine` (`productLine`),"),
            "index with backquote"
        );
        assert!(
            RE_COL_TYPE.is_match(" \nINDEX `productLine` (productLine),"),
            "index with mixed backquotes"
        );

        assert_eq!(
            RE_COL_TYPE
                .captures(" FOREIGN KEY (PersonID) REFERENCES artists (id) ")
                .unwrap()
                .name("key_type")
                .unwrap()
                .as_str(),
            "FOREIGN",
            "fk def"
        );
        assert_eq!(
            RE_COL_TYPE
                .captures(" FOREIGN KEY (PersonID) REFERENCES artists (id) ")
                .unwrap()
                .name("key_type")
                .unwrap()
                .as_str(),
            "FOREIGN",
            "fk def"
        );
        assert_eq!(
            RE_COL_TYPE
                .captures(" FOREIGN KEY (`PersonID`) REFERENCES `artists` (`id`) ")
                .unwrap()
                .name("key_type")
                .unwrap()
                .as_str(),
            "FOREIGN",
            "fk def with backcomas"
        );
        assert_eq!(
            RE_COL_TYPE
                .captures(" CONSTRAINT FOREIGN KEY (PersonID) REFERENCES artists (id) ")
                .unwrap()
                .name("key_type")
                .unwrap()
                .as_str(),
            "FOREIGN",
            "fk def with constraint"
        );
        assert_eq!(
            RE_COL_TYPE
                .captures(" CONSTRAINT FOREIGN KEY (`PersonID`) REFERENCES `artists` (`id`) ")
                .unwrap()
                .name("key_type")
                .unwrap()
                .as_str(),
            "FOREIGN",
            "fk def with constraint and backcomas"
        );
        assert_eq!(
            RE_COL_TYPE
                .captures(" \nPRIMARY KEY (PersonID)")
                .unwrap()
                .name("key_type")
                .unwrap()
                .as_str(),
            "PRIMARY",
            "primary"
        );
        assert_eq!(
            RE_COL_TYPE
                .captures(" \nPRIMARY KEY (`PersonID`)")
                .unwrap()
                .name("key_type")
                .unwrap()
                .as_str(),
            "PRIMARY",
            "primary with backquotes"
        );
        assert_eq!(
            RE_COL_TYPE
                .captures(" \nCONSTRAINT  PRIMARY KEY (PersonID)")
                .unwrap()
                .name("key_type")
                .unwrap()
                .as_str(),
            "PRIMARY",
            "constraint with primary"
        );
        assert_eq!(
            RE_COL_TYPE
                .captures(" \nCONSTRAINT PRIMARY KEY (`PersonID`)")
                .unwrap()
                .name("key_type")
                .unwrap()
                .as_str(),
            "PRIMARY",
            "constraint with primary and backquotes"
        );

        assert!(
            !RE_COL_TYPE.is_match("`productCode` varchar(15) NOT NULL,"),
            "col def"
        );
    }

    #[test]
    fn test_re_comments() {
        // Test single-line comments with --
        assert!(
            RE_COMMENTS.is_match("-- This is a single-line comment"),
            "single-line comment with --"
        );
        assert!(
            RE_COMMENTS.is_match("SELECT * FROM users; -- This is another comment"),
            "single-line comment with -- at the end of a line"
        );

        // Test single-line comments with #
        assert!(
            RE_COMMENTS.is_match("# This is a comment with hash"),
            "single-line comment with #"
        );
        assert!(
            RE_COMMENTS.is_match("SELECT * FROM products; # End of query"),
            "single-line comment with # at the end of a line"
        );

        // Test multi-line comments with /* */
        assert!(
            RE_COMMENTS.is_match("/* This is a multi-line comment */"),
            "multi-line comment"
        );
        assert!(
            RE_COMMENTS.is_match("SELECT * FROM orders; /* Comment within SQL */"),
            "multi-line comment inside SQL query"
        );
        assert!(
            RE_COMMENTS.is_match("/* This is a\nmulti-line comment */"),
            "multi-line comment spanning multiple lines"
        );

        // Test comments with special characters or mixed spacing
        assert!(
            RE_COMMENTS.is_match("  -- Comment with leading spaces"),
            "single-line comment with leading spaces"
        );
        assert!(
            RE_COMMENTS.is_match("    # Another comment with spaces before hash"),
            "single-line comment with spaces before #"
        );
        assert!(
            RE_COMMENTS.is_match("  /* A multi-line\n  comment with indentation */"),
            "multi-line comment with indentation"
        );

        // Ensure non-matching text does not match
        assert!(
            !RE_COMMENTS.is_match("SELECT * FROM users;"),
            "text without comments"
        );
        assert!(
            !RE_COMMENTS.is_match("CREATE TABLE products (id INT, name VARCHAR);"),
            "SQL definition without comments"
        );

        // Check the capturing groups to ensure proper comment capture
        assert_eq!(
            RE_COMMENTS
                .captures("-- This is a comment")
                .unwrap()
                .get(0)
                .unwrap()
                .as_str(),
            "-- This is a comment",
            "capture single-line comment with --"
        );
        assert_eq!(
            RE_COMMENTS
                .captures("# This is a hash comment")
                .unwrap()
                .get(0)
                .unwrap()
                .as_str(),
            "# This is a hash comment",
            "capture single-line comment with #"
        );
        assert_eq!(
            RE_COMMENTS
                .captures("/* This is a multi-line comment */")
                .unwrap()
                .get(0)
                .unwrap()
                .as_str(),
            "/* This is a multi-line comment */",
            "capture multi-line comment"
        );
        assert_eq!(
            RE_COMMENTS
                .captures("SELECT * FROM orders; /* Comment at the end */")
                .unwrap()
                .get(0)
                .unwrap()
                .as_str(),
            "/* Comment at the end */",
            "capture multi-line comment at the end of a SQL statement"
        );

        // Test edge cases like empty comments or comments with only spaces
        assert!(RE_COMMENTS.is_match("-- "), "empty single-line comment");
        assert!(
            RE_COMMENTS.is_match("  #     "),
            "empty single-line comment with leading spaces"
        );
        assert!(RE_COMMENTS.is_match("/*  */"), "empty multi-line comment");
        assert!(
            RE_COMMENTS.is_match("/*\n   \n*/"),
            "empty multi-line comment with newline characters"
        );

        // Ensure the comment is correctly captured even with different line breaks
        assert_eq!(
            RE_COMMENTS
                .captures("/*\nThis is a multi-line comment\n*/")
                .unwrap()
                .get(0)
                .unwrap()
                .as_str(),
            "/*\nThis is a multi-line comment\n*/",
            "capture multi-line comment with line breaks"
        );
    }
}
