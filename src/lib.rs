use core::fmt;

use std::path::{Path, PathBuf};


pub mod xpdf_info;
pub mod xpdf_text;
pub mod types;


use xpdf_info::PdfInfo;

use types::XpdfArgs;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PdfErrorKind {
    NoXpdfToolsDirectory,
    ToolNotAvailable,
    PdfInfoError,
    PdfToTextError,
    PdfToTextErrorNoOutput,
}
#[derive(Debug, Clone, PartialEq)]
pub struct PdfError {
    pub message: String,
    pub process_message: String,
    pub error_kind: PdfErrorKind,
}

#[allow(dead_code)]
impl PdfError {
    fn kind(&self) -> PdfErrorKind {
        self.error_kind
    }
}
impl fmt::Display for PdfError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
impl std::error::Error for PdfError {}

const VALID_TOOLS: &'static [&'static str] = &["pdfdetach", "pdffonts", "pdfimages", "pdfinfo", "pdftohtml", "pdftopng", "pdftoppm", "pdftops", "pdftotext"];

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

        match Self::pdf_check_tool(self, "pdfinfo") {
            Ok(_) => xpdf_info::pdf_info(pdf_file, &self),
            Err(e) => Err(e),
        }
    }
    pub fn pdf_text(&self, pdf_file: &Path) -> Result<Vec<u8>, PdfError> {
       
        match Self::pdf_check_tool(self, "pdfinfo") {
            Ok(_) =>  xpdf_text::pdf_to_binary(pdf_file, &self),
            Err(e) => Err(e),
        }
    }

    pub fn pdf_check_tool(&self, tool: &str) -> Result<bool, PdfError> {
        if self.available_tools.as_ref().unwrap().contains(&tool.to_string()) {
            Ok(true)
        } else {
            Err(Self::tool_not_available(tool))
        }
    }
    fn tool_not_available(tool: &str) -> PdfError {
        PdfError {
            message: format!("{} not found in tools folder", tool),
            process_message: "".into(),
            error_kind: PdfErrorKind::ToolNotAvailable,
        }
    }
}
#[derive(Debug)]
pub struct XpdfToolsBuilder {
    tools_folder: PathBuf,
    extra_args: Option<Vec<XpdfArgs>>,
    available_tools: Option<Vec<String>>,
}

impl XpdfToolsBuilder {

    fn new(tools_folder: PathBuf) -> Result<Self, PdfError> {

        if tools_folder.is_dir() {
           
            let dir = tools_folder.read_dir().unwrap();

            //prop check https://github.com/rust-lang/libs-team/issues/311 for a better way to do this..
            let valid_entries:Vec<_> =  dir
                .filter_map(|entry| entry.ok())
                .map(|entry| entry.path().file_stem().unwrap().to_owned())
                .filter_map(|stem| stem.into_string().ok())
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

    pub fn build(self) -> XpdfTools {
        XpdfTools { extra_args: self.extra_args, tools_folder: self.tools_folder, available_tools: self.available_tools}
    }
}

pub fn get_version() -> String {
    format!("XpdfTools version: {}",env!("CARGO_PKG_VERSION"))
}


#[test]
fn test_builder_errors() {
    let tools_result = XpdfTools::builder(PathBuf::from("./folder/not/there"));
    assert_eq!(tools_result.unwrap_err().error_kind, PdfErrorKind::NoXpdfToolsDirectory);

    let tools_result = XpdfTools::builder(PathBuf::from("./testData/binTester"));
    assert!(tools_result.unwrap().available_tools.unwrap().len() == 2);
    
}

#[test]
fn test_tool_available() {
    let tools_result = XpdfTools::builder(PathBuf::from("./testData/binTester")).unwrap().build();
    assert_eq!(tools_result.pdf_check_tool("pdfinfo"), Ok(true));
    assert_eq!(tools_result.pdf_check_tool("pdftotext"), Ok(true));
    assert_eq!(tools_result.pdf_check_tool("pdftoWindmill").unwrap_err().kind(), PdfErrorKind::ToolNotAvailable);
}

#[test]
fn test_arguments() {
    let tools_result = XpdfTools::builder(PathBuf::from("./testData/binTester")).unwrap()
        .extra_args(vec![XpdfArgs::RawDates, XpdfArgs::Metadata])
        .build();
    let result = tools_result.pdf_info(Path::new("./testData/pdfFile_01.pdf"));

    assert!(result.as_ref().unwrap().info_map.get("Metadata").is_some());
    //println!("{:?}", result.as_ref().unwrap().info_map);
    

    //assert!()
    
}