use std::{fs, path::Path};

#[macro_use]
extern crate lazy_static;

mod common;
mod formatters;
mod parsers;
mod trace;

use common::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut error_counter: u32 = 0;

    let mut g = trace::Graph::new();

    {
        let path = Path::new("doc/requirements/DESIGN.md");
        let cfg = ArtefactConfig::Markdown(&path);
        let artefact = Artefact::new("DSG", cfg);
        g.add_artefact(artefact);
    }
    {
        let path = Path::new("doc/requirements/README.md");
        let cfg = ArtefactConfig::Markdown(&path);
        let artefact = Artefact::new("README", cfg);
        g.add_artefact(artefact);
    }
    {
        let path = Path::new("doc/requirements/REQUIREMENTS.md");
        let cfg = ArtefactConfig::Markdown(&path);
        let artefact = Artefact::new("REQ", cfg);
        g.add_artefact(artefact);
    }
    {
        let path = Path::new("doc/requirements/FORMATS.md");
        let cfg = ArtefactConfig::Markdown(&path);
        let artefact = Artefact::new("FORMAT", cfg);
        g.add_artefact(artefact);
    }

    g.add_edge_group("README", &["REQ"])
        .map_err(|e| e.to_string())?;
    g.add_edge_group("REQ", &["DSG", "FORMAT"])?;

    {
        let r = g.get_all_reqs();
        let mut file = fs::File::create(".req_tags")?;
        formatters::tags::requirements_ctags(r.as_slice(), &mut file);
    }

    let t = g.trace_edge("REQ", "DSG");
    let tf = g.trace_edge("REQ", "FORMAT");
    let t = t.add(tf).unwrap();

    eprintln!("# Derived");
    for (id, _r) in t.derived {
        eprintln!("*  {}", id);
    }
    eprintln!("# Uncovered");
    for (id, _r) in t.uncovered {
        eprintln!("*  {}", id);
    }
    eprintln!("# Covered");
    for cov in t.covered {
        eprintln!("*  {} --> {}", cov.upper.id, cov.lower.id);
    }
    eprintln!("# Errors");
    for e in t.errors {
        eprintln!("*  {}", e);
        error_counter += 1;
    }

    if error_counter > 0 {
        return Err(format!("There were {} errors", error_counter).into());
    } else {
        return Ok(());
    }
}
