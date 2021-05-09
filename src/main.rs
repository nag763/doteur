use regex::Regex;
use std::fs;
use clap::App;
use std::path::Path;
use std::ffi::OsStr;

#[macro_use] extern crate clap;
#[macro_use] extern crate lazy_static;

lazy_static! {
    static ref RE_TABLE_DEFS : Regex = Regex::new(r"(?i)\s*CREATE\sTABLE[^;]*.").unwrap();
    static ref RE_TABLE_NAME : Regex = Regex::new(r"((?i)\s?CREATE\sTABLE\s*[`]?)+(\w*).").unwrap();
    static ref RE_FK : Regex = Regex::new(r"(?i)\sFOREIGN\sKEY").unwrap();
    static ref RE_IN_PARENTHESES : Regex = Regex::new(r"([^`][a-zA-Z]*\s*)(\(([^()]+)\))").unwrap();
    static ref RE_SEP_COMA : Regex = Regex::new(r",\s").unwrap();
    static ref RE_ALTERED_TABLE : Regex = Regex::new(r"(\s(?i)ALTER TABLE\s*)(`?(\w*)`?)([^;]*)").unwrap();
}

fn main() {

    let yaml = load_yaml!("cli.yml");
    let matches = App::from(yaml).get_matches(); 
    
    if matches.is_present("FILENAME"){
        let filename : &str = matches.value_of("FILENAME").unwrap();
        let contents = fs::read_to_string(&filename)
            .expect("Something went wrong while reading the file");
        let tables : Vec<&str> = RE_TABLE_DEFS.find_iter(&contents)
            .map(|element| element.as_str())
            .collect();
        if tables.len() != 0 {
            println!("Detected tables : {}", tables.len());
            let output_filename : &str = match matches.value_of("output") {
                Some(value) => value,
                _ => "output.dot",
            };
            let generated_content : Vec<(String, String)> = tables.iter().map(|element| convert_sql_to_dot(element)).collect::<Vec<(String, String)>>();
            let other_fks : Vec<&str> = RE_ALTERED_TABLE.find_iter(&contents)
                .map(|element| element.as_str())
                .collect();
            let other_relations : Vec<String> = other_fks
                .iter()
                .map(|element| 
                     // TODO : Améliorer ici, deux fois appel au regex
                     generate_relations(
                         RE_ALTERED_TABLE.captures(element)
                                        .unwrap()
                                        .get(3)
                                        .map(|s| s.as_str())
                                        .unwrap(), 
                        RE_ALTERED_TABLE.captures(element)
                                        .unwrap()
                                        .get(4)
                                        .map(|s| s.as_str())
                                        .unwrap()
                     ).unwrap_or_default()
                )
                .collect::<Vec<String>>();

            let other_relations_as_str : Vec<&str> = other_relations
                .iter()
                .map(|s| s.as_str())
                .collect::<Vec<&str>>();
            let output_content = [
                init_dot(filename), 
                generated_content
                    .iter()
                    .map(|element| element.0.as_str())
                    .collect::<Vec<&str>>()
                    .join("\n"),
                generated_content
                    .iter()
                    .map(|element| element.1.as_str())
                    .collect::<Vec<&str>>()
                    .into_iter()
                    .chain(other_relations_as_str.into_iter())
                    .collect::<Vec<&str>>()
                    .join("\n"),
            ].concat();
            
            match write_output_to_file(close_dot(output_content.as_str()).as_str(), output_filename) {
                Ok(_) => println!("The output has been successfully written to the {} file", output_filename),
                Err(_) => println!("An error happened while writing the output file")
            }
        } else {
            println!("Sorry, we couldn't find any table for the given file(s), please verify that the format of the file is correct, or report the incident on github");
        }
    } else {
        print!("Please provide a filename. Use --help to see possibilities");
    }

}

fn convert_sql_to_dot(input: &str) -> (String, String) {
    let table_name = RE_TABLE_NAME
        .captures(input)
        .unwrap()
        .get(2)
        .map_or("TABLE NAME".to_string(), |t| trim_leading_trailing(t.as_str()));
    let table_header : String = generate_table_header(table_name.as_str());
    let begin_dec : usize;
    let end_dec : usize;
    match input.find('('){
        Some(v) => begin_dec = v,
        None => return (close_table(table_header.as_str()), "".to_string())
    }
    match input.rfind(')') {
        Some(v) => end_dec = v,
        None => return (close_table(table_header.as_str()), "".to_string())
    }
    let lines : Vec<String> = RE_SEP_COMA.split(input
        .chars()
        .take(end_dec)
        .skip(begin_dec+1)
        .collect::<String>()
        .as_str()
        ).map(|s| s.to_string())
        .collect::<Vec<String>>();

    let generated : Vec<(String, Option<String>)> = lines
        .iter()
        .map(|s| (generate_attributes(s), generate_relations(&table_name, s)))
        .collect::<Vec<(String, Option<String>)>>();

    let body_content : String = generated
        .iter()
        .map(|s| s.0.as_str())
        .collect::<Vec<&str>>()
        .join("\n");

    let relations : String = generated
        .iter()
        .map(|s| s.1.as_deref().unwrap_or_default())
        .filter(|s| s.len() != 0)
        .collect::<Vec<&str>>()
        .join("\n");

    (close_table([table_header, body_content].join("\n").as_str()), relations)
}

