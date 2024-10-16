use super::super::common::*;
use std::rc::Rc;

use crate::errors::Error;
use crate::graph::Graph;
use crate::trace::Tracing;

use serde::ser::SerializeSeq;
use serde::ser::SerializeStruct;
use serde::Deserialize;
use serde::Serialize;
use serde::Serializer;

const VERSION: u32 = 0; // Version 0 is unstable, don't rely on it

#[derive(Serialize, Deserialize)]
pub struct Requirements {
    version: u32,
    requirements: Vec<Rc<Requirement>>,
}

impl Requirements {
    pub fn new<'r, R>(requirements: R) -> Self
    where
        R: Iterator<Item = &'r Rc<Requirement>>,
    {
        let mut requirements: Vec<_> = requirements.map(Rc::clone).collect();
        requirements.sort_by(|r, o| r.id.cmp(&o.id));
        Self {
            version: VERSION,
            requirements,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Errors {
    version: u32,
    errors: Vec<Error>,
}

impl Errors {
    pub fn new<'e, E>(errors: E) -> Self
    where
        E: Iterator<Item = &'e Error>,
    {
        let mut errors: Vec<_> = errors.map(Error::clone).collect();
        errors.sort_unstable_by_key(ToString::to_string);
        Self {
            version: VERSION,
            errors,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Artefact {
    /// identifier / name of the artefact
    id: String,
    /// input files
    files: Vec<String>,
    /// Parsed Requirements
    requirements: Vec<Rc<Requirement>>,
    /// Parser Errors
    errors: Vec<Error>,
}

#[derive(Serialize, Deserialize)]
pub struct Fork {
    /// id of upper Requirement
    upper: String,
    /// ids of lower requirement
    lower: Vec<String>,
    /// Requirement IDs, (upper, lower)
    covered: Vec<(String, String)>,
    /// uncovered Requirement IDs
    uncovered: Vec<(String, String)>,
}

#[derive(Serialize, Deserialize)]
pub struct Trace {
    /// File Format version
    version: u32,
    artefacts: Vec<Artefact>,
    coverages: Vec<Fork>,
    trace_errors: Vec<Error>,
    derived: Vec<Requirement>,
}

impl Trace {
    pub fn new(tracing: &Tracing<'_>, graph: &Graph) -> Self {
        todo!("Extract Coverage Data from tracing and graph")
    }
}
