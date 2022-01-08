use super::super::common::*;
use std::io;

use crate::errors::Error;
use Error::*;

pub fn errors<'r, W, R>(errors: R, w: &mut W) -> io::Result<()>
where
    W: io::Write,
    R: Iterator<Item = &'r Error>,
{
    writeln!(w, "# Parser Errors")?;
    for err in errors {
        match err {
            FormatError(loc, err) => {
                writeln!(w, "{}:{}: {}", loc.file.display(), loc.line, err)?;
            }
            DuplicateRequirement(r1, r2) => {
                writeln!(
                    w,
                    "{}:{}: Duplicate Requirement {} previously seen at {}:{}",
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
                    "{}:{}: Duplicate Attribute {}",
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
                    "{}:{}: {} covered with wrong title \n\texpected: {}\n\tactual  : {}",
                    upper.location.file.display(),
                    upper.location.line,
                    upper.id,
                    lower.title.as_ref().unwrap_or(&"<no title>".to_owned()),
                    wrong_title,
                )?;
            }
            DependWithWrongTitle {
                upper,
                lower,
                wrong_title,
            } => {
                writeln!(
                    w,
                    "{}:{}: {} depend with wrong title:\n\texpected: {}\n\tactual  : {}",
                    upper.location.file.display(),
                    upper.location.line,
                    upper.id,
                    lower.title.as_ref().unwrap_or(&"<no title>".to_owned()),
                    wrong_title,
                )?;
            }
        }
    }

    Ok(())
}
