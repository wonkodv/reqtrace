use std::path::Path;

#[macro_use]
extern crate lazy_static;

mod common;
mod parsers;
mod trace;

use common::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut error_counter: u32 = 0;

    let mut g = trace::Graph::new();

    {
        let path = Path::new("DESIGN.md");
        let cfg = ArtefactConfig::Markdown(&path);
        let artefact = Artefact::new("DSG", cfg);
        g.add_artefact(artefact);
    }
    {
        let path = Path::new("README.md");
        let cfg = ArtefactConfig::Markdown(&path);
        let artefact = Artefact::new("README", cfg);
        g.add_artefact(artefact);
    }
    {
        let path = Path::new("REQUIREMENTS.md");
        let cfg = ArtefactConfig::Markdown(&path);
        let artefact = Artefact::new("REQ", cfg);
        g.add_artefact(artefact);
    }
    {
        let path = Path::new("FORMATS.md");
        let cfg = ArtefactConfig::Markdown(&path);
        let artefact = Artefact::new("FORMAT", cfg);
        g.add_artefact(artefact);
    }

    g.add_edge("README", "REQ");
    g.add_edge("REQ", "DSG");
    g.add_edge("REQ", "FORMAT");

    let t = g.trace_shallow("REQ");


    eprintln!("# Derived");
    for r in t.derived {
        eprintln!("*  {}", r.id);
    }
    eprintln!("# Uncovered");
    for r in t.uncovered {
        eprintln!("*  {}", r.id);
    }
    eprintln!("# Covered");
    for (ur, lr, c) in t.covered {
        eprintln!("*  {} --> {}", ur.id, lr.id);
    }
    eprintln!("# Errors");
    for e in t.errors {
        eprintln!("*  {:?}", e);
        error_counter += 1;
    }


    if error_counter > 0 {
        return Err(format!("There were {} errors", error_counter).into());
    } else {
        return Ok(());
    }
}
