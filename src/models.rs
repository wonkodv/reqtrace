//! Simple Datastructures without much logic

use core::fmt;
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::io;
use std::path::PathBuf;
use std::rc::Rc;

use regex::Regex;
use serde::ser::SerializeTuple as _;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct ArtefactId(String);

impl From<&str> for ArtefactId {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}

impl From<String> for ArtefactId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl fmt::Display for ArtefactId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct RequirementId(String);

impl From<&str> for RequirementId {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}

impl From<String> for RequirementId {
    fn from(value: String) -> Self {
        Self(value)
    }
}
impl fmt::Display for RequirementId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ArtefactParser {
    Markdown,
    Rust,
    MonoRequirement,
    Json,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ArtefactConfig {
    pub id: ArtefactId,
    pub paths: Vec<String>,
    pub parser: ArtefactParser,
    /// Do not report requirements of this artefact, which do not cover other requirements, as
    /// derived
    pub ignore_derived_requirements: Option<bool>,
    /// References to requirements of this artefact must match with title (usually a good idea if
    /// the requirement ids are mostly numeric like `DSG_0123`)
    pub reference_with_title: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub version: Option<u32>,
    pub artefacts: Vec<ArtefactConfig>,
    pub relations: Vec<Relation>,
    pub jobs: Option<BTreeMap<String, Job>>,
    pub default_jobs: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Query {
    /// Parse all Artefacts
    Parse,

    /// Trace all Edges in Graph
    Trace,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Job {
    pub query: Query,
    pub format: Format,
    pub file: PathBuf,
    pub set_return_code: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Format {
    GnuError,
    Json,
    Markdown,
    Tags,
    TemplateFile(PathBuf),
    TemplateString(String),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum LocationInFile {
    Line(usize),
    LineAndColumn(usize, usize),
    String(String),
}

impl PartialOrd for LocationInFile {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self {
            LocationInFile::Line(line) => match other {
                LocationInFile::Line(other_line) => Some(line.cmp(other_line)),
                LocationInFile::LineAndColumn(other_line, other_column) => {
                    Some((line, &0_usize).cmp(&(other_line, other_column)))
                }
                LocationInFile::String(_) => None,
            },
            LocationInFile::LineAndColumn(line, column) => match other {
                LocationInFile::Line(other_line) => {
                    Some((line, column).cmp(&(other_line, &0_usize)))
                }
                LocationInFile::LineAndColumn(other_line, other_column) => {
                    Some((line, column).cmp(&(other_line, other_column)))
                }
                LocationInFile::String(_) => None,
            },
            LocationInFile::String(loc) => match other {
                LocationInFile::Line(_) | LocationInFile::LineAndColumn(_, _) => None,
                LocationInFile::String(other_loc) => Some(loc.cmp(other_loc)),
            },
        }
    }
}

impl fmt::Display for LocationInFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LocationInFile::Line(ref line) => write!(f, "{line}"),
            LocationInFile::LineAndColumn(line, column) => write!(f, "{line}:{column}"),
            LocationInFile::String(s) => write!(f, "{s}"),
        }
    }
}

#[derive(Debug, Default, PartialEq, PartialOrd, Clone, Serialize, Deserialize)]
pub struct Location {
    pub file: PathBuf,
    pub location_in_file: Option<LocationInFile>,
}

lazy_static::lazy_static! {
    static ref LOCATION_RE: Regex = Regex::new(r"^(?P<file>.+?)(?::(?P<line>\d+))?(?::(?P<column>\d+))?$").unwrap();
}

impl Location {
    pub fn new_with_no_pos(file: PathBuf) -> Self {
        Self {
            file,
            location_in_file: None,
        }
    }
    pub fn new_with_line_no(file: PathBuf, line: usize) -> Self {
        Self {
            file,
            location_in_file: Some(LocationInFile::Line(line)),
        }
    }
    pub fn new_with_line_and_column(file: PathBuf, line: usize, column: usize) -> Self {
        Self {
            file,
            location_in_file: Some(LocationInFile::LineAndColumn(line, column)),
        }
    }
    pub fn new_with_str(file: PathBuf, pos: String) -> Self {
        Self {
            file,
            location_in_file: Some(LocationInFile::String(pos)),
        }
    }

