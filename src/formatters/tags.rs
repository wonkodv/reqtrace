use std::io;

use crate::models::Graph;
use crate::models::LocationInFile;

pub fn requirements(graph: &Graph, w: &mut impl io::Write) -> io::Result<()> {
    w.write_all(
        b"!_TAG_FILE_SORTED	1	/0=unsorted, 1=sorted, 2=foldcase/
!_TAG_PROGRAM_NAME	reqtrace/
!_TAG_PROGRAM_URL	https://github.com/wonkodv/reqtrace/
!_TAG_PROGRAM_VERSION	0	//
", // TODO: Version
    )?;

    let mut lines: Vec<String> = Vec::new();
    for req in graph
        .artefacts
        .values()
        .flat_map(|art| art.requirements.values())
    {
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
