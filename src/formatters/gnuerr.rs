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
            e => {
                writeln!(w, "{:?}", e)?;
            }
        }
    }

    Ok(())
}
