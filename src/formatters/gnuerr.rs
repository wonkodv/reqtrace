use std::io;

use crate::{errors::Error, graph::Graph, trace::Tracing};

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
            Error::ArtefactTypeOnlyAllowsOnePath(_, _)
            | Error::UnknownArtefactType(_)
            | Error::Config(_)
            | Error::DuplicateArtefact(_)
            | Error::UnknownArtefact(_)
            | Error::EmptyGraph
            | Error::UnknownJob(_)
            | Error::UnknownFork(_, _) => {
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
                    location.as_ref().unwrap_or(&lower.location),
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
                    location.as_ref().unwrap_or(&upper.location),
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
                    location.as_ref().unwrap_or(&req.location),
                    req.id,
                    depend
                )?;
            }
            Error::CoversUnknownRequirement(req, cover, location) => {
                writeln!(
                    w,
                    "{}: {} Covers unknown requirement {}",
                    location.as_ref().unwrap_or(&req.location),
                    req.id,
                    cover
                )?;
            }
        }
    }

    Ok(())
}

pub fn tracing<W>(tracing: &Tracing<'_>, graph: &Graph, w: &mut W) -> io::Result<()>
where
    W: io::Write,
{
    writeln!(w, "Parsing Errors")?;
    errors(graph.get_parsing_errors(), w)?;
    writeln!(w, "Tracing Errors")?;
    errors(tracing.errors().iter(), w)?;

    writeln!(w, "Uncovered Requirement")?;
    for req in tracing.uncovered() {
        let req = req.requirement;
        if let Some(title) = &req.title {
            writeln!(w, "{}: {}:{}", req.location, req.id, title)?;
        } else {
            writeln!(w, "{}: {}", req.location, req.id)?;
        }
    }

    writeln!(w, "Derived Requirement")?;
    for req in tracing.derived() {
        let req = req.requirement;
        if let Some(title) = &req.title {
            writeln!(w, "{}: {}:{}", req.location, req.id, title)?;
        } else {
            writeln!(w, "{}:{}", req.location, req.id)?;
        }
    }

    Ok(())
}
