mod args;

#[cfg(feature = "mysql_addons")]
use dialoguer::{Input, Password};
use std::process::Command;
use which::which;

use crate::args::{get_clap_args, Args, POSSIBLE_DOTS_OUTPUT};

use doteur_core::tools::write_output_to_file;
use doteur_core::{contains_tables, process_data};

#[macro_use]
extern crate cfg_if;

fn main() {
    env_logger::init();
    std::process::exit(match run_main() {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("An error happened : {:?}", err);
            eprintln!(
                "Please use the --help argument if you need further informations about the tool."
            );
            eprintln!("If you think the error shouldn't be happening, please raise and detail an issue on github : https://github.com/nag763/doteur/issues/new/choose");
            eprintln!("The application finished with return code 1");
            1
        }
    });
}

fn run_main() -> Result<(), Box<dyn std::error::Error>> {
    // Bind args from clap
    let args = get_args_from_clap()?;

    if contains_tables(args.get_data()) {
        let output_content: String = process_data(
            args.get_data(),
            args.get_restrictions(),
            args.get_filename_without_specials().as_str(),
            args.get_legend(),
            args.get_dark_mode(),
        );
        let file_ext: &str = args.get_output_file_ext();

        // If it required to render in another format than the dot one, we need to check if
        // the graphviz library is in the system's path
        if file_ext != "dot" {
            if which("dot").is_err() {
                Err("The dot exe isn't in your path, we couldn't write the output.If you work on linux, use your package manager to download graphviz.If you work on windows, refer to the tutorial or download the tool via the official graphviz site.Graphviz official download page : https://graphviz.org/download/.".into())
            } else if !Args::ext_supported(file_ext) {
                Err(format!("The given extension isn't supported. Please verify it is one of the following :\n\n{}", POSSIBLE_DOTS_OUTPUT.join(";")).into())
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
        Err("No tables have been found for the given input".into())
    }
}

fn get_args_from_clap() -> Result<Args, Box<dyn std::error::Error>> {
    let matches = get_clap_args().get_matches();

    let mut args: Args;
    if matches.is_present("interactive") {
        // Interactive dialog is only available for the mysql feature
        cfg_if! {
            if #[cfg(feature="mysql_addons")] {
                let db_url: String = Input::new()
                    .with_prompt("Database url or ip")
                    .default("localhost".into())
                    .interact_text()
                    .unwrap();

                let db_port: u16 = Input::new()
                    .with_prompt("Database port")
                    .default(3306)
                    .interact_text()
                    .unwrap();

                let db_name: String = Input::new()
                    .with_prompt("Database name")
                    .interact_text()
                    .unwrap();

                let db_user: String = Input::new()
                    .with_prompt("Database user")
                    .interact_text()
                    .unwrap();

                let db_password: String = Password::new()
                    .with_prompt("Database user's password")
                    .interact()
                    .unwrap();

                args = Args::new_connect_with_params(db_url, db_port, db_name, db_user, db_password)?;
            // Should be unreachable as the interactive command isn't compiled in that case
            } else {
                return Err("Not available for your configuration".into());
            }
        }
    } else {
        // If we don't have the interactive command passed, we need to run the tool through the
        // input arg
        let input: Vec<&str> = match matches.values_of("input") {
            Some(v) => v.collect(),
            None => return Err("Please precise at least one argument as input".into()),
        };
        // Fake error thrown by clippy
        #[allow(clippy::if_same_then_else)]
        if matches.is_present("url") {
            cfg_if! {
                if #[cfg(feature="mysql_addons")] {
                    if input.len() != 1 {
                        return Err(
                            "Please ensure that if the url argument is present that only one url is passed"
                                .into(),
                        );
                    }
                    args = Args::new_from_url(input[0])?;
                // Shouldn't be reachable as it the subcommand wouldn't be compiled without the
                // mysql feature
                } else {
                    return Err("Not available for your configuration".into());
                }
            }
        } else if matches.is_present("sqlite") {
            cfg_if! {
                if #[cfg(feature="sqlite_addons")] {
                    if input.len() != 1 {
                        return Err(
                            "Please ensure that only one sqlite database path is passed as argument".into(),
                        );
                    }
                    args = Args::new_from_sqlite(input[0])?;
                // Shouldn't be reachable as the option shouldn't be compiled without the sqlite
                // feature
                } else {
                    return Err("Not available for your configuration".into());
                }
            }
        } else {
            args = Args::new_from_files(input)?;
        }
    }
    if let Some(value) = matches.value_of("output") {
        args.set_output_filename(value.to_string());
    }
    if matches.is_present("include") {
        args.set_inclusions(
            matches
                .values_of("include")
                .unwrap()
                .map(|s| s.to_string())
                .collect::<Vec<String>>(),
        );
    } else if matches.is_present("exclude") {
        args.set_exclusions(
            matches
                .values_of("exclude")
                .unwrap()
                .map(|s| s.to_string())
                .collect::<Vec<String>>(),
        );
    }
    args.set_dark_mode(matches.is_present("dark_mode"));
    args.set_dark_mode(matches.is_present("dark_mode"));
    args.set_legend(matches.is_present("legend"));
    Ok(args)
}
