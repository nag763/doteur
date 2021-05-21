use std::fmt;
use std::ffi::OsStr;
use std::path::Path;

use super::dot_table::{DotTable};


pub struct DotFile {
    header : String,
    dot_tables: Vec<DotTable>,
    relations: Vec<String>,
    footer : String
}

impl fmt::Display for DotFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{0}\n{1}\n{2}\n\n{3}",
            self.header,
            self.dot_tables.iter().map(|s| s.to_string()).collect::<Vec<String>>().join("\n"),
            self.relations.join("\n"),
            self.footer
        )
    }
}

impl DotFile {

    pub fn new(filename : &str) -> DotFile {

        DotFile {
            header : init_dot(filename),
            dot_tables: Vec::new(),
            relations: Vec::new(),
            footer: String::from("}")
        }
    }

        pub fn add_table(&mut self, table : DotTable) {
            self.dot_tables.push(table);
        }

        pub fn add_relation(&mut self, table_name: &str, table_end: &str, key: &str, refered: &str){
            self.relations.push(generate_relation(table_name, table_end, key, refered))
        }
}

///Create dot file header.
fn init_dot(filename: &str) -> String {
    format!("//This file has been generated with sqltodot, enjoy!
digraph {} {{\n
    node [\n
        shape = \"plaintext\"
    ]\n\n", Path::new(filename).file_stem().unwrap_or_else(|| OsStr::new("sql")).to_str().unwrap_or("sql"))
}

fn generate_relation(table_name: &str, table_end: &str, key: &str, refered: &str) -> String {
    format!("\t\t{0} -> {1} [label=\"Key {2} refers {3}\", arrowhead = \"dot\"]", table_name, table_end, key, refered)
}
