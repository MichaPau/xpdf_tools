use std::{path::Path, process::Command};

use crate::PdfErrorKind;

use super::{PdfError, XpdfTools};

#[allow(dead_code)]
pub fn pdf_to_text(pdf_file: &Path, tools: &XpdfTools) -> Result<String, PdfError> {
    let binary_result = pdf_to_binary(pdf_file, tools);
    match binary_result {
        Ok(output) => Ok(String::from_utf8_lossy(&output).into_owned()),
        Err(e) => Err(e),
    }
}
// pub fn _pdf_to_text2(pdf_file: &str, tools: &XpdfTools) -> Result<String, PdfError> {
//     let output = Command::new("./tools/xpdf-tools-win-4.04/bin64/pdftotext")
//     .args(["-layout", pdf_file, "-"])
//     .output();

//     match output {
//         Ok(o) => {
//             let result = String::from_utf8_lossy(&o.stdout);
//             Ok(result.into_owned())
//         },
//         Err(_e) => {
           
//             Err(
//                 PdfError { 
//                     message: "pdf_to_text parse error".to_string(),
//                     process_message: _e.to_string(),
//                     error_kind: PdfErrorKind::PdfToTextError
//                 }
//             )
//         }
//     }
// }

#[allow(dead_code)]
pub fn pdf_to_binary(pdf_file: &Path, tools: &XpdfTools) -> Result<Vec<u8>, PdfError> {
    
    let mut cmd = tools.tools_folder.clone();
    cmd.push("pdftotext");
    
    let mut args = vec![];
    if let Some(extra) = &tools.extra_args {
        //args.append(&mut extra.clone());
       
        args.extend(extra.into_iter().filter(|xpdfarg| xpdfarg.is_valid_totext_arg()).map(|xpdfarg| xpdfarg.to_osstr()));
    }
    args.push(pdf_file.into());
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
                        message: "pdf_to_text parse error".to_string(),
                        process_message: String::from_utf8_lossy(&o.stderr).to_string(),
                        error_kind: PdfErrorKind::PdfToTextErrorNoOutput,
                    })
            } else {
                Ok(o.stdout.to_owned())
            }
            // println!("{:?}", o.stdout);
            // println!("{:?}", o.stderr);

            // Ok(o.stdout.to_owned())
        },
        Err(_e) => {
           
            Err(
                PdfError { 
                    message: "pdf_to_text parse error".to_string(),
                    process_message: _e.to_string(),
                    error_kind: PdfErrorKind::PdfToTextError
                }
            )
        }
    }
}