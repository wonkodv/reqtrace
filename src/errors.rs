use std::{io, path::PathBuf, rc::Rc};

use serde::ser::{Serialize, SerializeTuple, Serializer};

use crate::common::{Location, Requirement};

#[derive(
    thiserror::Error, Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize,
)]
pub enum Error {
    #[error("Invalid Format {0}: {1}")]
    Format(Location, String),

    #[error("Duplicate Requirements: {0} {1}")]
    DuplicateRequirement(Rc<Requirement>, Rc<Requirement>),

    #[error("Duplicate Attribute {1} when parsing {2} at {0}")]
    DuplicateAttribute(Location, String, String),

    #[error("IO Error: {1} in {0}")]
    Io(PathBuf, String),

    #[error("Only one Path expected for {0}, got: {1:?}")]
    ArtefactTypeOnlyAllowsOnePath(String, Vec<String>),

    #[error("unknown Artefact type: {0}")]
    UnknownArtefactType(String),

    #[error("Config Error: {0}")]
    Config(String),

    #[error("Artefact added twice: {0}")]
    DuplicateArtefact(String),

    #[error("Unknown Artefact: {0}")]
    UnknownArtefact(String),

    #[error("Unknown Edge: {0} -> {1} ")]
    UnknownFork(String, String),

    #[error("Requirement Covered with Wrong Title {upper} <- {lower} with {wrong_title}")]
    CoveredWithWrongTitle {
        upper: Rc<Requirement>,
        lower: Rc<Requirement>,
        wrong_title: String,
        location: Option<Location>,
    },

    #[error("Requirement Depended with Wrong Title {upper} -> {lower} with {wrong_title}")]
    DependWithWrongTitle {
        upper: Rc<Requirement>,
        lower: Rc<Requirement>,
        wrong_title: String,
        location: Option<Location>,
    },

    #[error("Requirement {0} depends on unknown Requirement {1} at {2:?}")]
    DependOnUnknownRequirement(Rc<Requirement>, String, Option<Location>),

    #[error("Requirement {0} Covers unknown Requirement {1} at {2:?}")]
    CoversUnknownRequirement(Rc<Requirement>, String, Option<Location>),

    #[error("Empty Tracing Graph")]
    EmptyGraph,

    #[error("Unknown Job {0}")]
    UnknownJob(String),
}

impl Error {
    pub fn io(path: &std::path::Path, err: std::io::Error) -> Self {
        Self::Io(path.to_owned(), err.to_string())
    }
}

pub type Result<T> = std::result::Result<T, Error>;

fn serialize_io<S>(
    path: &PathBuf,
    err: &io::Error,
    serializer: S,
) -> std::result::Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let mut tup = serializer.serialize_tuple(2)?;
    tup.serialize_element(path)?;
    tup.serialize_element(&err.to_string())?;
    tup.end()
}
