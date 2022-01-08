use super::super::common::*;
use std::io;

use crate::{errors::Error, graph::Graph, trace::Tracing};
use Error::*;

pub fn errors<'r, W, R>(errors: R, w: &mut W) -> io::Result<()>
where
    W: io::Write,
    R: Iterator<Item = &'r Error>,
{
    for err in errors {
        match err {
            FormatError(loc, err) => {
                writeln!(w, "{}:{}: {}", loc.file.display(), loc.line, err)?;
            }
            DuplicateRequirement(r1, r2) => {
                writeln!(
                    w,
                    concat!(
                        "{}:{}: Duplicate Requirement: {}\n",
                        "{}:{}: note: previously seen here",
                    ),
                    r2.location.file.display(),
                    r2.location.line,
                    r1.id,
                    r1.location.file.display(),
                    r1.location.line,
                )?;
            }
            DuplicateAttribute(loc, attr) => {
                writeln!(
                    w,
                    "{}:{}: Duplicate Attribute: {}",
                    loc.file.display(),
                    loc.line,
                    attr,
                )?;
            }
            IoError(path, err) => {
                writeln!(w, "{}: IO Error: {}", path.display(), err,)?;
            }
            ArtefactTypeOnlyAllowsOnePath(_, _)
            | UnknownArtefactType(_)
            | ConfigError(_)
            | DuplicateArtefact(_)
            | UnknownArtefact(_)
            | EmptyGraph
            | UnknownJob(_)
            | UnknownFork(_, _) => {
                writeln!(w, "Error in config file: {:?}", err)?;
            }

            CoveredWithWrongTitle {
                upper,
                lower,
                wrong_title,
            } => {
                writeln!(
                    w,
                    concat!(
                        "{}:{}: {} covered with wrong title \n",
                        "    expected: {}\n",
                        "    actual  : {}\n",
                        "{}:{}: note: Defined here"
                    ),
                    lower.location.file.display(),
                    lower.location.line,
                    upper.id,
                    upper.title.as_ref().unwrap_or(&"<no title>".to_owned()),
                    wrong_title,
                    upper.location.file.display(),
                    upper.location.line,
                )?;
            }
            DependWithWrongTitle {
                upper,
                lower,
                wrong_title,
            } => {
                writeln!(
                    w,
                    concat!(
                        "{}:{}: {} depend with wrong title:\n",
                        "    expected: {}\n",
                        "     actual : {}\n",
                        "{}:{}: note: Defined here",
                    ),
                    upper.location.file.display(),
                    upper.location.line,
                    lower.id,
                    lower.title.as_ref().unwrap_or(&"<no title>".to_owned()),
                    wrong_title,
                    lower.location.file.display(),
                    lower.location.line,
                )?;
            }
        }
    }

    Ok(())
}

pub fn tracing<W>(tracing: &Tracing<'_>, graph: &Graph<'_>, w: &mut W) -> io::Result<()>
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
            writeln!(
                w,
                "{}:{}: {}:{}",
                req.location.file.display(),
                req.location.line,
                req.id,
                title
            )?;
        } else {
            writeln!(
                w,
                "{}:{}: {}",
                req.location.file.display(),
                req.location.line,
                req.id
            )?;
        }
    }

    writeln!(w, "Derived Requirement")?;
    for req in tracing.derived() {
        let req = req.requirement;
        if let Some(title) = &req.title {
            writeln!(
                w,
                "{}:{}: {}:{}",
                req.location.file.display(),
                req.location.line,
                req.id,
                title
            )?;
        } else {
            writeln!(
                w,
                "{}:{}: {}",
                req.location.file.display(),
                req.location.line,
                req.id
            )?;
        }
    }

    Ok(())
}
