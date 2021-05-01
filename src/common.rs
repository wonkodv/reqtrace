use once_cell::unsync::OnceCell;
use std::cell::UnsafeCell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;

use crate::parsers::markdown::markdown_parse;

pub const ATTR_COVERS: &str = "Covers";
pub const ATTR_DEPENDS: &str = "Depends";
pub const ATTR_DESCRIPTION: &str = "Description";
pub const ATTR_TAGS: &str = "Tags";

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Location {
    pub file: PathBuf,
    pub line: u32,
}

impl Location {
    pub fn new(file: PathBuf, line: u32) -> Self {
        Self { file, line }
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}:{}", self.file.display(), self.line);
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Reference {
    pub id: String,
    pub title: Option<String>,
}

#[derive(Debug, Default, PartialEq, Clone)]
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
            result += &title;
        }
        result += "\n";
        result += &self.location.to_string();

        if let Some(desc) = &self.attributes.get(ATTR_DESCRIPTION) {
            result += "\n\n";
            result += &desc;
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
            result += &k;
            result += ": ";
            result += &self.attributes[k];
        }

        return result;
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
    PrePopulated(Vec<Requirement>),
}

#[derive(Debug)]
pub enum ParserError {
    FormatError(Location, &'static str),
    DuplicateRequirement(Requirement, Requirement),
    DuplicateAttribute(Location, String),
    IOError(PathBuf, io::Error),
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParserError::FormatError(loc, err) => write!(f, "{}: {}", loc, err),
            ParserError::DuplicateRequirement(first, second) => write!(
                f,
                "{}: Duplicate Requirement {} previously defined at {}",
                second.location, second.id, first.location
            ),
            ParserError::DuplicateAttribute(loc, attr) => {
                write!(f, "{}: Duplicate Attribute {}", loc, attr)
            }
            ParserError::IOError(path, err) => write!(f, "{}: {}", path.display(), err),
        }
    }
}

#[derive(Debug, Default)]
struct ArtefactData {
    requirements: Vec<Requirement>,
    id_to_req: HashMap<String, u16>, // ID => Req  with  ID
    id_to_covering_req: HashMap<String, Vec<(u16, u16)>>, // ID => Reqs where ID in Req.Covers
    id_to_depending_req: HashMap<String, Vec<(u16, u16)>>, // ID => Reqs where ID in Req.Depends
    errors: Vec<ParserError>,
}

#[derive(Debug)]
pub struct Artefact<'a> {
    pub id: &'a str,
    pub config: ArtefactConfig<'a>,
    data: OnceCell<ArtefactData>,
}

impl<'a> Artefact<'a> {
    pub fn new(id: &'a str, config: ArtefactConfig<'a>) -> Self {
        let data = OnceCell::new();
        Self { id, config, data }
    }

    fn load<'s>(&'s self) -> &'s ArtefactData {
        if let Some(data) = self.data.get() {
            return data;
        }
        let mut data = ArtefactData::default();

        match &self.config {
            ArtefactConfig::Markdown(path) => {
                let file = fs::File::open(path).map_err(|e| ParserError::IOError(path.into(), e));
                match file {
                    Err(err) => {
                        data.errors = vec![err];
                    }
                    Ok(file) => {
                        let (s, e) = markdown_parse(file, path);
                        data.errors = e;
                        data.requirements = s;
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
                data.errors.push(ParserError::DuplicateRequirement(
                    data.requirements[old_idx].clone(),
                    req.clone(),
                ));
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

        if self.data.set(data).is_err() {
            unreachable!();
        }
        return self.data.get().unwrap();
    }

    fn req_with_idx(&self, idx: u16) -> &Requirement {
        let idx: usize = idx.into();
        &self.load().requirements[idx]
    }

    pub fn get_errors(&self) -> &[ParserError] {
        return &self.load().errors;
    }

    pub fn get_requirements(&self) -> &[Requirement] {
        return &self.load().requirements;
    }

    pub fn get_requirement_with_id(&self, id: &str) -> Option<&Requirement> {
        let d = self.load();
        if let Some(idx) = d.id_to_req.get(id) {
            return Some(self.req_with_idx(*idx));
        }
        None
    }

    pub fn get_requirements_that_cover(&self, id: &str) -> Vec<(&Requirement, Option<&str>)> {
        let d = self.load();
        let mut result = vec![];
        if let Some(covs) = d.id_to_covering_req.get(id) {
            for (req_id, cov_id) in covs {
                let r = self.req_with_idx(*req_id);
                let dep = r.covers[*cov_id as usize].title.as_ref();
                if let Some(title) = dep {
                    result.push((r, Some(title.as_str())));
                } else {
                    result.push((r, None));
                }
            }
        }
        return result;
    }
}



}



}

