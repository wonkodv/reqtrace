use std::rc::Rc;

use crate::models;

use serde::Deserialize;
use serde::Serialize;

const VERSION: u32 = 0; // Version 0 is unstable, don't rely on it

#[derive(Serialize, Deserialize)]
pub struct Requirements {
    version: u32,
    requirements: Vec<Rc<models::Requirement>>,
}

impl Requirements {
    pub fn new<'r, R>(requirements: R) -> Self
    where
        R: Iterator<Item = &'r Rc<models::Requirement>>,
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
    errors: Vec<models::Error>,
}

impl Errors {
    pub fn new<'e, E>(errors: E) -> Self
    where
        E: Iterator<Item = &'e models::Error>,
    {
        let mut errors: Vec<_> = errors.map(models::Error::clone).collect();
        errors.sort_unstable_by_key(ToString::to_string);
        Self {
            version: VERSION,
            errors,
        }
    }
}

#[derive(Serialize)]
pub struct Trace<'g> {
    /// File Format version
    version: u32,
    trace: &'g models::TracedGraph,
}

impl<'g> Trace<'g> {
    pub fn new(trace: &'g models::TracedGraph) -> Self {
        Self {
            version: VERSION,
            trace,
        }
    }
}
