use std::fs;
use std::path::{Path, PathBuf};
use mysql::{Opts, UrlError};

use super::restriction::{Restriction};

/// Possible dot output formats.
pub const POSSIBLE_DOTS_OUTPUT : [&str; 54] = ["bmp", "canon", "gv", "xdot", "xdot1.2", "xdot1.4",
                                            "cgimage", "cmap", "eps", "eps", "exr", "fig", "gd",
                                            "gd2" , "gif", "gtk", "ico", "imap", "cmapx", "imap_np",
                                            "cmapx_np", "ismap", "jp2", "jpg", "jpeg", "jpe", "json",
                                            "json0", "dot_json", "xdot_json", "pct", "pict","pdf",
                                            "pic", "plain", "plain-ext", "png", "pov", "ps2", "psd",
                                            "sgi", "svg", "svgz", "tga", "tif", "tiff", "tk", "vml",
                                            "vmlz", "vrml", "wbmp", "webp", "xlib", "x11"];

/// Cli Args, used to represent the options passed by the user to
/// the tool.
#[derive(Clone)]
pub struct Args {
    /// Filename
    filename: Option<String>,
    /// Connection options for database
    opts : Option<Opts>,
    /// File content
    filecontent: String,
    /// Output file name
    output_filename: String,
    /// Restrictions to apply
    restrictions: Option<Restriction>,
    /// Set if the legend has to be included
    legend: bool,
    /// Set if the dark mode has to be activated
    dark_mode: bool
}

impl Args {

    /// Returns a args object for the given filename
    ///
    /// # Arguments
    ///
    /// * `path_str` - The path of the input
    pub fn new_from_files(path_str: Vec<&str>) -> Args {
        let filename : String;
        if path_str.len() != 1 {
            filename = String::from("multifilearg");
        } else {
            filename = Path::new(path_str.first().unwrap()).file_name().expect("Incorrect file name").to_str().unwrap().to_string();
        }
        Args {
            filename : Some(filename),
            filecontent: path_str.iter().map(|path| {
                if Path::new(path).is_dir() {
                    fs::read_dir(path).expect("Directory can't be read").map(|file|
                        {
                            let file_path : &PathBuf = &file.unwrap().path();
                            if Path::new(file_path).is_file() {
                                fs::read_to_string(file_path).unwrap_or_else(|_| panic!("Something went wrong while reading the file : {}", file_path.as_path().to_str().unwrap_or("**ISSUE**")))
                            } else {
                                String::new()
                            }
                        }).collect::<Vec<String>>().join("\n")
                } else {
                    fs::read_to_string(path).unwrap_or_else(|_| panic!("Something went wrong while reading the file : {}", path))
                }
            }).collect::<Vec<String>>().join("\n"),
            output_filename: String::from("output.dot"),
            opts : None,
            restrictions: None,
            legend: false,
            dark_mode: false
        }
    }


    /// Returns a args object for the given filename
    ///
    /// # Arguments
    ///
    /// * `url` - The path of the input
    pub fn new_from_url(url: &str) -> Result<Args, UrlError> {
        let opts : Opts;
        match Opts::from_url(url){
            Ok(v) => opts = v,
            Err(e) => { return Err(e); }
        }
        Ok(
            Args {
                filename : None,
                filecontent: String::new(),
                output_filename: String::from("output.dot"),
                opts: Some(opts),
                restrictions: None,
                legend: false,
                dark_mode: false
            }
        )
    }


    /// Returns the filename without the non ascii digits and chars
    pub fn get_filename_without_specials(&self) -> String {
        match &self.filename {
            Some(v) => v.chars().filter(|c| c.is_ascii_alphanumeric() || c.is_ascii_whitespace()).collect::<String>(),
            None => "doteur".to_string()
        }
    }

    /// Returns the file extension
    pub fn get_output_file_ext(&self) -> &str {
        std::path::Path::new(self.output_filename.as_str()).extension().unwrap_or_default().to_str().unwrap_or_default()
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
    pub fn set_output_filename(&mut self, output_filename : String) {
        self.output_filename = output_filename;
    }

    pub fn get_opts(&self) -> Option<&Opts> {
        self.opts.as_ref()
    }

    /// Get restrictions
    pub fn get_restrictions(&self) -> Option<&Restriction> {
        self.restrictions.as_ref()
    }

    /// Sets the restrictions in inclusive way
    ///
    /// The inclusive arguments mean that only the given values with the -i cli arg will be
    /// rendered
    ///
    /// # Arguments
    ///
    /// * `inclusions` - The inclusions to set
    pub fn set_inclusions(&mut self, inclusions : Vec<String>) {
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
    pub fn set_exclusions(&mut self, exclusions : Vec<String>) {
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
    pub fn set_dark_mode(&mut self, dark_mode: bool){
        self.dark_mode = dark_mode;
    }

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_file_ext() {
        assert_eq!({
            let args = Args::new_from_files(vec!["./samples/samplefile3.sql"]);
            args.clone().get_output_file_ext()
        }, "dot", "default value");

        assert_eq!({
            let mut args = Args::new_from_files(vec!["./samples/samplefile3.sql", "./samples/samplefile1.sql"]);
            args.set_output_filename("hello.png".to_string());
            args.clone().get_output_file_ext()
        }, "png", "set value with multifile");

        assert_eq!({
            let mut args = Args::new_from_files(vec!["./samples"]);
            args.set_output_filename("./path/to/file/hello.png".to_string());
            args.clone().get_output_file_ext()
        }, "png", "set value");
    }

    #[test]
    fn test_file_in_list() {
        assert!(POSSIBLE_DOTS_OUTPUT.iter().all(|e| Args::ext_supported(e)), "normal use cases");
        assert!(!Args::ext_supported("dot"), "normal use case, we don't handle the dot files to the graphviz tool");
        assert!(!Args::ext_supported("file.png"), "normal use case, only the extension should be given");
    }

}
