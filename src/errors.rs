use std::{io, path::PathBuf, rc::Rc};

use crate::common::{Location, Requirement};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Invalid Format {0}: {1}")]
    FormatError(Location, String),

    #[error("Duplicate Requirements: {0} {1}")]
    DuplicateRequirement(Rc<Requirement>, Rc<Requirement>),

    #[error("Duplicate Attribute {1} in {0} when parsing {2}")]
    DuplicateAttribute(Location, String, String),

    #[error("IO Error: {1} in {0}")]
    IoError(PathBuf, io::Error),

    #[error("Only one Path expected for {0}, got: {1:?}")]
    ArtefactTypeOnlyAllowsOnePath(String, Vec<PathBuf>),

    #[error("unknown Artefact type: {0}")]
    UnknownArtefactType(String),

    #[error("Config Error: {0}")]
    ConfigError(String),

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
    },

    #[error("Requirement Depended with Wrong Title {upper} -> {lower} with {wrong_title}")]
    DependWithWrongTitle {
        upper: Rc<Requirement>,
        lower: Rc<Requirement>,
        wrong_title: String,
    },

    #[error("Requirement {0} depends on unknown Requirement {1}")]
    DependOnUnknownRequirement(Rc<Requirement>, String),

    #[error("Requirement {0} Covers unknown Requirement {1}")]
    CoversUnknownRequirement(Rc<Requirement>, String),

    #[error("Empty Tracing Graph")]
    EmptyGraph,

    #[error("Unknown Job {0}")]
    UnknownJob(String),
}

pub type Result<T> = std::result::Result<T, Error>;
