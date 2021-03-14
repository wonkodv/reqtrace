use std::path::Path;

#[macro_use]
extern crate lazy_static;

mod common;
mod markdown;
mod tracer;

use common::*;



fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut error_counter: u32 = 0;

    let req_path = Path::new("REQUIREMENTS.md");
    let req_cfg = ArtefactConfig::Markdown(&req_path);
    let req_artefact = Artefact::open(&req_cfg);

    let dsg_path = Path::new("DESIGN.md");
    let dsg_cfg = ArtefactConfig::Markdown(&dsg_path);
    let dsg_artefact = Artefact::open(&dsg_cfg);



    for e in req_artefact.get_errors() {
        error_counter += 1;
        eprint!("Error {}\n", e);
    }
    for e in dsg_artefact.get_errors() {
        error_counter += 1;
        eprint!("Error {}\n", e);
    }

    for dsg in dsg_artefact.get_requirements() {

        let ids:Vec<String> = dsg.covers.iter().map(|r| r.id.to_owned()).collect();

        let covs:String = ids.join(", ");
        eprint!("{}: {}\n", dsg.id, covs);
    }


    for req in req_artefact.get_requirements() {
        let cov = dsg_artefact.get_requirements_that_cover(&req.id);

        let ids:Vec<String> = cov.iter().map(|r| r.id.to_owned()).collect();

        let covs:String = ids.join(", ");
        eprint!("{}: {}\n", req.id, covs);
    }


    if error_counter > 0 {
        eprint!("There were {} errors", error_counter);
        return Err("Error".into());
    } else {
        return Ok(());
    }
}
