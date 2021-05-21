use std::fs;

use super::restriction::{Restriction};

pub const POSSIBLE_DOTS_OUTPUT : [&str; 54] = ["bmp", "canon", "gv", "xdot", "xdot1.2", "xdot1.4",
                                            "cgimage", "cmap", "eps", "eps", "exr", "fig", "gd",
                                            "gd2" , "gif", "gtk", "ico", "imap", "cmapx", "imap_np",
                                            "cmapx_np", "ismap", "jp2", "jpg", "jpeg", "jpe", "json",
                                            "json0", "dot_json", "xdot_json", "pct", "pict","pdf",
                                            "pic", "plain", "plain-ext", "png", "pov", "ps2", "psd",
                                            "sgi", "svg", "svgz", "tga", "tif", "tiff", "tk", "vml",
                                            "vmlz", "vrml", "wbmp", "webp", "xlib", "x11"];


#[derive(Clone)]
pub struct Args {
    filename: String,
    filecontent: String,
    output_filename: String,
    restrictions: Option<Restriction>,
    first_depth : bool
}

impl Args {
    pub fn new(filename: String) -> Args {
        Args {
            filename: filename.clone(),
            filecontent: fs::read_to_string(filename.as_str()).expect("Something went wrong while reading the file"),
            output_filename: String::from("output.dot"),
            restrictions: None,
            first_depth: false
        }
    }

    pub fn get_filename_without_specials(&self) -> String {
        self.filename.chars().filter(|c| c.is_ascii_alphanumeric() || c.is_ascii_whitespace()).collect::<String>()
    }

    pub fn get_file_ext(&self) -> &str {
        std::path::Path::new(self.output_filename.as_str()).extension().unwrap_or_default().to_str().unwrap_or_default()
    }

    pub fn ext_supported(&self) -> bool {
        let file_ext : &str = self.get_file_ext();
        POSSIBLE_DOTS_OUTPUT.iter().any(|&i| i == file_ext)
    }

    pub fn get_filecontent(&self) -> &str {
        self.filecontent.as_str()
    }

    pub fn get_output_filename(&self) -> &str {
        self.output_filename.as_str()
    }

    pub fn set_output_filename(&mut self, output_filename : String) {
        self.output_filename = output_filename;
    }

    pub fn get_restrictions(&self) -> Option<&Restriction>{
        self.restrictions.as_ref()
    }

    pub fn set_inclusions(&mut self, inclusions : Vec<String>) {
        self.restrictions = Some(Restriction::new_inclusion(inclusions));
    }


    pub fn set_exclusions(&mut self, exclusions : Vec<String>) {
        self.restrictions = Some(Restriction::new_exclusion(exclusions));
    }

    pub fn get_first_depth(&self) -> bool {
        self.first_depth
    }

    pub fn set_first_depth(&mut self, first_depth : bool) {
        self.first_depth = first_depth
    }

}
