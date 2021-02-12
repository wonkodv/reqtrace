use std::path::Path;

#[macro_use]
extern crate lazy_static;

mod common;
mod markdown;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new("REQUIREMENTS.md");
    let a = markdown::MarkdownArtefact::new(path);

    let reqs = a.parse()?;
    for r in reqs {
        eprint!("# {}: {}\n{}", r.id, r.title, r.description);
    }

    return Ok(());
}

