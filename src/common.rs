use core::fmt;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::path::PathBuf;
use std::rc::Rc;

use crate::errors::Error;
use crate::parsers::{ArtefactData, ArtefactParser};
use crate::util;

pub const ATTR_COVERS: &str = "Covers";
pub const ATTR_DEPENDS: &str = "Depends";
pub const ATTR_DESCRIPTION: &str = "Description";

lazy_static::lazy_static! {
    static ref LOCATION_RE: Regex = Regex::new(r"^(?P<file>.*?)(?::(?P<line>\d+))?(?::(?P<column>\d+))?$").unwrap();
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum LocationInFile {
    Line(usize),
    LineAndColumn(usize, usize),
    String(String),
}

impl PartialOrd for LocationInFile {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self {
            LocationInFile::Line(line) => match other {
                LocationInFile::Line(other_line) => Some(line.cmp(other_line)),
                LocationInFile::LineAndColumn(other_line, other_column) => {
                    Some((line, &0_usize).cmp(&(other_line, other_column)))
                }
                LocationInFile::String(_) => None,
            },
            LocationInFile::LineAndColumn(line, column) => match other {
                LocationInFile::Line(other_line) => {
                    Some((line, column).cmp(&(other_line, &0_usize)))
                }
                LocationInFile::LineAndColumn(other_line, other_column) => {
                    Some((line, column).cmp(&(other_line, other_column)))
                }
                LocationInFile::String(_) => None,
            },
            LocationInFile::String(loc) => match other {
                LocationInFile::Line(_) | LocationInFile::LineAndColumn(_, _) => None,
                LocationInFile::String(other_loc) => Some(loc.cmp(other_loc)),
            },
        }
    }
}

impl fmt::Display for LocationInFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LocationInFile::Line(ref line) => write!(f, "{line}"),
            LocationInFile::LineAndColumn(line, column) => write!(f, "{line}:{column}"),
            LocationInFile::String(s) => write!(f, "{s}"),
        }
    }
}

#[derive(Debug, Default, PartialEq, PartialOrd, Clone, Serialize, Deserialize)]
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

    fn from_str(location: &str) -> Result<Self, String> {
        let caps = LOCATION_RE
            .captures(location)
            .ok_or("Invalid Location Pattern")?;
        let file = PathBuf::from(&caps["file"]);

        let location_in_file = if let Some(line) = caps.name("line") {
            let line = line.as_str();
            let line = line
                .parse()
                .map_err(|e| format!("Error parsing line {line}: {e}"))?;
            if let Some(column) = caps.name("column") {
                let column = column.as_str();
                let column = column
                    .parse()
                    .map_err(|e| format!("Error parsing column {column}: {e}"))?;
                Some(LocationInFile::LineAndColumn(line, column))
            } else {
                Some(LocationInFile::Line(line))
            }
        } else {
            None
        };

        Ok(Self {
            file,
            location_in_file,
        })
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
    pub title: Option<String>,
    pub location: Location,
    pub covers: Vec<Reference>,
    pub depends: Vec<Reference>,
    pub tags: Vec<String>,
    pub attributes: BTreeMap<String, String>,
}
requirement_covered!(DSG_REQ_FIELDS, ""); //TODO: turn this macro into an attribute that can be
                                          //applied to any item

impl fmt::Display for Requirement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.id)
    }
}

impl PartialOrd for Requirement {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

pub struct RequirementBuilder {
    req: Requirement,
}

impl RequirementBuilder {
    pub fn new(id: &str) -> Self {
        Self {
            req: Requirement {
                id: id.to_owned(),
                ..Requirement::default()
            },
        }
    }

    pub fn title(mut self, s: &str) -> Self {
        self.req.title = Some(s.to_owned());
        self
    }

    pub fn location(mut self, location: &str) -> Result<Self, String> {
        let l = Location::from_str(location)?;
        self.req.location = l;
        Ok(self)
    }

    pub fn covers(
        mut self,
        id: &str,
        title: Option<&str>,
        location: Option<&str>,
    ) -> Result<Self, String> {
        let id = id.to_owned();
        let title = title.map(std::borrow::ToOwned::to_owned);
        let location = {
            if let Some(location) = location {
                Some(Location::from_str(location)?)
            } else {
                None
            }
        };

        self.req.covers.push(Reference {
            id,
            title,
            location,
        });
        Ok(self)
    }

    pub fn depends(
        mut self,
        id: &str,
        title: Option<&str>,
        location: &str,
    ) -> Result<Self, String> {
        let id = id.to_owned();
        let title = title.map(std::borrow::ToOwned::to_owned);
        let location = Some(Location::from_str(location)?);

        self.req.covers.push(Reference {
            id,
            title,
            location,
        });
        Ok(self)
    }

    pub fn tag(mut self, tag: &str) -> Self {
        self.req.tags.push(tag.to_owned());
        self
    }

    pub fn attribute(mut self, key: &str, value: &str) -> Self {
        self.req.attributes.insert(key.to_owned(), value.to_owned());
        self
    }

    pub fn build(self) -> Requirement {
        self.req
    }
}

#[derive(Debug)]
pub struct Artefact {
    pub id: String,
    data: util::lazy::Lazy<ArtefactData, ArtefactParser>,
    pub ignore_derived_requirements: bool,
}

impl Artefact {
    pub fn new(id: String, parser: ArtefactParser, ignore_derived_requirements: bool) -> Self {
        requirement_covered!(DSG_ART_PARSE, "Artefact parses input lazily");
        let data = util::lazy::Lazy::new(ArtefactParser::parse, parser);

        Self {
            id,
            data,
            ignore_derived_requirements,
        }
    }

    fn req_with_idx(&self, idx: u16) -> &Rc<Requirement> {
        let idx: usize = idx.into();
        &self.data.get().requirements[idx]
    }

    pub fn get_errors(&self) -> &[Error] {
        &self.data.get().errors
    }

    pub fn get_requirements(&self) -> &[Rc<Requirement>] {
        &self.data.get().requirements
    }

    pub fn get_requirement_with_id(&self, id: &str) -> Option<&Rc<Requirement>> {
        if let Some(idx) = self.data.get().id_to_req.get(id) {
            return Some(self.req_with_idx(*idx));
        }
        None
    }

    pub fn get_requirements_that_cover<'b>(
        &'b self,
        id: &'b str,
    ) -> impl Iterator<Item = (&'b Rc<Requirement>, &'b Reference)> {
        let mut i;
        if let Some(covs) = self.data.get().id_to_covering_req.get(id) {
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
