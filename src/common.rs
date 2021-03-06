use std::error::Error;
use std::fmt;
use std::path::PathBuf;


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
pub struct Requirement {
    pub id: String,
    pub title: String,
    pub description: String,
    pub location: Location,
}

impl Requirement {
    pub fn to_markdown(&self) -> String {
        format!(
            "# {}: {}\n\n{}\n\n{}\n",
            self.id, self.title, self.location, self.description
        )
    }
}

impl fmt::Display for Requirement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{} ({})", self.id, self.title);
    }
}

pub type Requirements = Vec<Requirement>;

pub trait Artefact: fmt::Debug + PartialEq {
    type ERROR: Error;

    fn parse(&self) -> Result<Requirements, Self::ERROR>;
}
