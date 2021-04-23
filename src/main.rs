use regex::Regex;
use std::fs;
use clap::App;

#[macro_use] extern crate clap;
#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref RE_TABLE_DEFS : Regex = Regex::new(r"(?i)(^\sCREATE\sTABLE[^;]*.)").unwrap();
    static ref RE_DB_DEFS : Regex = Regex::new(r"(?i)(^\sCREATE\sDATABASE[^\n])").unwrap();
}

fn main() {

    let yaml = load_yaml!("cli.yml");
    let matches = App::from(yaml).get_matches(); 
    
    if matches.is_present("FILENAME"){

        let filename : &str = matches.value_of("FILENAME").unwrap();
        let contents = fs::read_to_string(&filename)
            .expect("Something went wrong reading the file");
        let databases : Vec<&str> = RE_DB_DEFS.find_iter(&contents)
            .map(|element| element.as_str())
            .collect();
        let tables : Vec<&str> = RE_TABLE_DEFS.find_iter(&contents)
            .map(|element| element.as_str())
            .collect();
        println!("Databases found : {}", databases.len());
        println!("Tables found : {}", tables.len());
    } else {
        print!("Please provide a filename. Use --help to see possibilities");
    }

}
