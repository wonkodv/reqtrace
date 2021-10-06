use super::super::common::*;
use std::{io, rc::Rc};

pub fn requirements<'r, W, R>(reqs: R, w: &mut W) -> io::Result<()>
where
    W: io::Write,
    R: Iterator<Item = &'r Rc<Requirement>>,
{
    for req in reqs {
        writeln!(
            w,
            "{}\t{}\t{};\"\tr",
            req.id,
            req.location.file.display(),
            req.location.line
        )?;
    }
    Ok(())
}
