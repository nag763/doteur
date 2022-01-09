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

pub mod args;
pub mod dot_structs;
pub mod tools;

#[cfg(feature = "mysql_addons")]
mod mysql_tools;

#[cfg(feature = "sqlite_addons")]
mod sqlite_tools;

mod add_traits;
mod restriction;

use std::borrow::Cow;

use crate::add_traits::{Replacable, SplitVec, Trim};
use crate::restriction::Restriction;
use crate::tools::detect_comas;

use dot_structs::dot_file::DotFile;
use dot_structs::dot_table::DotTable;
use dot_structs::relation::Relation;

use log::{debug, error, info, warn};
use regex::{Captures, Regex};

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate cfg_if;

macro_rules! unwrap_captures_name_as_str {
    ($captures:ident, $key:expr, $err:block) => {
        match $captures.name($key) {
            Some(v) => v.as_str(),
            None => $err,
        }
    };
    ($captures:ident, $key:expr, $err_label:expr) => {
        unwrap_captures_name_as_str!($captures, $key, {
            return Err($err_label);
        })
    };
    ($captures:ident, $key:expr) => {
        unwrap_captures_name_as_str!($captures, $key, {
            return Err("Named group not found, early return");
        })
    };
}

lazy_static! {
    ///Get table name.
    static ref RE_TABLE_NAME : Regex = Regex::new(r####"(?i)\s*CREATE\s*TABLE\s*(?:IF\s*NOT\s*EXISTS)?\s*[`"'\[]?(?P<table_name>\w*)[`"'\]]?\s*\((?P<content>[^;]*)\)"####).unwrap();
    ///Get column type
    static ref RE_COL_TYPE : Regex = Regex::new(r####"(?i)\s*((?:FULLTEXT|SPATIAL)?\s+(?:INDEX|KEY|CHECK))|(?:CONSTRAINT\s*[`'"]\w*[`'"])?\s*(?P<key_type>UNIQUE|FOREIGN|PRIMARY)\s+"####).unwrap();
    ///Get columns definitioon
    static ref RE_COL_DEF : Regex = Regex::new(r####"(?i)\s*(?P<col_name>(?:[`"\[]{1}[^`"\]]+[`"\]]{1})|(?:\w*))\s*(?P<col_def>.*)"####).unwrap();
    ///Check if input is a primary key
    static ref RE_PK_DEF : Regex = Regex::new(r####"(?i)PRIMARY\s*KEY\s*["`]?(?:\w*)[`"]?\s*\((?P<col_name>[^\)]+)\)"####).unwrap();
    ///Check if a PK is declared in the line
    static ref RE_PK_IN_LINE : Regex = Regex::new(r####"(?i)\s*PRIMARY\s*KEY.*"####).unwrap();
    ///Check for the content in parenthesis.
    static ref RE_FK_DEF : Regex = Regex::new(r####"(?i)FOREIGN\s*KEY\s*\((?P<table_key>[^\)]+)\)\s*REFERENCES\s*[`"'\[]?(?P<distant_table>\w*)["`'\]]?\s*\((?P<distant_key>[^\)]+)\)\s*(?:(?:ON\s*UPDATE\s*(?:(?:SET\s*\w*|\w*))\s*)?(?:ON\s*DELETE\s*)?(?P<on_delete>(SET\s*NULL|CASCADE|RESTRICT|NO\s*ACTION|SET\s*DEFAULT)))?"####).unwrap();
    ///Look after alter table statements.
    static ref RE_ALTERED_TABLE : Regex = Regex::new(r####"\s*(?i)ALTER\s*TABLE\s*['`"\[]?(?P<table_name>\w*)[`"'\]]?\s*(?P<altered_content>[^;]*)"####).unwrap();
    ///Regex to remove comments
    static ref RE_COMMENTS : Regex = Regex::new(r####"(?:[-]{2}|[#]{1}).*$|(?:(?:\\\*)[^\*/]+(?:\*/))"####).unwrap();
}

/// Get the tables from the input
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
/// # Arguments
///
/// * `data` - The content where sql table are stored
pub fn contains_tables(data: &str) -> bool {
    RE_TABLE_NAME.is_match(data)
}

pub fn remove_sql_comments(data: &str) -> Cow<'_, str> {
    RE_COMMENTS.replace(data, "")
}

/// Convert a sql table to a dot table and store it in the given dot file
///
/// # Arguments
///
/// * `dot_file` - A mutable dot file
/// * `input` - The content to convert
/// * `restrictions` - The restriction to apply on the table
/// * `dark_mode` - Changes the rendering of the output file
fn convert_table_to_dot(
    input: &str,
    restrictions: Option<&Restriction>,
    dark_mode: bool,
) -> Result<Option<(DotTable, Vec<Relation>)>, &'static str> {
    let captures: Captures = RE_TABLE_NAME.captures(input).unwrap();

    let table_name: String = unwrap_captures_name_as_str!(
        captures,
        "table_name",
        "Regex error, the input is either not a sql table or isn't parsed properly by the process"
    )
    .trim_leading_trailing();
    debug!("Currently processing table {}", table_name);

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
            return Err("Attributes malformed");
        }
    };

    let mut dot_table: DotTable = DotTable::new(table_name.as_str(), dark_mode);
    let mut relations: Vec<Relation> = Vec::new();

    for line in lines {
        // If column type is common attribute
        if !RE_COL_TYPE.is_match(line) {
            debug!("Line {} is a column def", line.trim_leading_trailing());
            match generate_attributes(&mut dot_table, line) {
                Ok(m) => info!("Attribute processed correctly : {}", m),
                Err(e) => error!("An error happened while processing line : {}", e),
            }
        // If column type is a relation or an index
        } else {
            debug!("Line {} is not a column def", line.trim_leading_trailing());
            let col_type: Captures = match RE_COL_TYPE.captures(line) {
                Some(v) => v,
                None => {
                    error!("Regex error for line - capture not succesfull");
                    continue;
                }
            };

            let key_type: String = unwrap_captures_name_as_str!(col_type, "key_type", {
                warn!("Key type isn't interpreted by the process");
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
                            if let Some(relation) = v {
                                relations.push(relation.clone());
                                for pair_key_refered in relation.get_pairs_of_keys() {
                                    let _ = dot_table.add_fk_nature_to_attribute(
                                        pair_key_refered.0.as_str(),
                                        relation.get_refered_table(),
                                        pair_key_refered.1.as_str(),
                                    );
                                }
                            }
                        }
                        Err(e) => {
                            error!("An error happened while processing foreign key: {}", e)
                        }
                    }
                }
                "PRIMARY" => {
                    if !RE_PK_DEF.is_match(line) {
                        debug!(
                            "Line {} has been found as a primary key def including a column def",
                            line.trim_leading_trailing()
                        );
                        match generate_attributes(&mut dot_table, line) {
                            Ok(m) => info!("PK processed correctly : {}", m),
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
                            Ok(m) => info!("PK processed correctly : {}", m),
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
    info!(
        "The table {} has been added to the file with success",
        table_name
    );
    Ok(Some((dot_table, relations)))
}

/// Generate the attributes and write them into the dot_table
///
/// # Arguments
///
/// * `dot_table` - A mutable DotTable object where the attributes will be written
/// * `attr` - The attributes as string
fn generate_attributes(dot_table: &mut DotTable, attr: &str) -> Result<&'static str, &'static str> {
    // If a PK is present in line, process attribute as pk
    if RE_PK_IN_LINE.is_match(attr) {
        let trimmed_line: &str = &RE_PK_IN_LINE.replace(attr, "");
        let captures: Captures = RE_COL_DEF.captures(trimmed_line).unwrap();
        dot_table.add_attribute_pk(
            unwrap_captures_name_as_str!(captures, "col_name")
                .replace_enclosing()
                .trim_leading_trailing()
                .as_str(),
            unwrap_captures_name_as_str!(captures, "col_def"),
        );
        Ok("PK detected")
    // Otherwise, process as atribute
    } else {
        let captures: Captures = RE_COL_DEF.captures(attr).unwrap();
        dot_table.add_attribute(
            unwrap_captures_name_as_str!(captures, "col_name")
                .replace_enclosing()
                .trim_leading_trailing()
                .as_str(),
            unwrap_captures_name_as_str!(captures, "col_def"),
        );
        Ok("COLUMN DEF detected")
    }
}

/// Generate the attributes as primary and write them into the table
///
/// # Arguments
///
/// * `dot_table` - A mutable DotTable object where the attributes will be written
/// * `line` - The line as string
fn generate_primary(dot_table: &mut DotTable, line: &str) -> Result<&'static str, &'static str> {
    // Assert that the line matches regex and get the captures
    let captures: Captures = match RE_PK_DEF.captures(line) {
        Some(captures) => captures,
        None => return Err("Regex input err"),
    };
    // Check that the group column name has been captured, and detect the comas within
    let (col_name, comas_detected): (&str, Result<Vec<usize>, &str>) =
        match captures.name("col_name") {
            Some(v) => (v.as_str(), detect_comas(v.as_str())),
            None => return Err("Not a PK"),
        };
    match comas_detected {
        //If severeal comas are detected
        Ok(comas_vec) if !comas_vec.is_empty() => {
            if col_name
                .to_string()
                .split_vec(comas_vec)
                .iter()
                .any(|attr| {
                    dot_table
                        .add_pk_nature_to_attribute(
                            attr.replace_enclosing().trim_leading_trailing().as_str(),
                        )
                        .is_err()
                })
            {
                Err("One or more errors for multiple PK attr def")
            } else {
                Ok("Multiple attributes set as PK")
            }
        }
        // If no comas are detected
        _ => {
            if let Err(e) = dot_table.add_pk_nature_to_attribute(
                col_name
                    .replace_enclosing()
                    .trim_leading_trailing()
                    .as_str(),
            ) {
                Err(e)
            } else {
                Ok("PK nature added")
            }
        }
    }
}

/// Generates the relations and write them into the DotFile
///
/// # Arguments
///
/// * `dot_file` - Where the content should be written in
/// * `dot_table` - Table to add the attribute if needed
/// * `table_name` - The name of the table where the relations originates
/// * `input` - Where the relations are written
/// * `restrictive_regex` - The restrictions to apply
fn generate_relations(
    table_name: &str,
    line: &str,
    restrictive_regex: Option<&Restriction>,
) -> Result<Option<Relation>, &'static str> {
    // If the regex doesn't match the input, early return
    if !RE_FK_DEF.is_match(line) {
        return Err("Not a relation");
    }

    let captures: Captures = match RE_FK_DEF.captures(line) {
        Some(v) => v,
        None => return Err("Regex error"),
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
    return match detect_comas(table_key.as_str()) {
        // Case where attributes are separated by comas
        Ok(comas_vec) if !comas_vec.is_empty() => {
            return match detect_comas(distant_key.as_str()) {
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
                _ => Err("Error in file format"),
            };
        }
        // Single key processing
        _ => Ok(Some(Relation::new_with_single_pair(
            table_name.to_string(),
            distant_table.to_string(),
            table_key.replace_enclosing().trim_leading_trailing(),
            distant_key.replace_enclosing().trim_leading_trailing(),
            relation_type.to_string(),
        ))),
    };
}

/// Process the given file and return the output dot string
///
/// # Arguments
///
/// * `args` - The CLI args
pub fn process_data(
    data: &str,
    restrictions: Option<&Restriction>,
    dot_file_graph_name: &str,
    legend: bool,
    dark_mode: bool,
) -> String {
    let mut dot_file: DotFile = DotFile::new(dot_file_graph_name, legend, dark_mode);

    let cleaned_content: &str = &remove_sql_comments(data);

    // Generate content from the declared tables.
    get_tables(cleaned_content).iter().for_each(|element| {
        match convert_table_to_dot(element, restrictions, dark_mode) {
            Ok(result) => {
                if let Some((dot_table, relations)) = result {
                    dot_file.add_table(dot_table);
                    for relation in relations {
                        dot_file.add_relation(relation);
                    }
                    info!("Table added to dot file");
                } else {
                    info!("Table wasn't matching the restrictions");
                }
            }
            Err(e) => error!("An error happened while processing the sql file : {}", e),
        };
    });

    // Look after the other fks, declared on alter table statements.
    for element in RE_ALTERED_TABLE.captures_iter(cleaned_content) {
        match generate_relations(
            unwrap_captures_name_as_str!(element, "table_name", {
                panic!("Regex error");
            }),
            unwrap_captures_name_as_str!(element, "altered_content", {
                panic!("Regex error");
            }),
            restrictions,
        ) {
            Ok(v) => {
                if let Some(relation) = v {
                    dot_file.add_relation(relation);
                }
            }
            Err(e) => error!("Error while processing alter table : {}", e),
        };
    }

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
            "HELLO",
            "with backquotes"
        );
        assert_eq!(
            RE_TABLE_NAME
                .captures("CREATE TABLE IF NOT EXISTS `HELLO`();")
                .unwrap()
                .name("table_name")
                .unwrap()
                .as_str(),
            "HELLO",
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
            "HELLO",
            "with separative sequences"
        );
        assert_eq!(
            RE_TABLE_NAME
                .captures("\t\nCreATE\t\n TaBle\t\n `HeLlO`();")
                .unwrap()
                .name("table_name")
                .unwrap()
                .as_str(),
            "HeLlO",
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
}
