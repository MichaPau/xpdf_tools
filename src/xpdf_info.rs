use crate::types::{self, XpdfInfoMap};

use std::process::Command;
use std::path::Path;

use super::{PdfError, XpdfTools};

#[derive(Debug)]
pub struct PdfInfo {
    pub raw: String,
    pub info_map: XpdfInfoMap,
}

#[allow(dead_code)]
pub fn pdf_info(pdf_file: &Path, tools: &XpdfTools) -> Result<PdfInfo, PdfError> {
    let mut cmd = tools.tools_folder.clone();
    cmd.push("pdfinfo");

    let mut args = vec![];
    if let Some(extra) = &tools.extra_args {
        args.extend(extra.into_iter().map(|xpdfarg| xpdfarg.to_str()));
    }
    args.push(pdf_file.display().to_string());
    
    let output = Command::new(cmd)
    .args(&args)
    .output();

    match output {
        Ok(o) => {
            
            let mut map = types::XpdfInfoMap::create();
            let result = String::from_utf8_lossy(&o.stdout);

            let line_re = regex::Regex::new(r"(?<label>[a-zA-Z ]+):(?<value>.+)").unwrap();
            
            for line in result.lines() {
                match line_re.captures(line) {
                    Some(caps) => {
                        
                        let label = caps.name("label").unwrap().as_str();
                        
                        if map.contains_key(label) {
                            let value = caps.name("value").unwrap().as_str();
                            map.entry(label.into()).and_modify(|e| *e = Some(value.trim().into()));
                        } 
                    },
                    None => (),
                };
                
            }
           
            if args.iter().any(|e| e == "-meta") {
                
                let meta_re = regex::Regex::new(r"(?s)<\?xpacket begin=.*<?xpacket end=.*>").unwrap();
                if let Some(meta_match) = meta_re.find(result.as_ref()) {
                    let normalize_white = meta_match.as_str().lines().filter_map(|l| {
      
                        if l.trim_end() != "" {
                            Some(l)
                        } else {
                            None
                        }
                    }).collect::<Vec<_>>().join("\n");
                    map.entry("Metadata".into()).and_modify(|e| *e = Some(normalize_white));
                } 
                
            }
            let pdf_info = PdfInfo {
                raw: result.to_string(),
                info_map: map,
            };

            Ok(pdf_info)
        },
        Err(_e) => {
           
            Err(
                PdfError { 
                    message: "pdf_info error".to_string(),
                    process_message: _e.to_string(),
                }
            )
        }
    }
}