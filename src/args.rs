use mysql::{Opts, OptsBuilder};
use std::fs;
use std::path::{Path, PathBuf};

use super::{process_mysql_connection, process_sqlite_connection};
use crate::restriction::Restriction;

/// Possible dot output formats.
pub const POSSIBLE_DOTS_OUTPUT: [&str; 54] = [
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
    "webp",
    "xlib",
    "x11",
];

/// Cli Args, used to represent the options passed by the user to
/// the tool.
#[derive(Clone)]
pub struct Args {
    /// Filename
    filename: Option<String>,
    /// Connection options for database
    opts: Option<Opts>,
    /// Sqlite Path
    sqlite_path: Option<String>,
    /// File content
    filecontent: String,
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
        let mut filecontent: Vec<String> = vec![];
        for path in path_str.iter() {
            if Path::new(path).is_dir() {
                for subpath in fs::read_dir(path)? {
                    let file_path: &PathBuf = &subpath.unwrap().path();
                    // Ignore subdirs
                    if Path::new(file_path).is_file() {
                        filecontent.push(fs::read_to_string(file_path)?);
                    }
                }
            } else {
                filecontent.push(fs::read_to_string(path)?);
            }
        }
        Ok(Args {
            filename: Some(filename),
            filecontent: filecontent.join("\n"),
            output_filename: String::from("output.dot"),
            opts: None,
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
    pub fn new_from_url(url: &str) -> Result<Args, mysql::Error> {
        let opts: Opts = Opts::from_url(url)?;
        let mut args: Args = Args {
            filename: None,
            filecontent: String::new(),
            output_filename: String::from("output.dot"),
            opts: Some(opts),
            sqlite_path: None,
            restrictions: None,
            legend: false,
            dark_mode: false,
        };
        process_mysql_connection(&mut args)?;
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
            filecontent: String::new(),
            output_filename: String::from("output.dot"),
            opts: Some(Opts::from(opts_builder)),
            sqlite_path: None,
            restrictions: None,
            legend: false,
            dark_mode: false,
        };
        process_mysql_connection(&mut args)?;
        Ok(args)
    }

    /// Returns a args object for the given sqlite path
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the sqlite file
    pub fn new_from_sqlite(path: &str) -> Result<Args, rusqlite::Error> {
        let mut args: Args = Args {
            filename: None,
            filecontent: String::new(),
            output_filename: String::from("output.dot"),
            opts: None,
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
    pub fn get_filecontent(&self) -> &str {
        self.filecontent.as_str()
    }

    pub fn set_filecontent(&mut self, filecontent: String) {
        self.filecontent = filecontent;
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

    pub fn get_opts(&self) -> Option<&Opts> {
        self.opts.as_ref()
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
