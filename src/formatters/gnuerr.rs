use std::io;

use crate::models::{Error, TracedGraph};

pub fn errors<'r, W, R>(errors: R, w: &mut W) -> io::Result<()>
where
    W: io::Write,
    R: Iterator<Item = &'r Error>,
{
    for err in errors {
        match err {
            Error::Format(loc, err) => {
                writeln!(w, "{loc}: {err}")?;
            }
            Error::DuplicateRequirement(r1, r2) => {
                writeln!(
                    w,
                    concat!(
                        "{}: Duplicate Requirement: {}\n",
                        "{}: note: previously seen here",
                    ),
                    r2.location, r1.id, r1.location,
                )?;
            }
            Error::DuplicateAttribute(loc, attr, req) => {
                writeln!(w, "{loc}: {req} has duplicate Attribute: {attr}")?;
            }
            Error::Io(path, err) => {
                writeln!(w, "{}: IO Error: {}", path.display(), err,)?;
            }
            Error::UnusedRelation(_)
            | Error::ArtefactConfig(_)
            | Error::DuplicateArtefact(_)
            | Error::UnknownArtefact(_)
            | Error::EmptyGraph => {
                writeln!(w, "Error in config file: {err:?}")?;
            }

            Error::CoveredWithWrongTitle {
                upper,
                lower,
                wrong_title,
                location,
            } => {
                writeln!(
                    w,
                    concat!(
                        "{}: {} covered with wrong title \n",
                        "    expected: {}\n",
                        "    actual  : {}\n",
                        "{}: note: Defined here"
                    ),
                    location,
                    upper.id,
                    upper.title.as_ref().unwrap_or(&"<no title>".to_owned()),
                    wrong_title,
                    upper.location,
                )?;
            }
            Error::DependWithWrongTitle {
                upper,
                lower,
                wrong_title,
                location,
            } => {
                writeln!(
                    w,
                    concat!(
                        "{}: {} depend with wrong title:\n",
                        "    expected: {}\n",
                        "     actual : {}\n",
                        "{}: note: Defined here",
                    ),
                    location,
                    lower.id,
                    lower.title.as_ref().unwrap_or(&"<no title>".to_owned()),
                    wrong_title,
                    lower.location,
                )?;
            }
            Error::DependOnUnknownRequirement(req, depend, location) => {
                writeln!(
                    w,
                    "{}: {} Depends on unknown requirement {}",
                    location, req.id, depend
                )?;
            }
            Error::CoversUnknownRequirement(req, cover, location) => {
                writeln!(
                    w,
                    "{}: {} Covers unknown requirement {}",
                    location, req.id, cover
                )?;
            }
        }
    }

    Ok(())
}

pub fn tracing<W>(traced_graph: &TracedGraph, w: &mut W) -> io::Result<()>
where
    W: io::Write,
{
    writeln!(w, "Parsing Errors")?;
    errors(
        traced_graph
            .artefacts
            .values()
            .flat_map(|a| a.errors.iter()),
        w,
    )?;
    writeln!(w, "Tracing Errors")?;
    errors(traced_graph.errors.iter(), w)?;

    // TODO   writeln!(w, "Uncovered Requirement")?;
    // TODO
    // TODO   for rel in &traced_graph.relations {
    // TODO       for req in rel.uncovered {
    // TODO           let req = traced_graph.artefacts[&rel.upper].[req];
    // TODO           if let Some(title) = &req.title {
    // TODO               writeln!(w, "{}: {}: {}", req.location, req.id, title)?;
    // TODO           } else {
    // TODO               writeln!(w, "{}: {}", req.location, req.id)?;
    // TODO           }
    // TODO       }
    // TODO   }
    // TODO
    // TODO   writeln!(w, "Derived Requirement")?;
    // TODO   for req in tracing.derived() {
    // TODO       let req = req.requirement;
    // TODO       if let Some(title) = &req.title {
    // TODO           writeln!(w, "{}: {}: {}", req.location, req.id, title)?;
    // TODO       } else {
    // TODO           writeln!(w, "{}: {}", req.location, req.id)?;
    // TODO       }
    // TODO   }

    Ok(())
}
