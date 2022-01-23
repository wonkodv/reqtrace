use log::*;
use once_cell::unsync::OnceCell;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;
use std::{cell::UnsafeCell, rc::Rc};

use crate::errors::Error;
use crate::errors::Error::*;
use crate::parsers::markdown::markdown_parse;

pub const ATTR_COVERS: &str = "Covers";
pub const ATTR_DEPENDS: &str = "Depends";
pub const ATTR_DESCRIPTION: &str = "Description";
pub const ATTR_TAGS: &str = "Tags";

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
}

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Requirement {
    pub id: String,
    pub location: Location,
    pub title: Option<String>,
    pub covers: Vec<Reference>,
    pub depends: Vec<Reference>,
    pub tags: Vec<String>,
    pub attributes: HashMap<String, String>,
}

impl Requirement {
    pub fn to_markdown(&self) -> String {
        let mut result = String::with_capacity(1024);
        result += &self.id;
        if let Some(title) = &self.title {
            result += ": ";
            result += title;
        }
        result += "\n";
        result += &self.location.to_string();

        if let Some(desc) = &self.attributes.get(ATTR_DESCRIPTION) {
            result += "\n\n";
            result += desc;
        }

        if !self.covers.is_empty() {
            result += "\n\n";
            result += ATTR_COVERS;
            result += ": ";
            let mut i = self.covers.iter();
            result += &i.next().unwrap().id;
            for r in i {
                result += ", ";
                result += &r.id;
            }
        }

        if !self.depends.is_empty() {
            result += "\n\n";
            result += ATTR_DEPENDS;
            result += ": ";
            let mut i = self.depends.iter();
            result += &i.next().unwrap().id;
            for r in i {
                result += ", ";
                result += &r.id;
            }
        }

        if !self.tags.is_empty() {
            result += "\n\n";
            result += ATTR_TAGS;
            result += ": ";
            result += &self.tags.join(", ");
        }

        let mut keys = self.attributes.keys().into_iter().collect::<Vec<_>>();
        keys.sort();
        for k in keys {
            result += "\n\n";
            result += k;
            result += ": ";
            result += &self.attributes[k];
        }

        result
    }
}

impl fmt::Display for Requirement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.id)
    }
}

#[derive(Debug)]
pub enum ArtefactConfig<'a> {
    Markdown(&'a Path),
    PrePopulated(Vec<Rc<Requirement>>),
}

#[derive(Debug, Default)]
struct ArtefactData {
    requirements: Vec<Rc<Requirement>>,
    id_to_req: HashMap<String, u16>, // ID => Req  with  ID
    id_to_covering_req: HashMap<String, Vec<(u16, u16)>>, // ID => Reqs where ID in Req.Covers
    id_to_depending_req: HashMap<String, Vec<(u16, u16)>>, // ID => Reqs where ID in Req.Depends
    errors: Vec<Error>,
}

#[derive(Debug)]
pub struct Artefact {
    pub id: String,
    pub config: ArtefactConfig,
    data: OnceCell<ArtefactData>,
}

impl Artefact {
    pub fn new(id: String, config: ArtefactConfig) -> Self {
        let data = OnceCell::new();
        Self { id, config, data }
    }

    fn load(&self) -> &ArtefactData {
        if let Some(data) = self.data.get() {
            return data;
        }
        let mut data = ArtefactData::default();

        match &self.config {
            ArtefactConfig::Markdown(path) => {
                let file = fs::File::open(path).map_err(|e| IoError(path.into(), e));
                match file {
                    Err(err) => {
                        warn!("{}", err);
                        data.errors.push(err);
                    }
                    Ok(file) => {
                        let mut r = io::BufReader::new(file);
                        let (mut s, mut e) = markdown_parse(&mut r, path);
                        trace!(
                            "parsed {}: {} errors, {} requirements",
                            path.display(),
                            e.len(),
                            s.len()
                        );
                        trace!(
                            "parsed {}: {} errors, {} requirements",
                            path.display(),
                            e.len(),
                            s.len()
                        );
                        data.errors.append(&mut e);
                        data.requirements.append(&mut s);
                    }
                }
            }
            ArtefactConfig::PrePopulated(vec) => {
                data.requirements = vec.clone();
            }
        }

        for (req_idx, req) in data.requirements.iter().enumerate() {
            let old = data.id_to_req.insert(req.id.to_owned(), req_idx as u16);
            if let Some(old_idx) = old {
                let old_idx: usize = old_idx.into();

                /* Covers:  REQ_UNIQUE_ID: Requirements have a unique Identifier */
                let err =
                    DuplicateRequirement(Rc::clone(&data.requirements[old_idx]), Rc::clone(req));

                warn!("{}", err);

                data.errors.push(err);
            }

            for (cov_idx, cov) in req.covers.iter().enumerate() {
                data.id_to_covering_req
                    .entry(cov.id.to_owned())
                    .or_default()
                    .push((req_idx as u16, cov_idx as u16))
            }
            for (dep_idx, dep) in req.depends.iter().enumerate() {
                data.id_to_depending_req
                    .entry(dep.id.to_owned())
                    .or_default()
                    .push((req_idx as u16, dep_idx as u16))
            }
        }

        self.data.set(data).unwrap();
        self.data.get().unwrap()
    }

    fn req_with_idx(&self, idx: u16) -> &Rc<Requirement> {
        let idx: usize = idx.into();
        &self.load().requirements[idx]
    }

    pub fn get_errors(&self) -> &[Error] {
        return &self.load().errors;
    }

    pub fn get_requirements(&self) -> &[Rc<Requirement>] {
        return &self.load().requirements;
    }

    pub fn get_requirement_with_id(&self, id: &str) -> Option<&Rc<Requirement>> {
        let d = self.load();
        if let Some(idx) = d.id_to_req.get(id) {
            return Some(self.req_with_idx(*idx));
        }
        None
    }

    pub fn get_requirements_that_cover<'b>(
        &'b self,
        id: &'b str,
    ) -> impl Iterator<Item = (&'b Rc<Requirement>, Option<&'b str>)> {
        let d = self.load();

        let mut i;
        if let Some(covs) = d.id_to_covering_req.get(id) {
            i = Some(covs.iter());
        } else {
            i = None;
        }

        std::iter::from_fn(move || {
            if let Some(i) = &mut i {
                if let Some((req_id, cov_id)) = i.next() {
                    let r = self.req_with_idx(*req_id);
                    let dep = r.covers[*cov_id as usize].title.as_ref();
                    if let Some(title) = dep {
                        return Some((r, Some(title.as_str())));
                    } else {
                        return Some((r, None));
                    }
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
    Latex,
    TemplateFile(PathBuf),
    TemplateString(String),
}
