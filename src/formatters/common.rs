
#[derive(Debug)]
pub enum ParserError {
    FormatError(Location, &'static str),
    DuplicateRequirement(Requirement, Requirement),
    DuplicateAttribute(Location, String),
    IOError(PathBuf, io::Error),
}

