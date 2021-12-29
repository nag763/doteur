use std::fmt;

use super::super::add_traits::Trim;

/// The attribute type
enum AttributeType {
    PkFk,
    Pk,
    Fk,
    ColDef,
}

/// An attribute is a modelisation of a column
/// in SQL
pub struct Attribute {
    /// Name of the attribute
    name: String,
    /// Type of the attribute
    attribute_type: AttributeType,
    /// The definition associed to the attribute if
    /// appliable
    associed_definition: Option<String>,
    /// The refered table if appliable
    foreign_table: Option<String>,
    /// The refered key if appliable
    foreign_key: Option<String>,
    /// Whether the output needs to be rendered
    /// for dm or not
    dark_mode: bool,
}

impl fmt::Display for Attribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.attribute_type {
            AttributeType::ColDef => {
                let font_color: &str = match self.dark_mode {
                    true => "white",
                    false => "black",
                };
                write!(
                    f,
                    "
        <TR><TD ALIGN=\"LEFT\" BORDER=\"0\">
        <FONT COLOR=\"{0}\" FACE=\"Roboto\"><B>{1}</B></FONT>
        </TD><TD ALIGN=\"LEFT\">
        <FONT COLOR=\"{0}\" FACE=\"Roboto\">{2}</FONT>
        </TD></TR>",
                    font_color,
                    self.name.trim_leading_trailing(),
                    self.associed_definition
                        .as_ref()
                        .unwrap()
                        .trim_leading_trailing()
                )
            }

            AttributeType::Fk => {
                let font_color: &str = match self.dark_mode {
                    true => "white",
                    false => "black",
                };
                let refer_sign: &str = match cfg!(unix) {
                    true => "\u{1F5DD}",
                    _ => "[FK]",
                };
                write!(
                    f,
                    "
        <TR><TD ALIGN=\"LEFT\" BORDER=\"0\">
        <FONT COLOR=\"{0}\" FACE=\"Roboto\"><B>{1} {2}</B></FONT>
        </TD><TD ALIGN=\"LEFT\">
        <FONT FACE=\"Roboto\" COLOR=\"{0}\">Refers to <I>{3}[{4}]</I></FONT>
        </TD></TR>",
                    font_color,
                    self.name.trim_leading_trailing(),
                    refer_sign,
                    self.foreign_table.as_ref().unwrap().trim_leading_trailing(),
                    self.foreign_key.as_ref().unwrap().trim_leading_trailing()
                )
            }

            AttributeType::Pk => {
                let font_color: &str = match self.dark_mode {
                    true => "white",
                    false => "black",
                };
                let pk_sign: &str = match cfg!(unix) {
                    true => "\u{1F511}",
                    _ => "[PK]",
                };
                write!(
                    f,
                    "
        <TR><TD ALIGN=\"LEFT\" BORDER=\"0\">
        <FONT COLOR=\"{0}\" FACE=\"Roboto\"><B>{1} {2}</B></FONT>
        </TD><TD ALIGN=\"LEFT\">
        <FONT FACE=\"Roboto\" COLOR=\"{0}\">{3}</FONT>
        </TD></TR>",
                    font_color,
                    self.name.trim_leading_trailing(),
                    pk_sign,
                    self.associed_definition
                        .as_ref()
                        .unwrap()
                        .trim_leading_trailing()
                )
            }

            AttributeType::PkFk => {
                let font_color: &str = match self.dark_mode {
                    true => "white",
                    false => "black",
                };
                let pk_sign: &str = match cfg!(unix) {
                    true => "\u{1F511}",
                    _ => "[PK]",
                };
                let refer_sign: &str = match cfg!(unix) {
                    true => "\u{1F5DD}",
                    _ => "[FK]",
                };
                write!(
                    f,
                    "
        <TR><TD ALIGN=\"LEFT\" BORDER=\"0\">
        <FONT COLOR=\"{0}\" FACE=\"Roboto\"><B>{1} {2}{3}</B></FONT>
        </TD><TD ALIGN=\"LEFT\">
        <FONT FACE=\"Roboto\" COLOR=\"{0}\">Refers to <I>{4}[{5}]</I></FONT>
        </TD></TR>",
                    font_color,
                    self.name.trim_leading_trailing(),
                    pk_sign,
                    refer_sign,
                    self.foreign_table.as_ref().unwrap().trim_leading_trailing(),
                    self.foreign_key.as_ref().unwrap().trim_leading_trailing()
                )
            }
        }
    }
}

