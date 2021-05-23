use std::fmt;

use super::super::add_traits::{Trim};

/// A dot table is the corresponding rendering of a sql table in a dot file
pub struct DotTable {
    /// The header of the table
    header: String,
    /// The attribute of the table
    attributes: Vec<String>,
    /// The footer of the table
    footer: String
}

impl fmt::Display for DotTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{0}\n{1}\n\n\t\t{2}\n", self.header, self.attributes.join("\n"), self.footer)
    }
}

impl DotTable {

    /// Creates a new table
    ///
    /// # Arguments
    ///
    /// * `table_name` - The table to render in dot
    pub fn new(table_name: &str) -> DotTable {
        let header : String = generate_table_header(table_name);
        DotTable {
            header,
            attributes: Vec::new(),
            footer: String::from("</TABLE> >]")
        }
    }

    /// Adds an attribute to the table
    ///
    /// # Arguments
    ///
    /// * `title` - The title of the attribute
    /// * `desc` - The description of the attribute
    pub fn add_attribute(&mut self, title: &str, desc : &str) {
        self.attributes.push(generate_attribute(title, desc));
    }


    /// Adds a foreign key attribute
    ///
    /// # Arguments
    ///
    /// * `key` - The key of the attribute in the table
    /// * `fk_table` - The refered table
    /// * `fk_col` - The refered key
    pub fn add_attribute_fk(&mut self, key: &str, fk_table : &str, fk_col : &str) {
        self.attributes.push(generate_fk_attribute(key, fk_table, fk_col));
    }

}

/// Generate the .dot table header.
///
/// # Arguments
///
/// * `name` - The name of the talbe
fn generate_table_header(name: &str) -> String {
    format!("
    {0} [label=<
        <TABLE BGCOLOR=\"gray92\" BORDER=\"1\" CELLBORDER=\"0\" CELLSPACING=\"0\">

        <TR><TD COLSPAN=\"2\" CELLPADDING=\"5\" ALIGN=\"CENTER\" BGCOLOR=\"indigo\">
        <FONT FACE=\"Roboto\" COLOR=\"white\" POINT-SIZE=\"10\">
        <B>{0}</B>
        </FONT></TD></TR>", name.trim_leading_trailing())
}


/// Generate an attribute
///
/// # Arguments
///
/// * `title` - The name of the attribute
/// * `desc` - The description of the attribute
fn generate_attribute(title: &str, desc : &str) -> String {
    format!("
        <TR><TD ALIGN=\"LEFT\" BORDER=\"0\">
        <FONT FACE=\"Roboto\"><B>{0}</B></FONT>
        </TD><TD ALIGN=\"LEFT\">
        <FONT FACE=\"Roboto\">{1}</FONT>
        </TD></TR>", title.trim_leading_trailing(), desc.trim_leading_trailing()
    )
}

/// Generas a foreign key attribute
///
/// # Arguments
///
/// * `key` - The key of the attribute in the table
/// * `fk_table` - The refered table
/// * `fk_col` - The refered key
fn generate_fk_attribute(key : &str, fk_table : &str, fk_col : &str) -> String {
    format!("
        <TR><TD ALIGN=\"LEFT\" BORDER=\"0\">
        <FONT FACE=\"Roboto\"><B>[FK] {0}</B></FONT>
        </TD><TD ALIGN=\"LEFT\">
        <FONT FACE=\"Roboto\">Refers to <I>{1}[{2}]</I></FONT>
        </TD></TR>", key.trim_leading_trailing(), fk_table.trim_leading_trailing(), fk_col.trim_leading_trailing()
    )
}
