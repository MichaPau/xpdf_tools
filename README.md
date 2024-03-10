# XpfTools

> **Wrapper for Xpdf command line tools as a Rust library**

![Static Badge](https://img.shields.io/badge/version-0.1.0-blue)


GPLv3

## About

Wrapper for [Xpdf's Reader Command line tools](http://www.xpdfreader.com/download.html)

Tools included in this version:

pdfinfo
pdftotext

## Usage

You need the [Xpdf Command Line Tools](http://www.xpdfreader.com/download.html) to use this wrapper library.

```
use xpdf_tools::types::XpdfArgs;
use xpdf_tools::XpdfTools;

...

let tools = XpdfTools::builder(PathBuf::from("path/to/the/tools/folder")).unwrap()
        .extra_args(vec![XpdfArgs::RawDates, XpdfArgs::Metadata])
        .build();
    
match tools.pdf_info("path/to/the/pdf/file") {
    Ok(pdf_info) => {
        println!("{:#?}", pdf_info.info_map);
    },
    Err(e) => println!("{:?}", e),
}
```

Non english languages may need
```
.extra_args(vec![XpdfArgs::Encoding("UTF-8".into())])
```

