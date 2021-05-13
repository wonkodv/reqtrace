pub mod markdown;

use std::{io, path::PathBuf, rc::Rc};

use thiserror::Error;

use crate::common::{Location, Requirement};

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("Invalid Format {0}, {1}")]
    FormatError(Location, &'static str),

    #[error("Duplicate Requirement {0} {1}")]
    DuplicateRequirement(Rc<Requirement>, Rc<Requirement>),

    #[error("Duplicate Attribute {0}, {1}")]
    DuplicateAttribute(Location, String),

    #[error("IO Error {0} {1}")]
    IoError(PathBuf, io::Error),
}
