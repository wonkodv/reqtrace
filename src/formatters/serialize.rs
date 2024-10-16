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
    pub fn new<'r, R>(mut requirements: R) -> Self
    where
        R: Iterator<Item = &'r Rc<Requirement>>,
    {
        let requirements: Vec<_> = requirements.map(Rc::clone).collect();
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
    pub fn new<'e, E>(mut errors: E) -> Self
    where
        E: Iterator<Item = &'e Error>,
    {
        let errors: Vec<_> = errors.map(Rc::clone).collect();
        errors.sort_unstable_by_key(ToString::to_string);
        Self {
            version: VERSION,
            errors,
        }
    }
}

pub Artefact {
    /// identifier / name of the artefact
    id:String,
    /// input files
    files:Vec<PathBuf>,
    /// Parsed Requirements
    Requirements:Vec<Rc<Requirement>>,
    /// Parser Errors
    errors: Vec<Error>,
}

pub struct Fork {
    /// id of upper Requirement
    upper:String,
    /// ids of lower requirement
    lower:Vec<String>,
}

pub struct Traceing {
    trace_errors: Vec<Error>,
    coverages:
}

pub struct Tracing {
    /// File Format version
    version: u32,
    graph:Vec<String,Vec<String,String>>,
    requirements: Vec<Rc<Requirement>>,


