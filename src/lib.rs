use core::fmt;

use std::path::{Path, PathBuf};


pub mod xpdf_info;
pub mod xpdf_text;
pub mod types;


use xpdf_info::PdfInfo;

use types::XpdfArgs;

#[derive(Debug, Clone)]
pub enum PdfErrorKind {
    NoXpdfToolsDirectory,
    PdfInfoError,
    PdfToTextError,
    PdfToTextErrorNoOutput,
}
#[derive(Debug, Clone)]
pub struct PdfError {
    pub message: String,
    pub process_message: String,
    pub error_kind: PdfErrorKind,
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
    available_tools: Option<Vec<String>>,

}

impl XpdfTools {
    pub fn builder(tools_folder:PathBuf) -> Result<XpdfToolsBuilder, PdfError> {
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
    available_tools: Option<Vec<String>>,
}

//const VALID_TOOLS: [&'static str; 9] = ["pdfdetach", "pdffonts", "pdfimages", "pdfinfo", "pdftohtml", "pdftopng", "pdftoppm", "pdftops", "pdftotext"];
const VALID_TOOLS: &'static [&'static str] = &["pdfdetach", "pdffonts", "pdfimages", "pdfinfo", "pdftohtml", "pdftopng", "pdftoppm", "pdftops", "pdftotext"];
impl XpdfToolsBuilder {
    fn new(tools_folder: PathBuf) -> Result<Self, PdfError> {

        if tools_folder.is_dir() {
            //let valid_tools = vec!["pdfdetach", "pdffonts", "pdfimages", "pdfinfo", "pdftohtml", "pdftopng", "pdftoppm", "pdftops", "pdftotext"];
            let dir = tools_folder.read_dir().unwrap();

            //prop check https://github.com/rust-lang/libs-team/issues/311 for a better way to do this..
            let valid_entries:Vec<_> =  dir
                .filter_map(|entry| entry.ok())
                .map(|entry| entry.path().file_stem().unwrap().to_owned())
                .filter_map(|stem| stem.into_string().ok())
                //.filter(|s| valid_tools.contains(&s.as_str()))
                .filter(|s| VALID_TOOLS.contains(&s.as_str()))
                .collect();
            
            Ok(Self {
                tools_folder,
                extra_args: None,
                available_tools: Some(valid_entries),
            })
        } else {
            Err(PdfError { 
                message: "Specified tools folder not found".into(), 
                process_message: "".into(), 
                error_kind: PdfErrorKind::NoXpdfToolsDirectory })
        }
        
    }

    pub fn extra_args(mut self, extra_args: Vec<XpdfArgs>) -> Self {
        self.extra_args = Some(extra_args);
        self
    }

    // pub fn tools_folder(mut self, tools_folder: &str) -> Self {
    //     self.tools_folder = PathBuf::from(tools_folder);
    //     self
    // }

    pub fn build(self) -> XpdfTools {
        XpdfTools { extra_args: self.extra_args, tools_folder: self.tools_folder, available_tools: self.available_tools}
    }
}

pub fn get_version() -> String {
    format!("XpdfTools version: {}",env!("CARGO_PKG_VERSION"))
}


#[test]
fn test_builder_errors() {

}
