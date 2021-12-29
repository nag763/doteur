use std::fmt;

use super::super::add_traits::Trim;
use super::attribute::{Attribute, KeyValueMap};

/// A dot table is the corresponding rendering of a sql table in a dot file
pub struct DotTable {
    header: String,
    /// The attribute of the table
    attributes: Vec<Attribute>,
    /// The footer of the table
    footer: String,
    /// Changes the rendering of the file if true
    dark_mode: bool,
}

impl fmt::Display for DotTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{0}\n{1}\n\n\t{2}\n",
            self.header,
            self.attributes
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
                .join("\n"),
            self.footer
        )
    }
}

impl DotTable {
    /// Creates a new table
    ///
    /// # Arguments
    ///
    /// * `table_name` - The table to render in dot
    /// * `dark_mode` - Changes the rendering of the file if true
    pub fn new(table_name: &str, dark_mode: bool) -> DotTable {
        let header: String = generate_table_header(table_name, dark_mode);
        DotTable {
            header,
            attributes: Vec::new(),
            footer: String::from("</TABLE> >]"),
            dark_mode,
        }
    }

    /// Adds an attribute to the table
    ///
    /// # Arguments
    ///
    /// * `title` - The title of the attribute
    /// * `desc` - The description of the attribute
    pub fn add_attribute(&mut self, title: &str, desc: &str) {
        self.attributes.push(Attribute::new_col_def(
            title.to_string(),
            desc.to_string(),
            self.dark_mode,
        ));
    }

    /// Adds a PK to the table
    ///
    /// # Arguments
    ///
    /// * `title` - The title of the attribute
    /// * `desc` - The description of the attribute
    pub fn add_attribute_pk(&mut self, key: &str, desc: &str) {
        self.attributes.push_or_replace_attribute(Attribute::new_pk(
            key.to_string(),
            desc.to_string(),
            self.dark_mode,
        ));
    }

    /// Adds foreign key nature to given attribute
    ///
    /// # Arguments
    ///
    /// * `key` - The key of the attribute in the table
    /// * `fk_table` - The refered table
    /// * `fk_col` - The refered key
    pub fn add_fk_nature_to_attribute(
        &mut self,
        key: &str,
        fk_table: &str,
        fk_col: &str,
    ) -> Result<usize, &'static str> {
        self.attributes
            .add_fk_nature_to_attribute(key, fk_table, fk_col)
    }

    /// Adds primary key nature to given attribute
    ///
    /// # Arguments
    ///
    /// * `key` - The key of the attribute in the table
    pub fn add_pk_nature_to_attribute(&mut self, key: &str) -> Result<usize, &'static str> {
        self.attributes.add_pk_nature_to_attribute(key)
    }
}

/// Generate the .dot table header.
///
/// # Arguments
///
/// * `name` - The name of the table
/// * `dark_mode` - Changes the rendering of the table header if true
fn generate_table_header(name: &str, dark_mode: bool) -> String {
    let styles: (&str, &str) = match dark_mode {
        true => ("grey20", "grey10"),
        false => ("grey95", "indigo"),
    };
    format!(
        "
    {0} [label=<
        <TABLE BGCOLOR=\"{1}\" BORDER=\"1\" CELLBORDER=\"0\" CELLSPACING=\"0\">

        <TR><TD COLSPAN=\"2\" CELLPADDING=\"5\" ALIGN=\"CENTER\" BGCOLOR=\"{2}\">
        <FONT FACE=\"Roboto\" COLOR=\"white\" POINT-SIZE=\"12\">
        <B>{0}</B>
        </FONT></TD></TR>",
        name.trim_leading_trailing(),
        styles.0,
        styles.1
    )
}
