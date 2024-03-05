use core::fmt;

use std::ops::{Deref, DerefMut};
use std::collections::BTreeMap;
use std::path::PathBuf;





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

        write!(f, "end")
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
#[derive(Debug, Clone)]
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
    // valid for xpdfinfo
    Simple,
    // valid for xpdfinfo
    Simple2,
    // valid for xpdfinfo
    Table,
    // valid for xpdfinfo
    Lineprinter,
    // valid for xpdfinfo
    Raw,
    // valid for xpdfinfo
    Fixed(usize),
    // valid for xpdfinfo
    Linespacing(usize),
    // valid for xpdfinfo
    Clip,
    // valid for xpdfinfo
    NoDiag,
    // valid for xpdfinfo
    Eol(String),
    // valid for xpdfinfo
    NoPgBrk,
    // valid for xpdfinfo
    Bom,
    // valid for xpdfinfo
    MarginLeft(usize),
    // valid for xpdfinfo
    MarginRight(usize),
    // valid for xpdfinfo
    MarginTop(usize),
    // valid for xpdfinfo
    MarginBottom(usize),
    // valid for xpdfinfo
    Verbose,
    // valid for xpdfinfo
    Quit,
    // valid for xpdfinfo
    Listencodings
}

impl fmt::Display for XpdfArgs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_str())
    }
}
impl XpdfArgs {
    
    pub fn to_str(&self) -> String {
        match self {
            XpdfArgs::FirstPage(number) => format!("-f {}", number),
            XpdfArgs::LastPage(number) => format!("-f {}", number),
            XpdfArgs::Box => "-box".to_string(),
            XpdfArgs::RawDates => "-rawdates".to_string(),
            XpdfArgs::Metadata => "-meta".to_string(),
            XpdfArgs::Custom => "-custom".to_string(),
            XpdfArgs::Encoding(encoding_name) => format!("-enc {}", encoding_name),
            XpdfArgs::OwnerPassword(pwd) => format!("-opw {}", pwd),
            XpdfArgs::UserPassword(pwd) => format!("-upw {}", pwd),
            XpdfArgs::Config(config_file) => format!("-cfg {}", config_file.display()),
            XpdfArgs::Version => "-v".to_string(),
            XpdfArgs::Layout => "-layout".to_string(),
            XpdfArgs::Simple => "-simple".to_string(),
            XpdfArgs::Simple2 => "-simple2".to_string(),
            XpdfArgs::Table => "-table".to_string(),
            XpdfArgs::Lineprinter => "-lineprinter".to_string(),
            XpdfArgs::Raw => "-raw".to_string(),
            XpdfArgs::Fixed(number) => format!("-fixed {}", number),
            XpdfArgs::Linespacing(number) => format!("-linespacing {}", number),
            XpdfArgs::Clip => "-clip".to_string(),
            XpdfArgs::NoDiag => "-nodiag".to_string(),
            XpdfArgs::Eol(end_of_line) => format!("-enc {}", end_of_line),
            XpdfArgs::NoPgBrk => "-nopgbrk".to_string(),
            XpdfArgs::Bom => "-bom".to_string(),
            XpdfArgs::MarginLeft(number) => format!("-marginl {}", number),
            XpdfArgs::MarginRight(number) => format!("-marginr {}", number),
            XpdfArgs::MarginTop(number) => format!("-margint {}", number),
            XpdfArgs::MarginBottom(number) => format!("-marginb {}", number),
            XpdfArgs::Verbose => "-verbose".to_string(),
            XpdfArgs::Quit => "-q".to_string(),
            XpdfArgs::Listencodings => "-listencodings".to_string(),
            
        }
    }
}

// pub trait Join<Separator> {
//     type Output;

//     // Required method
//     fn join(slice: &Self, sep: Separator) -> Self::Output;
// }

// #[cfg(not(no_global_oom_handling))]
// #[unstable(feature = "slice_concat_ext", issue = "27747")]
// impl<S: Borrow<str>> Join<&str> for [S] {
//     type Output = String;

//     fn join(slice: &Self, sep: &str) -> String {
//         unsafe { String::from_utf8_unchecked(join_generic_copy(slice, sep.as_bytes())) }
//     }
// }
// pub trait Join<Separator> {
//     type Output;
//     fn join(slice: &Self, sep: Separator) -> Self::Output;
// }


// impl Join<&str> for XpdfArgs {
//     type Output = String;

//     fn join(slice: &Self, sep: &str) -> String {
//         // "test".to_string()
//         format!("{}{}", slice.to_str(), sep)
//     }
// }


// #[non_exhaustive]
// pub struct XpdfArgs; 

// impl XpdfArgs {
//     pub const RAW_DATES: XpdfArg = XpdfArg::String("-rawdates");
//     pub const METADATA: XpdfArg = XpdfArg::String("-meta");

// }

#[ignore]
#[test]
fn test_xpdf_map() {
    let mut map = XpdfInfoMap::create();
    map.insert("newKey".into(), Option::Some("newValue".into()));

    assert!(map.contains_key("Title"));
    assert!(map.contains_key("newKey"));
    assert!(!map.contains_key("some random key"));

}