fn init_dot(filename: &str) -> String {
    format!("digraph {} {{\n
    node [\n
        shape = \"plaintext\"\n
    ]\n", Path::new(filename).file_stem().unwrap_or(OsStr::new("sql")).to_str().unwrap_or("sql"))
}

fn close_dot(opened_dot: &str) -> String {
    format!("{}\n}}", opened_dot)
}

fn write_output_to_file(content: &str, filename: &str) -> std::io::Result<()>{
    fs::write(filename ,content)?;
    Ok(())
}   

fn generate_table_header(name: &str) -> String {
    format!("{0} [label=<
    <TABLE BGCOLOR=\"white\" BORDER=\"1\" CELLBORDER=\"0\" CELLSPACING=\"0\">
    <TR><TD COLSPAN=\"2\" CELLPADDING=\"5\" ALIGN=\"CENTER\" BGCOLOR=\"blue\">
    <FONT FACE=\"Roboto\" COLOR=\"white\" POINT-SIZE=\"10\"><B>
    {0}
    </B></FONT></TD></TR>", name)
}

fn close_table(table: &str) -> String {
    format!("{}\n</TABLE> >]", table)
}

fn generate_attributes(attr: &str) -> String {
    if !attr.to_lowercase().contains("key") {
        let title : String;
        let rest : String;
        let trimed : String = trim_leading_trailing(attr);
        if trimed.chars().collect::<Vec<char>>()[0] == '`' {
            let splitted = trimed
                .split('`')
                .map(|s| s.to_string())
                .collect::<Vec<String>>();
            title = splitted[1].to_string();
            rest = trim_leading_trailing(splitted[2].as_str());
        } else {
            let mut splitted = trimed
                .split(' ')
                .map(|s| s.to_string())
                .collect::<Vec<String>>();
            title = splitted.remove(0).to_string();
            rest = splitted.join(" ").into();
        }
        format!("<TR><TD ALIGN=\"LEFT\" BORDER=\"0\">
                <FONT FACE=\"Roboto\"><B>{0}</B></FONT>
                </TD><TD ALIGN=\"LEFT\">
                <FONT FACE=\"Roboto\">{1}</FONT>
                </TD></TR>", trim_leading_trailing(title.as_str()), trim_leading_trailing(rest.as_str())
        )
    } else {
        let is_fk : bool = RE_FK.find_iter(attr).map(|s| s.as_str()).collect::<Vec<&str>>().len() != 0;
        if is_fk {
            let matches : Vec<&str> = RE_IN_PARENTHESES
                .find_iter(attr)
                .map(
                    |s| s.as_str()
                ).collect::<Vec<&str>>();
            let title : String = trim_leading_trailing(
                matches[0]
                .chars()
                .take(matches[0].len()-1)
                .skip(matches[0].find('(').unwrap()+1) 
                .collect::<String>()
                .as_str()
                );
            format!("<TR><TD ALIGN=\"LEFT\" BORDER=\"0\">
                <FONT FACE=\"Roboto\"><B>[FK] {0}</B></FONT>
                </TD><TD ALIGN=\"LEFT\">
                <FONT FACE=\"Roboto\">Refers to {1}</FONT>
                </TD></TR>", title.replace("`", ""), trim_leading_trailing(matches[1]).replace("`", "")
            )

        } else {
            "".to_string()
        }
    }
}


fn generate_relations(table_name : &str, input: &str) -> Option<String> {
    let is_fk : bool = RE_FK.find_iter(input).map(|s| s.as_str()).collect::<Vec<&str>>().len() != 0;
    if is_fk {
        let replaced : &str = &input.replace("`", "");
        let matches : Vec<&str> = RE_IN_PARENTHESES.find_iter(replaced).map(|s| s.as_str()).collect();
        if matches.len() != 0 {
            let table_end : &str = matches[1].split("(").collect::<Vec<&str>>()[0];
            Some(format!("{} -> {} [label=\"{} refers {}\"]", table_name, table_end, matches[0], matches[1]))
        } else {
            None
        }
    } else {
        None
    }
}


fn trim_leading_trailing(input : &str) -> String {
    input.trim_start().trim_end().to_string()
}
