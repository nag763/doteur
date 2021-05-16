use std::fs;
use clap::App;
use std::process::Command;
use which::which;

use doteur::add_trait::{Replacable, ReSearchType};
use doteur::{process_file, write_output_to_file, contains_tables};

#[macro_use] extern crate clap;

const POSSIBLE_DOTS_OUTPUT : [&str; 54] = ["bmp", "canon", "gv", "xdot", "xdot1.2", "xdot1.4",
                                            "cgimage", "cmap", "eps", "eps", "exr", "fig", "gd",
                                            "gd2" , "gif", "gtk", "ico", "imap", "cmapx", "imap_np",
                                            "cmapx_np", "ismap", "jp2", "jpg", "jpeg", "jpe", "json",
                                            "json0", "dot_json", "xdot_json", "pct", "pict","pdf",
                                            "pic", "plain", "plain-ext", "png", "pov", "ps2", "psd",
                                            "sgi", "svg", "svgz", "tga", "tif", "tiff", "tk", "vml",
                                            "vmlz", "vrml", "wbmp", "webp", "xlib", "x11"];

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from(yaml).get_matches();
    let restrictions : Option<(Vec<&str>, ReSearchType)>;


    if matches.is_present("FILENAME"){
        let filename : &str = matches.value_of("FILENAME").unwrap();
        let filename_without_specials = filename.replace_specials();
        let contents = fs::read_to_string(&filename)
            .expect("Something went wrong while reading the file");
        if contains_tables(contents.as_str()) {
            let output_filename : &str = match matches.value_of("output") {
                Some(value) => value,
                _ => "output.dot",
            };
            if matches.is_present("include") {
                restrictions = Some((matches.values_of("include").unwrap().collect::<Vec<&str>>(), ReSearchType::INCLUSIVE));
            } else if matches.is_present("exclude") {
                restrictions = Some((matches.values_of("exclude").unwrap().collect::<Vec<&str>>(), ReSearchType::EXCLUSIVE));
            } else {
                restrictions = None;
            }

            let output_content : String = process_file(&filename_without_specials, contents.as_str(), restrictions);
            let file_ext : &str = get_ext(output_filename);

            if get_ext(output_filename) != "dot" {
                if  which("dot").is_err() {
                    panic!("The dot exe isn't in your path, we couldn't write the output.\nIf you work on linux, use your package manager to download graphviz.\nIf you work on windows, refer to the tutorial or download the tool via the official graphviz site.");
                } else if !ext_supported(file_ext) {
                    panic!("The given extension isn't supported. Please verify it is one of the following :\n\n{}", POSSIBLE_DOTS_OUTPUT.join(";"));
                } else {
                    match write_output_to_file(output_content.as_str(), "output.dot") {
                        Ok(_) => {
                            Command::new("dot")
                                    .arg(["-T", file_ext].join(""))
                                    .arg("output.dot")
                                    .arg(["-o", output_filename].join(""))
                                    .spawn()
                                    .expect("An error happened while writing the output file");

                            println!("The output has been successfully written to the {} file", output_filename);
                        },
                        Err(_) => panic!("An error happened while writing the output file")
                    }
                }
            } else {
                match write_output_to_file(output_content.as_str(), output_filename) {
                    Ok(_) => println!("The output has been successfully written to the {} file", output_filename),
                    Err(_) => panic!("An error happened while writing the output file")
                }
            }
        } else {
            panic!("Sorry, we couldn't find any table for the given file(s), please verify that the format of the file is correct, or report the incident on github");
        }
    } else {
        panic!("Please provide a filename. Use --help to see possibilities");
    }

}


fn get_ext(filename: &str) -> &str {
    std::path::Path::new(filename).extension().unwrap_or_default().to_str().unwrap_or_default()
}


fn ext_supported(ext: &str) -> bool {
    POSSIBLE_DOTS_OUTPUT.iter().any(|&i| i== ext)
}
