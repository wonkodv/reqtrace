use super::super::common::*;
use std::{io, rc::Rc};

pub fn requirements<'r, W, R>(reqs: R, w: &mut W) -> io::Result<()>
where
    W: io::Write,
    R: Iterator<Item = &'r Rc<Requirement>>,
{
    for req in reqs {
        match req.location.location_in_file {
            Some(LocationInFile::Line(line) | LocationInFile::LineAndColumn(line, _)) => {
                writeln!(
                    w,
                    "{}\t{}\t{};\"\tr",
                    req.id,
                    req.location.file.display(),
                    line,
                )?;
            }
            Some(LocationInFile::String(_)) => {},
            None => {},
        }
    }
    Ok(())
}
