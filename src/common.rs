use std::collections::HashMap;
use std::path::PathBuf;
use std::{fmt, fs, io, path::Path};
use thiserror::Error;

use crate::markdown::MarkdownParser;

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
    pub description: Option<String>,
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

        if let Some(desc) = &self.description {
            result += "\n\n";
            result += &desc;
        }

        if !self.covers.is_empty() {
            result += "\n\nCovers: ";
            let mut i = self.covers.iter();
            result += &i.next().unwrap().id;
            for r in i {
                result += ", ";
                result += &r.id;
            }
        }

        if !self.depends.is_empty() {
            result += "\n\nDepends: ";
            let mut i = self.covers.iter();
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

#[allow(dead_code)] // TODO
#[derive(Error, Debug)]
pub enum ParserError {
    #[error("Duplicate Requirement {0} and {1}")]
    DuplicateRequirement(Requirement, Requirement),

    #[error("Bad Format: {1} at {0}")]
    FormatError(Location, &'static str),

    #[error("Duplicate Attribute: {1} at {0}")]
    DuplicateAttribute(Location, String),

    #[error("File Read error")]
    IOError(PathBuf, io::Error),
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
        let mut s = Self {
            config,
            store: Vec::new(),
            requirements: HashMap::new(),
            covers: HashMap::new(),
            depends: HashMap::new(),
            errors: Vec::new(),
        };

        match config {
            ArtefactConfig::Markdown(path) => {
                let file = fs::File::open(path).map_err(|e| ParserError::IOError(path.into(), e));
                match file {
                    Err(err) => s.errors.push(err),
                    Ok(file) => {
                        let parser = MarkdownParser::new(file, path.into());
                        for res in parser {
                            match res {
                                Ok(req) => s.store.push(req),
                                Err(e) => s.errors.push(e),
                            }
                        }
                    }
                }
            }
        }

        let mut idx: u16 = 0;
        for req in &s.store {
            let old = s.requirements.insert(req.id.to_owned(), idx);
            if let Some(old_idx) = old {
                s.errors.push(ParserError::DuplicateRequirement(
                    s.req_with_idx(old_idx).clone(),
                    req.clone(),
                ));
            }

            for cov in &req.covers {
                s.covers.entry(cov.id.to_owned()).or_default().push(idx)
            }
            for dep in &req.depends {
                s.depends.entry(dep.id.to_owned()).or_default().push(idx)
            }

            idx += 1;
        }
        s
    }

    fn req_with_idx(&self, idx:u16) -> &Requirement {
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
    pub fn get_requirement_with_id(&self, id:&str) -> Option<&Requirement> {
        if let Some(idx) = self.requirements.get(id) {
            return Some(self.req_with_idx(*idx));
        }
        None
    }

    pub fn get_requirements_that_cover(&self, id:&str) -> Vec<&Requirement> {
        if let Some(covs) = self.covers.get(id) {
            covs.iter().map(|idx| self.req_with_idx(*idx)).collect()
        } else {
            vec![]
        }
    }
}
