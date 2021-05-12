pub mod add_trait;

use std::path::Path;
use std::ffi::OsStr;
use regex::Regex;
use std::fs;

use add_trait::{Trim};

#[macro_use] extern crate lazy_static;


lazy_static! {
    ///Look after table defs.
    static ref RE_TABLE_DEFS : Regex = Regex::new(r"(?i)\s*CREATE\sTABLE[^;]*.").unwrap();
    ///Get table name.
    static ref RE_TABLE_NAME : Regex = Regex::new(r"((?i)\s?CREATE\sTABLE\s*[`]?)+(\w*).").unwrap();
    ///Check if foreign key exists.
    static ref RE_FK : Regex = Regex::new(r"(?i)\sFOREIGN\sKEY").unwrap();
    ///Check for the content in parenthesis.
    static ref RE_IN_PARENTHESES : Regex = Regex::new(r"([^`][a-zA-Z]*\s*)(\(([^()]+)\))").unwrap();
    ///Split on coma.
    static ref RE_SEP_COMA : Regex = Regex::new(r",\s").unwrap();
    ///Look after alter table statements.
    static ref RE_ALTERED_TABLE : Regex = Regex::new(r"(\s(?i)ALTER TABLE\s*)(`?(\w*)`?)([^;]*)").unwrap();
}

///Get the tables from the input.
fn get_tables(input: &str) -> Vec<&str> {
    RE_TABLE_DEFS.find_iter(input)
            .map(|element| element.as_str())
            .collect::<Vec<&str>>()
}

///Check if the given input has declared tables.
pub fn contains_tables(input: &str) -> bool {
    get_tables(input).len() != 0
}


///Convert sql table to dot output.
fn convert_sql_to_dot(input: &str) -> (String, String) {
    let table_name = RE_TABLE_NAME.captures(input)
                                  .unwrap()
                                  .get(2)
                                  .map_or("TABLE NAME".to_string(), |t| t.as_str().trim_leading_trailing());
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
        .map(|s| (generate_attributes(s), generate_relations(&table_name, s)))
        .collect::<Vec<(String, Option<String>)>>();

    let body_content : String = generated
        .iter()
        .map(|s| s.0.as_str())
        .filter(|s| s.len() != 0)
        .collect::<Vec<&str>>()
        .join("\n\n");

    let relations : String = generated
        .iter()
        .map(|s| s.1.as_deref().unwrap_or_default())
        .filter(|s| s.len() != 0)
        .collect::<Vec<&str>>()
        .join("\n");

    (close_table([table_header, body_content].join("\n").as_str()), relations)
}


///Create dot file header.
fn init_dot(filename: &str) -> String {
    format!("digraph {} {{\n
    node [\n
        shape = \"plaintext\"\n
    ]\n\n", Path::new(filename).file_stem().unwrap_or(OsStr::new("sql")).to_str().unwrap_or("sql"))
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
            title = splitted.remove(0).to_string();
            rest = splitted.join(" ").into();
        }
        format!("
        <TR><TD ALIGN=\"LEFT\" BORDER=\"0\">
        <FONT FACE=\"Roboto\"><B>{0}</B></FONT>
        </TD><TD ALIGN=\"LEFT\">
        <FONT FACE=\"Roboto\">{1}</FONT>
        </TD></TR>", title.trim_leading_trailing(), rest.trim_leading_trailing()
        )
    } else {
        let is_fk : bool = RE_FK.find_iter(attr).map(|s| s.as_str()).collect::<Vec<&str>>().len() != 0;
        // If the key is a foreign key, write it.
        if is_fk {
            let matches : Vec<&str> = RE_IN_PARENTHESES
                .find_iter(attr)
                .map(
                    |s| s.as_str()
                ).collect::<Vec<&str>>();
            let title : String = matches[0].chars()
                                           .take(matches[0].len()-1)
                                           .skip(matches[0].find('(').unwrap()+1)
                                           .collect::<String>()
                                           .as_str()
                                           .trim_leading_trailing();
            // The tabs here are for the output file.
            format!("
        <TR><TD ALIGN=\"LEFT\" BORDER=\"0\">
        <FONT FACE=\"Roboto\"><B>[FK] {0}</B></FONT>
        </TD><TD ALIGN=\"LEFT\">
        <FONT FACE=\"Roboto\">Refers to {1}</FONT>
        </TD></TR>", title.replace("`", ""), matches[1].trim_leading_trailing().replace("`", "")
            )
        //If not, write an empty string.
        } else {
            "".to_string()
        }
    }
}


///Generate relations from the given inputs.
fn generate_relations(table_name : &str, input: &str) -> Option<String> {
    let is_fk : bool = RE_FK.find_iter(input).map(|s| s.as_str()).collect::<Vec<&str>>().len() != 0;
    // No PK support yet.
    if is_fk {
        let replaced : &str = &input.replace("`", "");
        let matches : Vec<&str> = RE_IN_PARENTHESES.find_iter(replaced).map(|s| s.as_str()).collect();
        if matches.len() != 0 {
            let table_end : &str = matches[1].split("(").collect::<Vec<&str>>()[0];
            Some(format!("\t{} -> {} [label=\"{} refers {}\"]", table_name, table_end, matches[0].trim_leading_trailing(), matches[1].trim_leading_trailing()))
        } else {
            None
        }
    } else {
        None
    }
}


///Process the given filename and content to generate a
///.dot file.
pub fn process_file(filename: &str, content: &str) -> String {
    // Generate content from the declared tables.
    let generated_content : Vec<(String, String)> = get_tables(content).iter()
                                                                       .map(|element| convert_sql_to_dot(element))
                                                                       .collect::<Vec<(String, String)>>();

    // Look after the other fks, declared on alter table statements.
    let other_fks : Vec<&str> = RE_ALTERED_TABLE.find_iter(content)
                                                .map(|element| element.as_str())
                                                .collect();

    // Generate the relations from the altered statements.
    let other_relations : Vec<String> = other_fks.iter()
                                                 .map(|element|
                                                    {
                                                        let captures = RE_ALTERED_TABLE.captures(element)
                                                        .unwrap();
                                                        // The fourth element is the table content.
                                                        let lines : Vec<String> = RE_SEP_COMA.split(captures.get(4)
                                                                                             .map(|s| s.as_str())
                                                                                             .unwrap())
                                                                                             .map(|s| s.to_string())
                                                                                             .collect();

                                                        let altered_table_name : &str = captures.get(3)
                                                                                                .map(|s| s.as_str())
                                                                                                .unwrap();
                                                        // Returns the new relation if they aren't empty.
                                                        return lines.iter()
                                                                    .map(|s| generate_relations(altered_table_name, s).unwrap_or_default())
                                                                    .filter(|s| s.len() != 0)
                                                                    .collect::<Vec<String>>()
                                                                    .join("\n");
                                                    }
    ).collect::<Vec<String>>();

    let other_relations_as_str : Vec<&str> = other_relations.iter()
                                                            .map(|s| s.as_str())
                                                            .collect::<Vec<&str>>();

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
                             .collect::<Vec<&str>>()
                             .into_iter()
                             .chain(other_relations_as_str.into_iter())
                             .collect::<Vec<&str>>()
                             .join("\n"),
        ].concat().as_str()
    )
}
