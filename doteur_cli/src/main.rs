mod args;
mod errors;

use std::process::Command;
use which::which;

use crate::args::{Args, POSSIBLE_DOTS_OUTPUT};
use crate::errors::DoteurCliError;

use doteur_core::tools::write_output_to_file;
use doteur_core::{contains_tables, process_data};

use clap::Parser;

#[macro_use]
extern crate cfg_if;

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

fn run_main() -> Result<(), Box<dyn std::error::Error>> {
    // Bind args from clap
    let args: Args = Args::parse();

    let data: String = args.get_data()?;

    if contains_tables(data.as_str()) {
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
                write_output_to_file(output_content.as_str(), ".output.dot")?;
                Command::new("dot")
                    .arg(["-T", file_ext].join(""))
                    .arg(".output.dot")
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
