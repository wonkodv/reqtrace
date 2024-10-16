use super::super::common::{LocationInFile, Requirement};
use std::{io, rc::Rc};

pub fn requirements<'r, W, R>(reqs: R, w: &mut W) -> io::Result<()>
where
    W: io::Write,
    R: Iterator<Item = &'r Rc<Requirement>>,
{
    w.write_all(
        b"!_TAG_FILE_SORTED	1	/0=unsorted, 1=sorted, 2=foldcase/
!_TAG_PROGRAM_NAME	reqtrace/
!_TAG_PROGRAM_URL	https://github.com/wonkodv/reqtrace/
!_TAG_PROGRAM_VERSION	0	//
", // TODO: Version
    )?;

    let mut lines: Vec<String> = Vec::new();
    for req in reqs {
        match req.location.location_in_file {
            Some(LocationInFile::Line(line) | LocationInFile::LineAndColumn(line, _)) => {
                lines.push(format!(
                    "{}\t{}\t{};\"\tr\n",
                    req.id,
                    req.location.file.display(),
                    line,
                ));
            }
            Some(LocationInFile::String(_)) | None => {}
        }
    }
    lines.sort();
    for l in lines {
        w.write_all(l.as_bytes())?;
    }

    Ok(())
}
