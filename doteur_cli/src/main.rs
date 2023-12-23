//! Copyright ⓒ 2021-2022 LABEYE Loïc
//! This tool is distributed under the MIT License, check out [here](https://github.com/nag763/doteur/blob/main/LICENCE.MD).
//! # General information
//! <p align="center"><img src="https://raw.githubusercontent.com/nag763/doteur/main/.github/assets/logo.png"></img></p>
//! <h2 align="center">Doteur</h2>
//! <h4 align="center">A simple tool to draw your mysql relations from exports.</h4>
//! <p align="center"><img height ="480" width="640" src="https://raw.githubusercontent.com/nag763/doteur/main/.github/assets/sample.jpeg"></img></p>
//! <u>Warning :</u> It is highly recommended to install <a href="https://graphviz.org/download/">Graphviz</a> prior using this tool
//! <p>Doteur is a CLI (Command Line Interface) tool that has for purpose to render the SQL schemas into good looking graphs. This will help you to easily understand the structure of a large database and understand what happens behind the scenes of your project.</p>
//! Besides, you will be able to use the large panel of features to either sort the tables you want to visualize or render with a different color scheme for instance.
//! So far the tool handles both the MySQL and SQLite syntaxes, but it is planned to handle the Postgre one as soon as the formers will be considered as stable. The input of the tool can be either a sql file export, or given the version you downloaded, connect to either a MySQL running instance or an existing SQLite database.
//! The tool has been developed on Linux, but is also available for Windows 10 and 11 and macOS.
//! <br/>
//! <p>Useful links :</p>
//! <ul>
//! <li><a href="https://github.com/nag763/doteur"/>Github repository</a></li>
//! <li><a href="https://nag763.github.io/doteur"/>Official documentation</a></li>
//! <li><a href="https://docker.com/nag763/doteur">Docker tool</a></li>
//! </ul>

/// Module used to parse the user CLI args
///
/// This module mainly use clap derive in order to get the user input
/// and use them in order to render
mod args;
/// Module used to handle common errors
mod errors;

use std::env;
use std::process::Command;
use which::which;

use crate::args::{Args, POSSIBLE_DOTS_OUTPUT};
use crate::errors::DoteurCliError;

use doteur_core::tools::write_output_to_file;
use doteur_core::{contains_sql_tables, process_data};

use clap::Parser;

#[macro_use]
extern crate cfg_if;

/// Entry point of the cli
///
/// Returns 0 if the process executed correctly, 1 if an error has been thrown during the exectuion
fn main() {
    env_logger::init();
    std::process::exit(match run_main() {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("An error happened : {}", err);
            eprintln!("\nIf you think the error shouldn't be happening, please raise and detail an issue on github : https://github.com/nag763/doteur/issues/new/choose");
            eprintln!("The application finished with return code 1");
            1
        }
    });
}

/// Runnable used to call the core libraries
fn run_main() -> Result<(), Box<dyn std::error::Error>> {
    // Bind args from clap
    let args: Args = Args::parse();

    let data: String = args.get_data()?;

    if contains_sql_tables(data.as_str()) {
        let output_content: String = process_data(
            data.as_str(),
            args.get_restrictions().as_ref(),
            args.get_legend(),
            args.get_dark_mode(),
        );
        let file_ext: &str = args.get_output_file_ext();

        // If it required to render in another format than the dot one, we need to check if
        // the graphviz library is in the system's path
        if file_ext != "dot" {
            if which("dot").is_err() {
                Err(DoteurCliError::dot_exe_not_in_path().into())
            } else if !args.can_render_with_graphviz() {
                Err(DoteurCliError::ext_not_supported(&POSSIBLE_DOTS_OUTPUT.join(";")).into())
            } else {
                let temp_dir = env::temp_dir();
                let temp_file_location = format!("{}/.output.dot", temp_dir.to_str().unwrap());
                write_output_to_file(output_content.as_str(), &temp_file_location)?;
                Command::new("dot")
                    .arg(["-T", file_ext].join(""))
                    .arg(&temp_file_location)
                    .arg(["-o", args.get_output_filename()].join(""))
                    .spawn()?;

                println!(
                    "The output has been successfully written to the {} file",
                    args.get_output_filename()
                );
                Ok(())
            }
        } else {
            write_output_to_file(output_content.as_str(), args.get_output_filename())?;
            println!(
                "The output has been successfully written to the {} file",
                args.get_output_filename()
            );
            Ok(())
        }
    } else {
        Err(DoteurCliError::no_table_found().into())
    }
}
