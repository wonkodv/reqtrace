use std::fmt;
use std::path::PathBuf;
use std::{collections::HashMap, error::Error};

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

pub trait Artefact: fmt::Debug + PartialEq {
    type Error: Error;
    type Parser: Iterator<Item = Result<Requirement, Self::Error>>;

    fn get_parser(&self) -> Result<Self::Parser, Self::Error>;

    fn parse(&self) -> (Vec<Requirement>, Vec<Self::Error>) {
        let mut errors = Vec::<Self::Error>::new();
        let mut reqs = Vec::<Requirement>::new();
        match self.get_parser() {
            Err(e) => errors.push(e),
            Ok(parser) => {
                for res in parser {
                    match res {
                        Ok(req) => reqs.push(req),
                        Err(e) => errors.push(e),
                    }
                }
            }
        }
        return (reqs, errors);
    }
}
