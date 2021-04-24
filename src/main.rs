use regex::Regex;
use std::fs;
use clap::App;

#[macro_use] extern crate clap;
#[macro_use] extern crate lazy_static;

lazy_static! {
    static ref RE_TABLE_DEFS : Regex = Regex::new(r"(?i)\s*CREATE\sTABLE[^;]*.").unwrap();
    static ref RE_TABLE_NAME : Regex = Regex::new(r"((?i)\s?CREATE\sTABLE\s*[`]?)+(\w*).").unwrap();
}

fn main() {

    let yaml = load_yaml!("cli.yml");
    let matches = App::from(yaml).get_matches(); 
    
    if matches.is_present("FILENAME"){
        let filename : &str = matches.value_of("FILENAME").unwrap();
        let contents = fs::read_to_string(&filename)
            .expect("Something went wrong reading the file");
        let tables : Vec<&str> = RE_TABLE_DEFS.find_iter(&contents)
            .map(|element| element.as_str())
            .collect();
        if tables.len() != 0 {
            println!("Detected tables : {}", tables.len());
            let output_filename : &str = match matches.value_of("output") {
                Some(value) => value,
                _ => "output.dot",
            };
            let mut output_content : String = String::from(init_dot(filename));
            output_content = [output_content, tables.iter().map(|element| convert_sql_to_dot(element)).collect::<Vec<String>>().join("\n")].concat();
            
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

fn convert_sql_to_dot(input: &str) -> String {
    let table_name = RE_TABLE_NAME
        .captures(input)
        .unwrap()
        .get(2)
        .map_or("TABLE NAME", |t| t.as_str().trim_start().trim_end());
    let table_header : String = generate_table_header(table_name);
    let begin_dec : usize;
    let end_dec : usize;
    match input.find('('){
        Some(v) => begin_dec = v,
        None => return close_table(table_header.as_str())
    }
    match input.rfind(')') {
        Some(v) => end_dec = v,
        None => return close_table(table_header.as_str())
    }
    let body_content : String = input
        .chars()
        .take(end_dec)
        .skip(begin_dec+1)
        .collect::<String>()
        .split(',')
        .map(|s| generate_attributes(s).to_string())
        .collect::<Vec<String>>()
        .join("\n"); 
    
    close_table([table_header, body_content].join("\n").as_str())
}

fn init_dot(filename: &str) -> String {
    format!("digraph {} {{\n
    node [\n
        shape = \"plaintext\"\n
    ]\n", filename)
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
        let trimed : &str = attr.trim_start().trim_end();
        if trimed.chars().collect::<Vec<char>>()[0] == '`' {
            let splitted = trimed
                .split('`')
                .map(|s| s.to_string())
                .collect::<Vec<String>>();
            title = splitted[1].to_string();
            rest = splitted[2].trim_start().to_string();
        } else {
            let mut splitted = trimed
                .split(' ')
                .map(|s| s.to_string())
                .collect::<Vec<String>>();
            title = splitted.remove(0).to_string();
            rest = splitted.join(" ").into();
        }
        format!("<TR><TD ALIGN=\"LEFT\" BORDER=\"0\">
                <FONT FACE=\"Roboto\">{0}</FONT>
                </TD><TD ALIGN=\"LEFT\">
                <FONT FACE=\"Roboto\">{1}</FONT>
                </TD></TR>", title, rest
        )
    } else {
        "".to_string()
    }
}
