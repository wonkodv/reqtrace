use super::super::common::*;
use std::io;

pub fn requirements<'r, W, R>(reqs: R, w: &mut W) -> io::Result<()>
where
    W: io::Write,
    R: Iterator<Item = &'r Requirement>,
{
    for req in reqs {
        writeln!(
            w,
            "{}\t{}\t{};\"\tr",
            req.id,
            req.location.file.display(),
            req.location.line
        )?;
        for c in &req.covers {
            writeln!(
                w,
                "{}\t{}\t{};\"\tc",
                c.id,
                req.location.file.display(),
                req.location.line
            )?;
        }
        for d in &req.depends {
            writeln!(
                w,
                "{}\t{}\t{};\"\td",
                d.id,
                req.location.file.display(),
                req.location.line
            )?;
        }
    }
    Ok(())
}
