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
