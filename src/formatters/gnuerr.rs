use std::io;

use crate::models::Error;
use crate::models::TracedGraph;

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
            | Error::Config(_)
            | Error::DuplicateArtefact(_)
            | Error::UnknownArtefact(_)
            | Error::EmptyGraph => {
                writeln!(w, "Error in config file: {err:?}")?;
            }

            Error::CoveredWithWrongTitle {
                upper,
                wrong_title,
                location,
                ..
            } => {
                writeln!(
                    w,
                    concat!(
                        "{}: {} covered with wrong title. Expected {:?}, actual {:?}\n",
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
                lower,
                wrong_title,
                location,
                ..
            } => {
                writeln!(
                    w,
                    concat!(
                        "{}: {} depended with wrong title. Expected {:?}, actual {:?}\n",
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
            Error::ReferencedWithoutTitle {
                referenced,
                location,
            } => {
                writeln!(
                    w,
                    "{}: should reference {} with title {:?}",
                    location,
                    referenced.id,
                    referenced
                        .title
                        .as_ref()
                        .unwrap_or(&"<no title>".to_owned())
                )?;
            }
            Error::RequirementWithoutTitle(req) => {
                writeln!(
                    w,
                    "{}: requirement {} without title in artefact that requires referencing with title",
                    req.location,
                    req.id,
                )?;
            }
        }
    }

    Ok(())
}

pub fn tracing(traced_graph: &TracedGraph, w: &mut impl io::Write) -> io::Result<()> {
    writeln!(w, "# Parsing Errors")?;
    errors(
        traced_graph
            .artefacts
            .values()
            .flat_map(|a| a.errors.iter()),
        w,
    )?;
    writeln!(w, "# Tracing Errors")?;
    errors(traced_graph.errors.iter(), w)?;

    writeln!(w, "# Uncovered Requirement")?;

    for rel in &traced_graph.traced_relations {
        writeln!(w, "## {}", rel)?;

        for req in &rel.uncovered {
            let req = &traced_graph.artefacts[&rel.relation.upper].requirements[req];
            writeln!(w, "{}: {} Uncovered", req.location, req.id)?;
        }
    }

    writeln!(w, "Derived Requirement")?;
    for (art, derived) in &traced_graph.derived {
        let artefact = &traced_graph.artefacts[art];
        writeln!(w, "## {}", art)?;
        for req_id in derived {
            let req = &artefact.requirements[req_id];
            writeln!(w, "{}: {} Derived", req.location, req.id)?;
        }
    }

    Ok(())
}
