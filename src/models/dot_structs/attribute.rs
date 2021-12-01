use std::fmt;

use super::super::add_traits::{Trim};

enum AttributeType {
    PkFk,
    Pk,
    Fk,
    ColDef
}

pub struct Attribute {
    name : String,
    attribute_type : AttributeType,
    associed_definition: Option<String>,
    foreign_table: Option<String>,
    foreign_key: Option<String>,
    dark_mode: bool
}

impl fmt::Display for Attribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.attribute_type {
            AttributeType::ColDef => {

                let font_color : &str = match self.dark_mode {
                    true => "white",
                    false => "black"
                };
                write!(f, "
        <TR><TD ALIGN=\"LEFT\" BORDER=\"0\">
        <FONT COLOR=\"{0}\" FACE=\"Roboto\"><B>{1}</B></FONT>
        </TD><TD ALIGN=\"LEFT\">
        <FONT COLOR=\"{0}\" FACE=\"Roboto\">{2}</FONT>
        </TD></TR>", font_color, self.name.trim_leading_trailing(), self.associed_definition.as_ref().unwrap().trim_leading_trailing()
                )
            },
            AttributeType::Fk => {
                    let font_color : &str = match self.dark_mode {
                        true => "white",
                        false => "black"
                    };
                    let refer_sign : &str = match cfg!(unix) {
                        true => "\u{1F5DD}",
                        _ => "[FK]"
                    };
                    write!(f, "
        <TR><TD ALIGN=\"LEFT\" BORDER=\"0\">
        <FONT COLOR=\"{0}\" FACE=\"Roboto\"><B>{1} {2}</B></FONT>
        </TD><TD ALIGN=\"LEFT\">
        <FONT FACE=\"Roboto\" COLOR=\"{0}\">Refers to <I>{3}[{4}]</I></FONT>
        </TD></TR>", font_color,  self.name.trim_leading_trailing(), refer_sign, self.foreign_table.as_ref().unwrap().trim_leading_trailing(), self.foreign_key.as_ref().unwrap().trim_leading_trailing()
                    )
            },
            _  => write!(f, "")
        }
    }
}

impl Attribute {
    
    pub fn new_col_def(name: String, associed_definition: String, dark_mode: bool) -> Attribute {
        Attribute {
            name, 
            attribute_type: AttributeType::ColDef,
            associed_definition: Some(associed_definition),
            foreign_table: None,
            foreign_key: None,
            dark_mode
        }
    }

    pub fn new_fk(name: String, foreign_table: String, foreign_key: String, dark_mode: bool) -> Attribute {
       Attribute {
           name,
           attribute_type: AttributeType::Fk,
           associed_definition : None,
           foreign_table: Some(foreign_table),
           foreign_key: Some(foreign_key),
           dark_mode
       } 
    }


}
