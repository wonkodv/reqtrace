use std::fmt;
use std::path::PathBuf;

pub trait Location: fmt::Display + fmt::Debug + Clone + PartialEq {}

#[derive(Debug, PartialEq, Clone)]
pub struct FileLineLocation {
    file: PathBuf,
    line: u32,
}

impl FileLineLocation {
    pub fn new(file: PathBuf, line: u32) -> Self {
        Self { file, line }
    }
}

impl fmt::Display for FileLineLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}:{}", self.file.display(), self.line);
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Requirement {
    pub id: String,
    pub title: String,
    pub description: String,
    pub line_number: u32,
}

pub type Requirements = Vec<Requirement>; /* TODO: slice? */

impl fmt::Display for Requirement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{} ({})", self.id, self.title);
    }
}
