use regex::Regex;
use std::fs;
use clap::App;

#[macro_use] extern crate clap;

fn main() {

    let yaml = load_yaml!("cli.yml");
    let matches = App::from(yaml).get_matches(); 
    
    if matches.is_present("FILENAME"){

        
        let filename : &str = matches.value_of("FILENAME").unwrap();
        let table_defs = Regex::new(r"(?i)(^\s*CREATe).(?i)(\s*TABLE).([^;]*)").unwrap();
        let contents = fs::read_to_string(&filename)
            .expect("Something went wrong reading the file");
        println!("{}", table_defs.is_match(&contents));
    } else {
        print!("Please provide a filename. Use --help to see possibilities");
    }

}
