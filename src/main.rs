use clap::App;
use std::process::Command;
use which::which;

use doteur::models::{ReSearchType, Args, POSSIBLE_DOTS_OUTPUT};
use doteur::{process_file, write_output_to_file, contains_tables};

#[macro_use] extern crate clap;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from(yaml).get_matches();


    let mut args : Args = Args::new(matches.value_of("FILENAME").expect("Please provide a filename. Use --help to see possibilities").to_string());
    if contains_tables(args.get_filecontent()) {
        if let Some(value) = matches.value_of("output") {
            args.set_output_filename(value.to_string()); 
        }
        if matches.is_present("include") {
            args.set_restrictions(Some((matches.values_of("include").unwrap().map(|s| s.to_string()).collect::<Vec<String>>(), ReSearchType::Inclusive)));
        } else if matches.is_present("exclude") {
            args.set_restrictions(Some((matches.values_of("exclude").unwrap().map(|s| s.to_string()).collect::<Vec<String>>(), ReSearchType::Exclusive)));
        } else {
            args.set_restrictions(None);
        }

        let output_content : String = process_file(args.get_filename_without_specials().as_str(), args.get_filecontent(), args.get_restrictions());
        let file_ext : &str = args.get_file_ext();

        if file_ext != "dot" {
            if  which("dot").is_err() {
                panic!("The dot exe isn't in your path, we couldn't write the output.\nIf you work on linux, use your package manager to download graphviz.\nIf you work on windows, refer to the tutorial or download the tool via the official graphviz site.");
            } else if !args.ext_supported() {
                panic!("The given extension isn't supported. Please verify it is one of the following :\n\n{}", POSSIBLE_DOTS_OUTPUT.join(";"));
            } else {
                match write_output_to_file(output_content.as_str(), "output.dot") {
                    Ok(_) => {
                        Command::new("dot")
                                .arg(["-T", file_ext].join(""))
                                .arg("output.dot")
                                .arg(["-o", args.get_output_filename()].join(""))
                                .spawn()
                                .expect("An error happened while writing the output file");

                        println!("The output has been successfully written to the {} file", args.get_output_filename());
                    },
                    Err(_) => panic!("An error happened while writing the output file")
                }
            }
        } else {
            match write_output_to_file(output_content.as_str(), args.get_output_filename()) {
                Ok(_) => println!("The output has been successfully written to the {} file", args.get_output_filename()),
                Err(_) => panic!("An error happened while writing the output file")
            }
        }
    } else {
        panic!("Sorry, we couldn't find any table for the given file(s), please verify that the format of the file is correct, or report the incident on github");
    }
}
