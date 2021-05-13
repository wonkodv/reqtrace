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
            e => {
                // TODO
                writeln!(w, "{:?}", e)?;
            }
        }
    }

    Ok(())
}
