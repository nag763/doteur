use std::fs;
use std::path::{Path, PathBuf};

use crate::restriction::Restriction;

#[cfg(feature = "mysql_addons")]
use crate::mysql_tools::process_mysql_connection;
#[cfg(feature = "mysql_addons")]
use mysql::{Opts, OptsBuilder};

#[cfg(feature = "sqlite_addons")]
use crate::sqlite_tools::process_sqlite_connection;

use clap::{App, Arg};

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

pub fn get_clap_args() -> App<'static> {
    let app : App = App::new("doteur")
        .version("0.4.1")
        .author("LABEYE Loïc <loic.labeye@pm.me>")
        .about("Parse a SQL configuration and convert it into a .dot file, render the output if Graphviz is installed")
        .after_help("Some functionnalities might not appear as they depend on which version this tool has been downloaded or built for.")
        .arg(
            Arg::new("input")
                .help("Name of the sql file or database location if an URL arg is passed, can also be a directory or several files")
                .required(false)
                .index(1)
                .multiple_values(true)
        ).arg(
            Arg::new("output")
                .help("output file name")
                .short('o')
                .long("output")
                .takes_value(true)
        );
    let trailing_args: Vec<Arg> = vec![
        Arg::new("include")
            .help("Filter to include only the given tables, accept simple regexs")
            .short('i')
            .long("include")
            .takes_value(true)
            .multiple_values(true)
            .conflicts_with("exclude"),
        Arg::new("exclude")
            .help("Filter to exclude the given tables, accept simple regexs")
            .short('x')
            .long("exclude")
            .takes_value(true)
            .multiple_values(true)
            .conflicts_with("include"),
        Arg::new("dark_mode")
            .help("Render in dark mode")
            .long("dark-mode")
            .takes_value(false),
        Arg::new("legend")
            .help("Includes hint about the relations type at the bottom of the outpout file")
            .long("legend")
            .takes_value(false),
    ];
    cfg_if! {
        if #[cfg(feature = "mysql_addons")] {
            app.args(
                vec![
                    Arg::new("url")
                        .help("Specificate that the input is an URL (i.e. mysql://usr:password@localhost:3306/database)")
                        .long("url")
                        .conflicts_with_all(&["sqlite", "interactive"]),
                    Arg::new("interactive")
                        .help("Starts an interactive dialog to connect to a remote database")
                        .long("it")
                        .conflicts_with_all(&["sqlite", "url"])
                ]
            ).args(trailing_args)
        } else if #[cfg(feature = "sqlite_addons")] {
            app.args(vec![
                Arg::new("sqlite")
                    .help("Specificate that the input is a sqlite3 database")
                    .long("sqlite")
                ]
            ).args(trailing_args)
        } else {
            app.args(trailing_args)
        }
    }
}

/// Cli Args, used to represent the options passed by the user to
/// the tool.
#[derive(Clone)]
pub struct Args {
    /// Filename
    filename: Option<String>,
    /// Sqlite Path
    sqlite_path: Option<String>,
    /// Data to transform
    data: String,
    /// Output file name
    output_filename: String,
    /// Restrictions to apply
    restrictions: Option<Restriction>,
    /// Set if the legend has to be included
    legend: bool,
    /// Set if the dark mode has to be activated
    dark_mode: bool,
}

