use std::fs;
use clap::App;
use sqltodot::{process_file, write_output_to_file, contains_tables};

#[macro_use] extern crate clap;

fn main() {

    let yaml = load_yaml!("cli.yml");
    let matches = App::from(yaml).get_matches();

    if matches.is_present("FILENAME"){
        let filename : &str = matches.value_of("FILENAME").unwrap();
        let contents = fs::read_to_string(&filename)
            .expect("Something went wrong while reading the file");
        if contains_tables(contents.as_str()) {
            let output_filename : &str = match matches.value_of("output") {
                Some(value) => value,
                _ => "output.dot",
            };

            let output_content : String = process_file(filename, contents.as_str());

            match write_output_to_file(output_content.as_str(), output_filename) {
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
