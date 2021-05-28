use std::fmt;

use super::dot_table::{DotTable};

/// A DotFile object is used to render the compiled schema in argument.
pub struct DotFile {
    /// The header of the dot file
    header : String,
    /// The tables to include in the file
    dot_tables: Vec<DotTable>,
    /// The relations to include in the file
    relations: Vec<String>,
    /// The footer of the file
    footer : String,
    /// Define if the graph has to be in dark mode
    dark_mode: bool
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

    /// Creates a new dotfile with the given name
    ///
    /// # Arguments
    ///
    /// * `filename` - This will be set as the graph's filename
    /// * `dark_mode` - Set if the file has to be in dark mode
    pub fn new(filename : &str, dark_mode : bool) -> DotFile {

        DotFile {
            header : init_dot(filename, dark_mode),
            dot_tables: Vec::new(),
            relations: Vec::new(),
            footer: String::from("}"),
            dark_mode
        }
    }

    /// Adds a table to the DotFile
    ///
    /// # Arguments
    ///
    /// * `table` - The table to add to the file
    pub fn add_table(&mut self, table : DotTable) {
        self.dot_tables.push(table);
    }

    /// Add a relation to the DotFile
    ///
    /// # Arguments
    ///
    /// * `table_name` - The table where the relation begins
    /// * `table_end` - The table where the relation ends
    /// * `key` - The key of the begin table
    /// * `refered` - The key of the end table
    pub fn add_relation(&mut self, table_name: &str, table_end: &str, key: &str, refered: &str){
        self.relations.push(generate_relation(table_name, table_end, key, refered, self.dark_mode))
    }
}

/// Creates the dot file header
///
/// # Arguments
///
/// * `filename` - The name of the input file
/// * `dark_mode` - Changes the color of rendering
fn init_dot(filename: &str, dark_mode: bool) -> String {
    let bg_color : &str = match dark_mode {
            true => "bgcolor= black;",
            false => "",
    };
    format!("//This file has been generated with sqltodot, enjoy!
digraph {0} {{\n

    {1}

    node [\n
        shape = \"plaintext\"
    ]\n\n", filename, bg_color)
}

/// Generate a dot relation with the given arguments
///
/// # Arguments
///
/// * `table_name` - The table where the relation begins
/// * `table_end` - The table where the relation ends
/// * `key` - The key of the begin table
/// * `refered` - The key of the end table
fn generate_relation(table_name: &str, table_end: &str, key: &str, refered: &str, dark_mode: bool) -> String {
    let color_scheme : &str = match dark_mode {
        true => "fontcolor=white, color=white",
        false => ""
    };
    format!("\t{0} -> {1} [label=<<I>{2} \u{27A1} {3}</I>>, arrowhead = \"dot\", fontsize=\"12.0\", {4}]", table_name, table_end, key, refered, color_scheme)
}