impl Args {
    /// Returns a args object for the given filename
    ///
    /// # Arguments
    ///
    /// * `path_str` - The path of the input
    pub fn new_from_files(path_str: Vec<&str>) -> Result<Args, Box<dyn std::error::Error>> {
        let filename: String;
        if path_str.len() != 1 {
            filename = String::from("multifilearg");
        } else {
            filename = match Path::new(path_str[0]).file_name() {
                Some(v) => v.to_str().unwrap().to_string(),
                None => return Err("No file found for given input".into()),
            };
        }
        let mut data: Vec<String> = vec![];
        // Reads the filecontent of a directory, ignores subdirectories
        for path in path_str.iter() {
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
        Ok(Args {
            filename: Some(filename),
            data: data.join("\n"),
            output_filename: String::from("output.dot"),
            sqlite_path: None,
            restrictions: None,
            legend: false,
            dark_mode: false,
        })
    }

    /// Returns a args object for the given filename
    ///
    /// # Arguments
    ///
    /// * `url` - The path of the input
    #[cfg(feature = "mysql_addons")]
    pub fn new_from_url(url: &str) -> Result<Args, mysql::Error> {
        let opts: Opts = Opts::from_url(url)?;
        let mut args: Args = Args {
            filename: None,
            data: String::new(),
            output_filename: String::from("output.dot"),
            sqlite_path: None,
            restrictions: None,
            legend: false,
            dark_mode: false,
        };
        process_mysql_connection(&mut args, opts)?;
        Ok(args)
    }

    /// Returns a args object for the given parameters
    ///
    /// # Arguments
    ///
    /// * `db_url` - Database url or ip
    /// * `db_port` - Database remote port
    /// * `db_name` - Database remote schema name
    /// * `db_user` - Database remote user
    /// * `db_password` - Database remote user's password
    #[cfg(feature = "mysql_addons")]
    pub fn new_connect_with_params(
        db_url: String,
        db_port: u16,
        db_name: String,
        db_user: String,
        db_password: String,
    ) -> Result<Args, mysql::Error> {
        let opts_builder: OptsBuilder = OptsBuilder::new()
            .ip_or_hostname(Some(db_url))
            .tcp_port(db_port)
            .db_name(Some(db_name))
            .user(Some(db_user))
            .pass(Some(db_password));
        let mut args: Args = Args {
            filename: None,
            data: String::new(),
            output_filename: String::from("output.dot"),
            sqlite_path: None,
            restrictions: None,
            legend: false,
            dark_mode: false,
        };
        process_mysql_connection(&mut args, Opts::from(opts_builder))?;
        Ok(args)
    }

    /// Returns a args object for the given sqlite path
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the sqlite file
    #[cfg(feature = "sqlite_addons")]
    pub fn new_from_sqlite(path: &str) -> Result<Args, rusqlite::Error> {
        let mut args: Args = Args {
            filename: None,
            data: String::new(),
            output_filename: String::from("output.dot"),
            sqlite_path: Some(path.to_string()),
            restrictions: None,
            legend: false,
            dark_mode: false,
        };
        process_sqlite_connection(&mut args)?;
        Ok(args)
    }

    /// Returns the filename without the non ascii digits and chars
    pub fn get_filename_without_specials(&self) -> String {
        match &self.filename {
            Some(v) => v
                .chars()
                .filter(|c| c.is_ascii_alphanumeric() || c.is_ascii_whitespace())
                .collect::<String>(),
            None => "doteur".to_string(),
        }
    }

    /// Returns the file extension
    pub fn get_output_file_ext(&self) -> &str {
        std::path::Path::new(self.output_filename.as_str())
            .extension()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default()
    }

    /// Check if the file extension is supported by the graphviz tool
    ///
    /// # Arguments
    ///
    /// * `ext` - extension to verify
    pub fn ext_supported(ext: &str) -> bool {
        POSSIBLE_DOTS_OUTPUT.iter().any(|&i| i == ext)
    }

    /// Returns the file content
    pub fn get_data(&self) -> &str {
        self.data.as_str()
    }

    pub fn set_data(&mut self, filecontent: String) {
        self.data = filecontent;
    }

    /// Get the output file name
    pub fn get_output_filename(&self) -> &str {
        self.output_filename.as_str()
    }

    /// Sets the output filename
    ///
    /// # Arguments
    ///
    /// * `output_filename` - The name of the output file
    pub fn set_output_filename(&mut self, output_filename: String) {
        self.output_filename = output_filename;
    }

    /// Get restrictions
    pub fn get_restrictions(&self) -> Option<&Restriction> {
        self.restrictions.as_ref()
    }

    pub fn get_sqlite_path(&self) -> Option<&String> {
        self.sqlite_path.as_ref()
    }

    /// Sets the restrictions in inclusive way
    ///
    /// The inclusive arguments mean that only the given values with the -i cli arg will be
    /// rendered
    ///
    /// # Arguments
    ///
    /// * `inclusions` - The inclusions to set
    pub fn set_inclusions(&mut self, inclusions: Vec<String>) {
        self.restrictions = Some(Restriction::new_inclusion(inclusions));
    }

    /// Sets the restrictions in exclusive way
    ///
    /// The exclusive arguments mean that the given values with the -x cli arg won't be
    /// rendered
    ///
    /// # Arguments
    ///
    /// * `exclusions` - The exclusions to set
    pub fn set_exclusions(&mut self, exclusions: Vec<String>) {
        self.restrictions = Some(Restriction::new_exclusion(exclusions));
    }

    /// Returns the legend attribute
    pub fn get_legend(&self) -> bool {
        self.legend
    }

    /// Sets the legend attribute
    ///
    /// # Arguments
    ///
    /// * `legend` - The new value
    pub fn set_legend(&mut self, legend: bool) {
        self.legend = legend;
    }

    /// Returns the dark_mode attribute
    pub fn get_dark_mode(&self) -> bool {
        self.dark_mode
    }

    /// Set the dark mode attribute
    ///
    /// # Arguments
    ///
    /// * `dark_mode` - The new dark mode value
    pub fn set_dark_mode(&mut self, dark_mode: bool) {
        self.dark_mode = dark_mode;
    }
}
