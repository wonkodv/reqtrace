use core::fmt;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::PathBuf;
use std::rc::Rc;

use crate::errors::Error;
use crate::parsers::{ArtefactData, ArtefactParser};
use crate::util;

pub const ATTR_COVERS: &str = "Covers";
pub const ATTR_DEPENDS: &str = "Depends";
pub const ATTR_DESCRIPTION: &str = "Description";

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum LocationInFile {
    Line(usize),
    LineAndColumn(usize, usize),
    String(String),
}

impl fmt::Display for LocationInFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LocationInFile::Line(line) => write!(f, "{line}"),
            LocationInFile::LineAndColumn(line, column) => write!(f, "{line}:{column}"),
            LocationInFile::String(s) => write!(f, "{s}"),
        }
    }
}

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Location {
    pub file: PathBuf,
    pub location_in_file: Option<LocationInFile>,
}

impl Location {
    pub fn new_with_no_pos(file: PathBuf) -> Self {
        Self {
            file,
            location_in_file: None,
        }
    }
    pub fn new_with_line_no(file: PathBuf, line: usize) -> Self {
        Self {
            file,
            location_in_file: Some(LocationInFile::Line(line)),
        }
    }
    pub fn new_with_line_and_column(file: PathBuf, line: usize, column: usize) -> Self {
        Self {
            file,
            location_in_file: Some(LocationInFile::LineAndColumn(line, column)),
        }
    }
    pub fn new_with_str(file: PathBuf, pos: String) -> Self {
        Self {
            file,
            location_in_file: Some(LocationInFile::String(pos)),
        }
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(loc) = &self.location_in_file {
            write!(f, "{}:{}", self.file.display(), loc)
        } else {
            write!(f, "{}", self.file.display())
        }
    }
}

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Reference {
    pub id: String,
    pub title: Option<String>,
    pub location: Option<Location>,
}

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Requirement {
    pub id: String,
    pub location: Location,
    pub title: Option<String>,
    pub covers: Vec<Reference>,
    pub depends: Vec<Reference>,
    pub tags: Vec<String>,
    pub attributes: BTreeMap<String, String>,
}

impl fmt::Display for Requirement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.id)
    }
}

#[derive(Debug)]
pub struct Artefact {
    pub id: String,
    data: util::lazy::Lazy<ArtefactData, ArtefactParser>,
}

impl Artefact {
    pub fn new(id: String, parser: ArtefactParser) -> Self {
        let data = util::lazy::Lazy::new(ArtefactParser::parse, parser);
        Self { id, data }
    }

    fn req_with_idx(&self, idx: u16) -> &Rc<Requirement> {
        let idx: usize = idx.into();
        &self.data.requirements[idx]
    }

    pub fn get_errors(&self) -> &[Error] {
        return &self.data.errors;
    }

    pub fn get_requirements(&self) -> &[Rc<Requirement>] {
        return &self.data.requirements;
    }

    pub fn get_requirement_with_id(&self, id: &str) -> Option<&Rc<Requirement>> {
        if let Some(idx) = self.data.id_to_req.get(id) {
            return Some(self.req_with_idx(*idx));
        }
        None
    }

    pub fn get_requirements_that_cover<'b>(
        &'b self,
        id: &'b str,
    ) -> impl Iterator<Item = (&'b Rc<Requirement>, &'b Reference)> {
        let mut i;
        if let Some(covs) = self.data.id_to_covering_req.get(id) {
            i = Some(covs.iter());
        } else {
            i = None;
        }

        std::iter::from_fn(move || {
            if let Some(i) = &mut i {
                if let Some((req_id, cov_id)) = i.next() {
                    let r = self.req_with_idx(*req_id);
                    let reference = &r.covers[*cov_id as usize];
                    return Some((r, reference));
                }
            }
            None
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Format {
    GnuError,
    Json,
    Markdown,
    Tags,
    TemplateFile(PathBuf),
    TemplateString(String),
}