    pub fn parse(location: &str) -> Result<Self, String> {
        let caps = LOCATION_RE.captures(location).ok_or("Invalid Location")?;
        let file = PathBuf::from(&caps["file"]);

        let location_in_file = if let Some(line) = caps.name("line") {
            let line = line.as_str();
            let line = line
                .parse()
                .map_err(|e| format!("Error parsing line {line}: {e}"))?;
            if let Some(column) = caps.name("column") {
                let column = column.as_str();
                let column = column
                    .parse()
                    .map_err(|e| format!("Error parsing column {column}: {e}"))?;
                Some(LocationInFile::LineAndColumn(line, column))
            } else {
                Some(LocationInFile::Line(line))
            }
        } else {
            None
        };

        Ok(Self {
            file,
            location_in_file,
        })
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(loc) = &self.location_in_file {
            write!(f, "{}:{}", self.file.display(), loc)
        } else {
            write!(f, "{}", self.file.display())
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Reference {
    pub id: RequirementId,
    pub title: Option<String>,
    pub location: Location,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Requirement {
    pub id: RequirementId,
    pub title: Option<String>,
    pub location: Location,
    pub covers: Vec<Reference>,
    pub depends: Vec<Reference>,
    pub tags: Vec<String>,
    pub attributes: BTreeMap<String, String>,
}
requirement_covered!(DSG_REQ_FIELDS, ""); //TODO: turn this macro into an attribute that can be
                                          //applied to any item

impl fmt::Display for Requirement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.id.fmt(f)
    }
}

impl PartialOrd for Requirement {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Artefact {
    pub id: ArtefactId,
    pub files: Vec<PathBuf>,
    pub ignore_derived_requirements: bool,
    pub reference_with_title: bool,
    pub requirements: BTreeMap<RequirementId, Rc<Requirement>>,
    pub errors: Vec<Error>,
}

#[derive(
    thiserror::Error, Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize,
)]
pub enum Error {
    #[error("Invalid Format {0}: {1}")]
    Format(Location, String),

    #[error("Duplicate Requirements: {0} {1}")]
    DuplicateRequirement(Rc<Requirement>, Rc<Requirement>),

    #[error("Duplicate Attribute {1} when parsing {2} at {0}")]
    DuplicateAttribute(Location, String, RequirementId),

    #[error("IO Error: {1} in {0}")]
    Io(PathBuf, String),

    #[error("Configuration Error  {0}")]
    Config(String),

    #[error("Artefact added twice: {0}")]
    DuplicateArtefact(String),

    #[error("Unknown Artefact: {0}")]
    UnknownArtefact(ArtefactId),

    #[error(
        "Requirement Covered with Wrong Title {upper} <- {lower} with {wrong_title} at {location}"
    )]
    CoveredWithWrongTitle {
        upper: Rc<Requirement>,
        lower: Rc<Requirement>,
        wrong_title: String,
        location: Location,
    },

    #[error(
        "Requirement Depended with Wrong Title {upper} -> {lower} with {wrong_title} at {location}"
    )]
    DependWithWrongTitle {
        upper: Rc<Requirement>,
        lower: Rc<Requirement>,
        wrong_title: String,
        location: Location,
    },

    #[error("Requirement referenced without title {referenced} at {location}")]
    ReferencedWithoutTitle {
        referenced: Rc<Requirement>,
        location: Location,
    },

    #[error("Requirement {0} must have a title")]
    RequirementWithoutTitle(Rc<Requirement>),

    #[error("Requirement {0} depends on unknown Requirement {1} at {2}")]
    DependOnUnknownRequirement(Rc<Requirement>, RequirementId, Location),

    #[error("Requirement {0} covers unknown Requirement {1} at {2}")]
    CoversUnknownRequirement(Rc<Requirement>, RequirementId, Location),

    #[error("Empty Tracing Graph")]
    EmptyGraph,

    #[error("No requirements covered by relation {0}")]
    UnusedRelation(Relation),
}

