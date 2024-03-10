use core::fmt;

use std::ffi::OsString;
use std::ops::{Deref, DerefMut};
use std::collections::BTreeMap;
use std::path::PathBuf;




#[derive(PartialEq)]
pub struct XpdfInfoMap(pub BTreeMap<String, Option<String>>);

impl Deref for XpdfInfoMap {
    type Target = BTreeMap<String, Option<String>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for XpdfInfoMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl fmt::Debug for XpdfInfoMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
       
        for (key, value) in &self.0 {
            let value = value.as_deref().unwrap_or("None".into());
            writeln!(f, "{key} - {value}")?;
        }

        write!(f, "")
    }
}

impl XpdfInfoMap {
    pub fn create() -> Self {
        XpdfInfoMap(
            BTreeMap::from([
                ("Title".into(), Option::None),
                ("Subject".into(), Option::None),
                ("Keywords".into(), Option::None),
                ("Author".into(), Option::None),
                ("Creator".into(), Option::None),
                ("Producer".into(), Option::None),
                ("CreationDate".into(), Option::None),
                ("ModDate".into(), Option::None),
                ("Tagged".into(), Option::None),
                ("Form".into(), Option::None),
                ("Pages".into(), Option::None),
                ("Encrypted".into(), Option::None),
                //("Print and Copy Permissions".into(), Option::None),
                ("Page size".into(), Option::None),
                ("File size".into(), Option::None),
                ("Optimized".into(), Option::None),
                //("Uses javascript".into(), Option::None),
                ("PDF version".into(), Option::None),
                ("Metadata".into(), Option::None),
            ])
        )
    }

}


/// Enumerates all possible arguments for the XpdfTools process arguments
#[derive(Debug, Clone, PartialEq)]
pub enum XpdfArgs {
    /// valid for xpdfinfo, xpdftext; Specifies the first page to examine.
    FirstPage(usize), 
    /// valid for xpdfinfo, xpdftext; Specifies the last page to examine.
    LastPage(usize),
    /// valid for xpdfinfo
    Box,
    // valid for xpdfinfo
    Metadata,
    // valid for xpdfinfo
    RawDates,
    // valid for xpdfinfo
    Custom,
    // valid for xpdfinfo, xpdftext
    Encoding(String),
    // valid for xpdfinfo, xpdftext
    OwnerPassword(String),
    // valid for xpdfinfo, xpdftext
    UserPassword(String),
    // valid for xpdfinfo, xpdftext
    Config(PathBuf),
    // valid for xpdfinfo, xpdtext
    Version,
    // valid for xpdftext
    Layout,
    // valid for xpdftext
    Simple,
    // valid for xpdftext
    Simple2,
    // valid for xpdftext
    Table,
    // valid for xpdftext
    Lineprinter,
    // valid for xpdftext
    Raw,
    // valid for xpdftext
    Fixed(usize),
    // valid for xpdftext
    Linespacing(usize),
    // valid for xpdftext
    Clip,
    // valid for xpdftext
    NoDiag,
    // valid for xpdftext
    Eol(String),
    // valid for xpdftext
    NoPgBrk,
    // valid for xpdftext
    Bom,
    // valid for xpdftext
    MarginLeft(usize),
    // valid for xpdftext
    MarginRight(usize),
    // valid for xpdftext
    MarginTop(usize),
    // valid for xpdftext
    MarginBottom(usize),
    // valid for xpdftext
    Verbose,
    // valid for xpdftext
    Quit,
    // valid for xpdftext
    Listencodings
}

impl fmt::Display for XpdfArgs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_osstr().to_string_lossy())
    }
}

//TODO
//this is so verbose - find a better way
impl XpdfArgs {
    
