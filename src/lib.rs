use core::fmt;

use std::path::{Path, PathBuf};


pub mod xpdf_info;
pub mod xpdf_text;
pub mod types;


use xpdf_info::PdfInfo;

use types::XpdfArgs;

#[derive(Debug, Clone)]
pub struct PdfError {
    pub message: String,
    pub process_message: String,
}

impl fmt::Display for PdfError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
impl std::error::Error for PdfError {}

#[derive(Debug)]
pub struct XpdfTools {
    tools_folder: PathBuf,
    extra_args: Option<Vec<XpdfArgs>>,

}

impl XpdfTools {
    pub fn builder(tools_folder:PathBuf) -> XpdfToolsBuilder {
        XpdfToolsBuilder::new(tools_folder)
    }

    pub fn pdf_info(&self, pdf_file: &Path) -> Result<PdfInfo, PdfError> {
        xpdf_info::pdf_info(pdf_file, &self)
    }

    pub fn pdf_text(&self, pdf_file: &Path) -> Result<Vec<u8>, PdfError> {
        xpdf_text::pdf_to_binary(pdf_file, &self)
    }
}
pub struct XpdfToolsBuilder {
    tools_folder: PathBuf,
    extra_args: Option<Vec<XpdfArgs>>,
}

impl XpdfToolsBuilder {
    fn new(tools_folder: PathBuf) -> Self {
        // #[cfg(target_pointer_width = "64")]
        // let folder_post = "bin64/";
        // #[cfg(target_pointer_width = "32")]
        // let folder_post = "bin32/";

        // let tools_folder:PathBuf = match std::env::consts::OS {
        //     "linux" => PathBuf::from("./tools/"),
        //     "windows" => PathBuf::from("./tools/xpdf-tools-win-4.05/".to_string()+folder_post),
        //     "macos" => PathBuf::from("./tools"),
        //     _ => PathBuf::new(),
        // };
        Self {
            tools_folder,
            extra_args: None,
        }
    }

    pub fn extra_args(mut self, extra_args: Vec<XpdfArgs>) -> Self {
        self.extra_args = Some(extra_args);
        self
    }

    pub fn tools_folder(mut self, tools_folder: &str) -> Self {
        self.tools_folder = PathBuf::from(tools_folder);
        self
    }

    pub fn build(self) -> XpdfTools {
        XpdfTools { extra_args: self.extra_args, tools_folder: self.tools_folder}
    }
}

pub fn get_version() -> String {
    format!("XpdfTools version: {}",env!("CARGO_PKG_VERSION"))
}

