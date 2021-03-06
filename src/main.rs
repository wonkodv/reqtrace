use std::path::Path;

#[macro_use]
extern crate lazy_static;

mod common;
mod markdown;

use common::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut error_counter: u32 = 0;
    for path in vec!["REQUIREMENTS.md", "FORMATS.md"] {
        eprint!("parsing {}\n", path);
        let path = Path::new(path);
        let a = markdown::MarkdownArtefact::new(path);
        let (reqs, errors) = a.parse();
        for e in errors {
            error_counter += 1;
            eprint!("Error {}", e);
        }
        for r in reqs {
            eprint!("{}", r.to_markdown());
        }
    }
    if error_counter > 0 {
        eprint!("There were {} errors", error_counter);
        return Err("Error".into());
    } else {
        return Ok(());
    }
}
