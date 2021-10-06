use std::{io, path::PathBuf, rc::Rc};

use thiserror;

use crate::common::{Location, Requirement};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Invalid Format {0}, {1}")]
    FormatError(Location, String),

    #[error("Duplicate Requirement {0} {1}")]
    DuplicateRequirement(Rc<Requirement>, Rc<Requirement>),

    #[error("Duplicate Attribute {0}, {1}")]
    DuplicateAttribute(Location, String),

    #[error("IO Error {0} {1}")]
    IoError(PathBuf, io::Error),

    #[error("Only one Path expected")]
    OnlyOnePathExpected,

    #[error("unknown Artefact type {0}")]
    UnknownArtefactType(String),

    #[error("Config Error {0}")]
    ConfigError(String),

    #[error("Artefact added twice {0}")]
    DuplicateArtefact(String),

    #[error("Unknown Artefact {0}")]
    UnknownArtefact(String),

    #[error("Unknown Edge {0} {1} ")]
    UnknownFork(String, String),

    #[error("Requirement Covered with Wrong Title {0} {1} {2}")]
    CoveredWithWrongTitle(Rc<Requirement>, Rc<Requirement>, String),

    #[error("Requirement Depended with Wrong Title {0} {1} {2}")]
    DependWithWrongTitle(Rc<Requirement>, Rc<Requirement>, String),

    #[error("Combining Tracing with Edges contained in both")]
    CombinedTracingsWithIntersectingEdges,

    #[error("Empty Tracing Graph")]
    EmptyGraph,
}

pub type Result<T> = std::result::Result<T, Error>;