    pub fn to_str(&self) -> String {
        self.to_osstr().to_str().unwrap().to_owned()
    }
    pub fn to_osstr(&self) -> OsString {
        match self {
            XpdfArgs::FirstPage(number) => format!("-f {}", number).into(),
            XpdfArgs::LastPage(number) => format!("-f {}", number).into(),
            XpdfArgs::Box => OsString::from("-box"),//.to_string(),
            XpdfArgs::RawDates => OsString::from("-rawdates"),
            XpdfArgs::Metadata => OsString::from("-meta"),
            XpdfArgs::Custom => OsString::from("-custom"),
            XpdfArgs::Encoding(encoding_name) => format!("-enc {}", encoding_name).into(),
            XpdfArgs::OwnerPassword(pwd) => format!("-opw {}", pwd).into(),
            XpdfArgs::UserPassword(pwd) => format!("-upw {}", pwd).into(),
            XpdfArgs::Config(config_file) => format!("-cfg {}", config_file.display()).into(),
            XpdfArgs::Version => OsString::from("-v"),
            XpdfArgs::Layout => OsString::from("-layout"),
            XpdfArgs::Simple => OsString::from("-simple"),
            XpdfArgs::Simple2 => OsString::from("-simple2"),
            XpdfArgs::Table => OsString::from("-table"),
            XpdfArgs::Lineprinter => OsString::from("-lineprinter"),
            XpdfArgs::Raw => OsString::from("-raw"),
            XpdfArgs::Fixed(number) => format!("-fixed {}", number).into(),
            XpdfArgs::Linespacing(number) => format!("-linespacing {}", number).into(),
            XpdfArgs::Clip => OsString::from("-clip"),
            XpdfArgs::NoDiag => OsString::from("-nodiag"),
            XpdfArgs::Eol(end_of_line) => format!("-enc {}", end_of_line).into(),
            XpdfArgs::NoPgBrk => OsString::from("-nopgbrk"),
            XpdfArgs::Bom => OsString::from("-bom"),
            XpdfArgs::MarginLeft(number) => format!("-marginl {}", number).into(),
            XpdfArgs::MarginRight(number) => format!("-marginr {}", number).into(),
            XpdfArgs::MarginTop(number) => format!("-margint {}", number).into(),
            XpdfArgs::MarginBottom(number) => format!("-marginb {}", number).into(),
            XpdfArgs::Verbose => OsString::from("-verbos"),
            XpdfArgs::Quit => OsString::from("-q"),
            XpdfArgs::Listencodings => OsString::from("-listencodings"),
            //_ => OsString::new(),
        }
    }

    pub fn is_valid_for(&self, tool: &str) -> bool {
        match tool {
            "pdfinfo" => self.is_valid_info_arg(),
            "pdftotext" => self.is_valid_totext_arg(),
            _ => false,
        }
    }
    pub fn is_valid_info_arg(&self) -> bool {
        match self {
            XpdfArgs::FirstPage(_) | XpdfArgs::LastPage(_) | 
            XpdfArgs::Box | XpdfArgs::Metadata | XpdfArgs::RawDates | XpdfArgs::Custom | 
            //mainly shared
            XpdfArgs::Encoding(_) | XpdfArgs:: OwnerPassword(_) | XpdfArgs::UserPassword(_) | XpdfArgs::Config(_) => true,

            _ => false,
        }
    }

    pub fn is_valid_totext_arg(&self) -> bool {
        match self {
            XpdfArgs::FirstPage(_) | XpdfArgs::LastPage(_) | 
            XpdfArgs::Layout | XpdfArgs::Simple | XpdfArgs::Simple2 | XpdfArgs::Table | 
            XpdfArgs::Lineprinter | XpdfArgs::Raw | XpdfArgs::Fixed(_) | XpdfArgs::Linespacing(_) | 
            XpdfArgs::Clip | XpdfArgs::NoDiag | XpdfArgs::Eol(_) | XpdfArgs::NoPgBrk | XpdfArgs::Bom | 
            XpdfArgs::MarginLeft(_) | XpdfArgs::MarginRight(_) | XpdfArgs::MarginBottom(_) | XpdfArgs::MarginTop(_) | 
            XpdfArgs::Verbose | XpdfArgs::Quit | XpdfArgs::Listencodings |
            //mainly shared
            XpdfArgs::Encoding(_) | XpdfArgs:: OwnerPassword(_) | XpdfArgs::UserPassword(_) | XpdfArgs::Config(_) => true,

            _ => false,
        }
    }
    
}

#[ignore]
#[test]
fn test_xpdf_map() {
    let mut map = XpdfInfoMap::create();
    map.insert("newKey".into(), Option::Some("newValue".into()));

    assert!(map.contains_key("Title"));
    assert!(map.contains_key("newKey"));
    assert!(!map.contains_key("some random key"));

}

#[test]
fn test_arguments() {
    let args = vec![XpdfArgs::FirstPage(1), XpdfArgs::Metadata, XpdfArgs::MarginBottom(10)];
    let info_args:Vec<_> = args.iter().filter(|&arg| arg.is_valid_info_arg()).collect();
    let text_args:Vec<_> = args.iter().filter(|&arg| arg.is_valid_totext_arg()).collect();

    assert_eq!(info_args, vec![&XpdfArgs::FirstPage(1), &XpdfArgs::Metadata]);
    assert_eq!(text_args, vec![&XpdfArgs::FirstPage(1), &XpdfArgs::MarginBottom(10)]);
    
    
}