impl Attribute {
    /// Define a new sql column
    ///
    /// # Arguments
    ///
    /// * `name` - Name of the column
    /// * `associed_definition` - Associed definition
    /// * `dark_mode` - Whether the output needs to be rendered in dark mode or not
    pub fn new_col_def(name: String, associed_definition: String, dark_mode: bool) -> Attribute {
        Attribute {
            name,
            attribute_type: AttributeType::ColDef,
            associed_definition: Some(associed_definition),
            foreign_table: None,
            foreign_key: None,
            dark_mode,
        }
    }

    /// Define a new primary key
    ///
    /// # Arguments
    ///
    /// * `name` - Name of the column
    /// * `associed_definition` - Associed definition
    /// * `dark_mode` - Whether the output needs to be rendered in dark mode or not
    pub fn new_pk(name: String, associed_definition: String, dark_mode: bool) -> Attribute {
        Attribute {
            name,
            attribute_type: AttributeType::Pk,
            associed_definition: Some(associed_definition),
            foreign_table: None,
            foreign_key: None,
            dark_mode,
        }
    }

    /// Add PK nature to the current attribute
    pub fn add_pk_nature(&mut self) {
        match self.attribute_type {
            AttributeType::Fk => {
                self.attribute_type = AttributeType::PkFk;
            }
            AttributeType::ColDef => self.attribute_type = AttributeType::Pk,
            _ => (),
        };
    }

    /// Add FK nature to a current attribute
    ///
    /// # Arguments
    ///
    /// * `foreign_table` - Refered table
    /// * `foreign_key` - Refered key
    pub fn add_fk_nature(&mut self, foreign_table: String, foreign_key: String) {
        match self.attribute_type {
            AttributeType::Pk => {
                self.attribute_type = AttributeType::PkFk;
                self.foreign_table = Some(foreign_table);
                self.foreign_key = Some(foreign_key);
            }
            AttributeType::ColDef => {
                self.attribute_type = AttributeType::Fk;
                self.foreign_table = Some(foreign_table);
                self.foreign_key = Some(foreign_key);
            }
            _ => (),
        };
    }
}

/// Trait for retrieving and modifying values from a vec
pub trait KeyValueMap {
    /// Returns the index of an attribute
    ///
    /// # Arguments
    ///
    /// * `attr_name` - Name of the attribute to be retrieved
    fn index_of_attribute(&mut self, attr_name: &str) -> Result<usize, &'static str>;

    /// Add PK nature to an attribute in the vec
    ///
    /// # Arguments
    ///
    /// * `attr_name` - Name of the attribute to be retrieved
    fn add_pk_nature_to_attribute(&mut self, attr_name: &str) -> Result<usize, &'static str>;

    /// Add FK nature to an existing attribute
    ///
    /// # Arguments
    ///
    /// * `attr_name` - Name of the attribute to be retrieved
    /// * `foreign_table` - Name of the refered table
    /// * `foreign_key` - Name of the refered key
    fn add_fk_nature_to_attribute(
        &mut self,
        attr_name: &str,
        foreign_table: &str,
        foreign_key: &str,
    ) -> Result<usize, &'static str>;

    /// Push attribute or replace it
    ///
    /// # `attr_name` - Name of the attribute to be retrieved
    fn push_or_replace_attribute(&mut self, value: Attribute);
}

impl KeyValueMap for Vec<Attribute> {
    fn index_of_attribute(&mut self, attr_name: &str) -> Result<usize, &'static str> {
        let index: Option<usize> = self
            .iter_mut()
            .enumerate()
            .filter(|(_, attr)| attr.name == attr_name)
            .map(|(i, _)| i)
            .last();

        match index {
            Some(v) => Ok(v),
            None => Err("No such attribute"),
        }
    }

    fn push_or_replace_attribute(&mut self, value: Attribute) {
        match self.index_of_attribute(value.name.as_str()) {
            Ok(index) => {
                let _ = std::mem::replace(&mut self[index], value);
            }
            Err(_) => self.push(value),
        };
    }

    fn add_pk_nature_to_attribute(&mut self, attr_name: &str) -> Result<usize, &'static str> {
        match self.index_of_attribute(attr_name) {
            Ok(index) => {
                self[index].add_pk_nature();
                Ok(index)
            }
            Err(_) => Err("Index not found"),
        }
    }

    fn add_fk_nature_to_attribute(
        &mut self,
        attr_name: &str,
        foreign_table: &str,
        foreign_key: &str,
    ) -> Result<usize, &'static str> {
        match self.index_of_attribute(attr_name) {
            Ok(index) => {
                self[index].add_fk_nature(foreign_table.to_string(), foreign_key.to_string());
                Ok(index)
            }
            Err(_) => Err("Index not found"),
        }
    }
}
