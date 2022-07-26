// Copyright ⓒ 2021-2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/doteur/blob/main/LICENCE.MD).

use std::fs;
use std::path::{Path, PathBuf};

use crate::DoteurCliError;
use doteur_core::restriction::Restriction;

#[cfg(feature = "mysql_addons")]
use doteur_core::mysql_tools::{get_schemas_from_mysql_params, get_schemas_from_mysql_url};

#[cfg(feature = "mysql_addons")]
use dialoguer::{Input, Password};

#[cfg(feature = "sqlite_addons")]
use doteur_core::sqlite_tools::get_schemas_from_sqlite_instance;

use clap::Parser;

/// Possible dot output formats.
pub const POSSIBLE_DOTS_OUTPUT: [&str; 53] = [
    "bmp",
    "canon",
    "gv",
    "xdot",
    "xdot1.2",
    "xdot1.4",
    "cgimage",
    "cmap",
    "eps",
    "eps",
    "exr",
    "fig",
    "gd",
    "gd2",
    "gif",
    "gtk",
    "ico",
    "imap",
    "cmapx",
    "imap_np",
    "cmapx_np",
    "ismap",
    "jp2",
    "jpg",
    "jpeg",
    "jpe",
    "json",
    "json0",
    "dot_json",
    "xdot_json",
    "pct",
    "pict",
    "pdf",
    "pic",
    "plain",
    "plain-ext",
    "png",
    "pov",
    "ps2",
    "psd",
    "sgi",
    "svg",
    "svgz",
    "tga",
    "tif",
    "tiff",
    "tk",
    "vml",
    "vmlz",
    "vrml",
    "wbmp",
    // Webp format scraped as it isn't supported by the stable graphviz version
    //"webp",
    "xlib",
    "x11",
];

#[derive(Parser)]
#[clap(
    author = "LABEYE Loïc <loic.labeye@pm.me>",
    version = "0.5.3",
    about = "Parse a SQL configuration and convert it into a .dot file, render the output if Graphviz is installed",
    after_help = "Some functionnalities might not appear as they depend on which version this tool has been downloaded or built for."
)]
pub struct Args {
    #[clap(required = false, index = 1)]
    /// Name of the sql file or database location if an URL arg is passed, can also be a directory or several files
    input: Vec<String>,
    #[clap(long = "output", short = 'o', default_value = "output.dot")]
    /// Name of the output file
    output: String,
    #[cfg(feature = "mysql_addons")]
    #[clap(long = "url", conflicts_with_all = &["it", "sqlite"])]
    /// Specificate that the input is an URL (i.e. mysql://usr:password@localhost:3306/database)
    url: bool,
    #[cfg(feature = "mysql_addons")]
    #[clap(long = "it", conflicts_with_all = &["url", "sqlite"])]
    /// Starts an interactive dialog to connect to a remote database
    interactive: bool,
    #[cfg(feature = "sqlite_addons")]
    #[clap(long = "sqlite", conflicts_with_all = &["url", "interactive"])]
    /// Starts an interactive dialog to connect to a remote database
    sqlite: bool,
    #[clap(short = 'i', long = "include")]
    /// Filter to include only the given tables, accept simple regexs
    include: Vec<String>,
    #[clap(short = 'x', long = "exclude", conflicts_with = "include")]
    /// Filter to exclude the given tables, accept simple regexs
    exclude: Vec<String>,
    #[clap(long = "dark_mode")]
    /// Wheter to render in dark mode or not
    dark_mode: bool,
    #[clap(long = "legend")]
    /// Includes hint about the relations type at the bottom of the output file
    legend: bool,
}

impl Args {
    pub fn get_data(&self) -> Result<String, Box<dyn std::error::Error>> {
        cfg_if! {
            if #[cfg(feature="mysql_addons")] {
                if self.interactive {
                    // Interactive dialog is only available for the mysql feature
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
                            let data : String = get_schemas_from_mysql_params(db_url, db_port, db_name, db_user, db_password)?;
                            return Ok(data);
                    }
                if self.url {
                    if self.input.len() != 1 {
                        return Err(DoteurCliError::bad_input("Please ensure that if the url argument is present that only one url is passed").into());
                    } else {
                        let data : String = get_schemas_from_mysql_url(&self.input[0])?;
                        return Ok(data);
                    }
                }
            }
        }
        cfg_if! {
            if #[cfg(feature="sqlite_addons")] {
                if self.sqlite {
                    if self.input.len() != 1 {
                        return Err(DoteurCliError::bad_input("Please ensure that only one sqlite database path is passed as argument").into(),
                        );
                    } else {
                        let data : String = get_schemas_from_sqlite_instance(&self.input[0])?;
                        return Ok(data);
                    }
                }
            }
        }
        if !self.input.is_empty() {
            let mut data: Vec<String> = vec![];
            // Reads the filecontent of a directory, ignores subdirectories
            for path in self.input.iter() {
                if Path::new(path).is_dir() {
                    for subpath in fs::read_dir(path)? {
                        let file_path: &PathBuf = &subpath.unwrap().path();
                        // Ignore subdirs
                        if Path::new(file_path).is_file() {
                            data.push(fs::read_to_string(file_path)?);
                        }
                    }
                } else {
                    data.push(fs::read_to_string(path)?);
                }
            }
            Ok(data.join("\n"))
        } else {
            Err(DoteurCliError::no_input().into())
        }
    }

    pub fn get_restrictions(&self) -> Option<Restriction> {
        if !self.include.is_empty() {
            Some(Restriction::new_inclusion(self.include.clone()))
        } else if !self.exclude.is_empty() {
            Some(Restriction::new_exclusion(self.exclude.clone()))
        } else {
            None
        }
    }

    pub fn get_output_file_ext(&self) -> &str {
        std::path::Path::new(self.output.as_str())
            .extension()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default()
    }

    pub fn get_output_filename(&self) -> &str {
        &self.output
    }

    pub fn can_render_with_graphviz(&self) -> bool {
        let extension: &str = self.get_output_file_ext();
        POSSIBLE_DOTS_OUTPUT.iter().any(|&i| i == extension)
    }

    pub fn get_legend(&self) -> bool {
        self.legend
    }

    pub fn get_dark_mode(&self) -> bool {
        self.dark_mode
    }
}
