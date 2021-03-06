use std::path::Path;

#[macro_use]
extern crate lazy_static;

mod common;
mod markdown;

use common::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new("REQUIREMENTS.md");
    let a = markdown::MarkdownArtefact::new(path);

    let reqs = &a.parse()?;
    for r in reqs {
        eprint!("# {}: {}\n", r.id, r.title);
    }

    let path = Path::new("FORMATS.md");
    let a = markdown::MarkdownArtefact::new(path);

    let reqs = &a.parse()?;
    for r in reqs {
        eprint!("{}", r.to_markdown());
    }

    return Ok(());
}
