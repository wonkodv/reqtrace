use crate::parsers::ParserError;

use super::super::common::*;
use std::io;

pub fn errors<'r, W, R>(errors: R, w: &mut W) -> io::Result<()>
where
    W: io::Write,
    R: Iterator<Item = &'r ParserError>,
{
    writeln!(w, "# Parser Errors")?;
    for err in errors {
        match err {
            ParserError::FormatError(loc, err) => {
                writeln!(w, "{}:{}: {}", loc.file.display(), loc.line, err)?;
            }
            ParserError::DuplicateRequirement(r1, r2) => {
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
            ParserError::DuplicateAttribute(loc, attr) => {
                writeln!(
                    w,
                    "{}:{}: Duplicate Attribute {}",
                    loc.file.display(),
                    loc.line,
                    attr,
                )?;
            }
            ParserError::IoError(path, err) => {
                writeln!(w, "{}: IO Error: {}", path.display(), err,)?;
            }
        }
    }

    Ok(())
}
