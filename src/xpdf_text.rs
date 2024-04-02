use std::{path::Path, process::Command};

use crate::PdfErrorKind;

use super::{PdfError, XpdfTools, args_parser};

#[allow(dead_code)]
pub fn pdf_to_text(pdf_file: &Path, tools: &XpdfTools) -> Result<String, PdfError> {
    let binary_result = pdf_to_binary(pdf_file, tools);
    match binary_result {
        Ok(output) => Ok(String::from_utf8_lossy(&output).into_owned()),
        Err(e) => Err(e),
    }
}

#[allow(dead_code)]
pub fn pdf_to_binary(pdf_file: &Path, tools: &XpdfTools) -> Result<Vec<u8>, PdfError> {
    
    let mut cmd = tools.tools_folder.clone();
    cmd.push("pdftotext");
    
    let mut args = vec![];
    if let Some(extra) = &tools.extra_args {
        args.extend(args_parser(&extra, "pdftotext"));
    }
    args.push(String::from(pdf_file.to_str().unwrap()));
    args.push("-".into());
    
    let output = Command::new(cmd)
    .args(&args)
    .output();

    match output {
        Ok(o) => {
            //let result = String::from_utf8_lossy(&o.stdout);
            if o.stdout.is_empty() {
                Err(
                    PdfError { 
                        //message: "pdf_to_text parse error".to_string(),
                        message: format!("pdf_to_text parse error: {:?}", pdf_file),
                        process_message: String::from_utf8_lossy(&o.stderr).to_string(),
                        error_kind: PdfErrorKind::PdfToTextErrorNoOutput,
                    })
            } else {
                Ok(o.stdout.to_owned())
            }
        },
        Err(_e) => {
           
            Err(
                PdfError { 
                    //message: "pdf_to_text parse error".to_string(),
                    message: format!("pdf_to_text parse error: {:?}", pdf_file),
                    process_message: _e.to_string(),
                    error_kind: PdfErrorKind::PdfToTextError
                }
            )
        }
    }
}