impl Error {
    pub fn io(path: &std::path::Path, err: &std::io::Error) -> Self {
        Self::Io(path.to_owned(), err.to_string())
    }
}

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

#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq, Serialize, Deserialize)]
pub struct Relation {
    /// id of the upper artefact
    pub upper: ArtefactId,

    /// id of lower artefacts
    pub lower: Vec<ArtefactId>,
}

impl fmt::Display for Relation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let lower = self
            .lower
            .iter()
            .map(|id| id.0.clone()) // TODO: there has to be a better way
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "{} => [{}]", self.upper, lower,)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Graph {
    /// Artefacts indexed by their name
    pub artefacts: BTreeMap<ArtefactId, Rc<Artefact>>,

    /// Traced Relationships in the Graph
    pub relations: Vec<Relation>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CoverageDirection {
    /// The lower requirement claims to cover the upper
    Upwards,

    /// The upper requirement claims it is covered by the lower
    Downwards,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Coverage {
    pub upper: RequirementId,
    pub lower: RequirementId,
    pub location: Location,
    pub direction: CoverageDirection,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TracedRelation {
    #[serde(flatten)]
    pub relation: Relation,

    /// Requirements covered by this relation
    /// `(upper.id, lower.id, location)`
    pub covered: Vec<Coverage>,

    /// Requirements in upper that were not covered by Lower
    /// `upper.id`
    pub uncovered: Vec<RequirementId>,
}

impl fmt::Display for TracedRelation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.relation.fmt(f)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TracedGraph {
    /// Artefacts indexed by their name
    pub artefacts: BTreeMap<ArtefactId, Rc<Artefact>>,

    /// Traced Relationships in the Graph
    pub traced_relations: Vec<TracedRelation>,

    /// Derived requirements per artefact
    pub derived: BTreeMap<ArtefactId, Vec<RequirementId>>,

    /// Errors while Tracing
    pub errors: Vec<Error>,
}

#[cfg(test)]
mod test {
    use super::*;

    mod location_from_string {
        use super::*;

        #[test]
        fn empty_returns_an_error() {
            assert_eq!(Location::parse(""), Err("Invalid Location".to_owned()));
        }

        #[test]
        fn file_only_gives_no_pos() {
            assert_eq!(
                Location::parse("path/to/file.txt"),
                Ok(Location::new_with_no_pos(PathBuf::from("path/to/file.txt")))
            );
        }
        #[test]
        fn file_and_line_give_line_pos() {
            assert_eq!(
                Location::parse("path/to/file.txt:42"),
                Ok(Location::new_with_line_no(
                    PathBuf::from("path/to/file.txt"),
                    42
                ))
            );
        }
        #[test]
        fn file_line_col_give_line_and_column_pos() {
            assert_eq!(
                Location::parse("path/to/file.txt:42:67"),
                Ok(Location::new_with_line_and_column(
                    PathBuf::from("path/to/file.txt"),
                    42,
                    67
                ))
            );
        }
        #[test]
        fn rightmost_number_fields_count() {
            assert_eq!(
                Location::parse("path/to/file.txt:42:67:111"),
                Ok(Location::new_with_line_and_column(
                    PathBuf::from("path/to/file.txt:42"),
                    67,
                    111
                ))
            );
        }
        #[test]
        fn filename_can_have_whitespace() {
            assert_eq!(
                Location::parse("path/to/f ile.txt:42:67"),
                Ok(Location::new_with_line_and_column(
                    PathBuf::from("path/to/f ile.txt"),
                    42,
                    67
                ))
            );
        }
        #[test]
        fn windows_drive_letter_filenames_work() {
            assert_eq!(
                Location::parse(r"C:\path\to\file.txt:42:67"),
                Ok(Location::new_with_line_and_column(
                    PathBuf::from(r"C:\path\to\file.txt"),
                    42,
                    67
                ))
            );
        }
    }
}
