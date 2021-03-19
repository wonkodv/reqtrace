use std::collections::HashMap;
use std::path::PathBuf;
use std::{fmt, fs, io, path::Path};

use crate::markdown::markdown_parse;

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

#[allow(dead_code)]
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

pub enum ArtefactConfig<'a> {
    Markdown(&'a Path),
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
            ParserError::DuplicateRequirement(first, second) => write!(f, "{}: Duplicate Requirement {} previously defined at {}", second.location, second.id, first.location ),
            ParserError::DuplicateAttribute(loc, attr) => write!(f, "{}: Duplicate Attribute {}", loc, attr),
            ParserError::IOError(path, err) => write!(f, "{}: {}", path.display(), err),
        }
    }
}




pub struct Artefact<'a> {
    #[allow(dead_code)] // TODO
    config: &'a ArtefactConfig<'a>,
    store: Vec<Requirement>,
    requirements: HashMap<String, u16>, // ID => Req  with  ID
    covers: HashMap<String, Vec<u16>>,  // ID => Reqs where ID in Req.Covers
    depends: HashMap<String, Vec<u16>>, // ID => Reqs where ID in Req.Depends
    errors: Vec<ParserError>,
}

impl<'a> Artefact<'a> {
    pub fn open(config: &'a ArtefactConfig) -> Self {
        let store;
        let mut errors;

        match config {
            ArtefactConfig::Markdown(path) => {
                let file = fs::File::open(path).map_err(|e| ParserError::IOError(path.into(), e));
                match file {
                    Err(err) => {
                        errors = vec![err];
                        store = vec![];
                    }
                    Ok(file) => {
                        let (s, e) = markdown_parse(file, path);
                        errors = e;
                        store = s;
                    }
                }
            }
        }

        let mut requirements = HashMap::<String, u16>::new();
        let mut covers = HashMap::<String, Vec<u16>>::new();
        let mut depends = HashMap::<String, Vec<u16>>::new();

        let mut idx: u16 = 0;
        for req in &store {
            let old = requirements.insert(req.id.to_owned(), idx);
            if let Some(old_idx) = old {
                let old_idx: usize = old_idx.into();
                errors.push(ParserError::DuplicateRequirement(
                    store[old_idx].clone(),
                    req.clone(),
                ));
            }

            for cov in &req.covers {
                covers.entry(cov.id.to_owned()).or_default().push(idx)
            }
            for dep in &req.depends {
                depends.entry(dep.id.to_owned()).or_default().push(idx)
            }

            idx += 1;
        }

        return Self {
            config,
            store,
            requirements,
            covers,
            depends,
            errors,
        };
    }

    fn req_with_idx(&self, idx: u16) -> &Requirement {
        let idx: usize = idx.into();
        &self.store[idx]
    }

    pub fn get_errors(&self) -> &[ParserError] {
        return &self.errors;
    }

    pub fn get_requirements(&self) -> &[Requirement] {
        return &self.store;
    }

    #[allow(dead_code)]
    pub fn get_requirement_with_id(&self, id: &str) -> Option<&Requirement> {
        if let Some(idx) = self.requirements.get(id) {
            return Some(self.req_with_idx(*idx));
        }
        None
    }

    pub fn get_requirements_that_cover(&self, id: &str) -> Vec<&Requirement> {
        if let Some(covs) = self.covers.get(id) {
            covs.iter().map(|idx| self.req_with_idx(*idx)).collect()
        } else {
            vec![]
        }
    }
}
