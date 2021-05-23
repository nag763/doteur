use std::fs;
use std::path::Path;

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
    filename: String,
    /// File content
    filecontent: String,
    /// Output file name
    output_filename: String,
    /// Restrictions to apply
    restrictions: Option<Restriction>,
}

impl Args {

    /// Returns a args object for the given filename
    ///
    /// # Arguments
    ///
    /// * `path_str` - The path of the input file
    pub fn new(path_str: String) -> Args {
        let filename : String = Path::new(path_str.as_str()).file_name().expect("Incorrect file name").to_str().unwrap().to_string();
        if Path::new(path_str.as_str()).is_dir() {
            panic!("Directories aren't supported");
        } else {
            Args {
                filename,
                filecontent: fs::read_to_string(path_str.as_str()).expect("Something went wrong while reading the file"),
                output_filename: String::from("output.dot"),
                restrictions: None,
            }
        }
    }


    /// Returns the filename without the non ascii digits and chars
    pub fn get_filename_without_specials(&self) -> String {
        self.filename.chars().filter(|c| c.is_ascii_alphanumeric() || c.is_ascii_whitespace()).collect::<String>()
    }

    /// Returns the file extension
    pub fn get_file_ext(&self) -> &str {
        std::path::Path::new(self.output_filename.as_str()).extension().unwrap_or_default().to_str().unwrap_or_default()
    }

    /// Check if the file extension is supported by the graphviz tool
    pub fn ext_supported(&self) -> bool {
        let file_ext : &str = self.get_file_ext();
        POSSIBLE_DOTS_OUTPUT.iter().any(|&i| i == file_ext)
    }

    /// Returns the file content
    pub fn get_filecontent(&self) -> &str {
        self.filecontent.as_str()
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

    /// Get restrictions
    pub fn get_restrictions(&self) -> Option<&Restriction>{
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

}
