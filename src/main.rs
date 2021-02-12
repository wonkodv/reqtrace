// use pulldown_cmark::Parser;
// use anyhow::Result;
use std::fmt;
use std::fs;
use std::io;
use std::io::BufRead;
use std::path::Path;
use std::path::PathBuf;

use regex::Regex;
use thiserror::Error;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref HEADING_LINE: Regex = Regex::new(r"^(#+)").unwrap();
    static ref REQUIREMENT_LINE: Regex =
        Regex::new(r"^(#+)\s*([A-Za-z][a-zA-Z0-9_-]*):\s*(.+)\s*$").unwrap();
    static ref COVER_LINE: Regex =
        Regex::new(r"^Covers:\s*([A-Za-z][a-zA-Z0-9_-]*)\s*\(([^)]+)\)\s*$").unwrap();
}

trait Location: fmt::Display + fmt::Debug + Clone + PartialEq {}

#[derive(Debug, PartialEq, Clone)]
struct FileLineLocation {
    file: PathBuf,
    line: u32,
}

impl FileLineLocation {
    fn new(file: PathBuf, line: u32) -> Self {
        Self { file, line }
    }
}

impl fmt::Display for FileLineLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}:{}", self.file.display(), self.line);
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
struct Requirement {
    id: String,
    title: String,
    description: String,
    line_number: u32,
}

type Requirements = Vec<Requirement>; /* TODO: slice? */

impl fmt::Display for Requirement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{} ({})", self.id, self.title);
    }
}

/* Move to parsers/markdown  {{{ */

#[allow(dead_code)]
#[derive(Error, Debug)]
enum MarkdownParserError {
    #[error("If `prefixes` is empty, require_prefix must be false")]
    RequiredPrefixWithoutPrefix,

    #[error("Heading with Prefix not declaring a Requirement at {0}")]
    InvalidPrefix(FileLineLocation),

    #[error("Nested Requirement at {0}")]
    IllegalNesting(FileLineLocation),

    #[error("File Read error")]
    IOError(PathBuf, io::Error),
}

type MarkdownParserResult = Result<Requirements, MarkdownParserError>;

#[derive(Debug, PartialEq, Clone)]
struct MarkdownArtefact<'a> {
    path: &'a Path,
}

impl<'a> fmt::Display for MarkdownArtefact<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "MarkdownArtefact({})", self.path.display());
    }
}

fn markdown_parse<R: io::Read>(read: R, path: PathBuf) -> MarkdownParserResult {
    let mut reader = io::BufReader::new(read);
    let mut results = Vec::new();

    let mut line_number: u32 = 0;
    let mut line = String::new();

    let mut current_level: usize = 0;
    let mut req = Requirement::default();

    let mut collecting = false;

    loop {
        line_number += 1;
        line.clear();
        let r = reader.read_line(&mut line).unwrap(); // TODO: .map_err(|e| MarkdownParserError::IOError(p, e))?;

        if let Some(heading) = HEADING_LINE.captures(&line) {
            let level = heading[1].len();

            let requirement_match = REQUIREMENT_LINE.captures(&line);

            if let Some(requirement_match) = requirement_match {
                let id = requirement_match[2].into();
                let title = requirement_match[3].into();

                /* new Requirement starting  */
                if collecting {
                    /* working on a req currently               */
                    if level > current_level {
                        /*  ### REQ
                         *  #### NESTED-REQ
                         */
                        return Err(MarkdownParserError::IllegalNesting(FileLineLocation {
                            file: path,
                            line: line_number,
                        }));
                    } else if level == current_level {
                        results.push(req.clone());
                    } else if level < current_level {
                        /*  ### SORT-OF-NESTED-REQ
                         *  ## REQ
                         */
                        /* this is not strictly nesting but looks like it. needs a headline
                         * without req in between.
                         */
                        return Err(MarkdownParserError::IllegalNesting(FileLineLocation {
                            file: path,
                            line: line_number,
                        }));
                    } else {
                        unreachable!();
                    }
                }

                req.title = title;
                req.id = id;
                req.description.clear();
                req.line_number = line_number;
                current_level = level;
                collecting = true;
            } else {
                /* headline but not a requirement */
                if level > current_level {
                    /*
                     * ## REQ
                     * ### Head Line as part of description
                     */
                    req.description += line[current_level..].trim_start(); /* part of description */
                } else {
                    /*
                     * ## REQ
                     * ## Head Line
                     */
                    eprint!("finished Req: {}\n", req);
                    results.push(req.clone());
                    collecting = false;
                }
            }
        } else {
            /* TODO: if CoverLine.matches */
            req.description += &line;
        }

        // end of File
        if r == 0 {
            break;
        }
    }

    return Ok(results);
}

impl MarkdownArtefact<'_> {
    fn parse(&self) -> MarkdownParserResult {
        let p = self.path;
        let file = fs::File::open(p).map_err(|e| MarkdownParserError::IOError(p.into(), e))?;

        let reader = io::BufReader::new(file);

        let requirements = markdown_parse(reader, self.path.into())?;

        return Ok(requirements);
    }
}
/* }}} */

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new("REQUIREMENTS.md");
    let a = MarkdownArtefact { path };

    let reqs = a.parse()?;
    for r in reqs {
        eprint!("# {}: {}\n{}", r.id, r.title, r.description);
    }

    return Ok(());
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_req_regex_matches() {
        let cap = REQUIREMENT_LINE.captures("#### REQ-123: Tit le").unwrap();
        assert_eq!(&cap[1], "####");
        assert_eq!(&cap[2], "REQ-123");
        assert_eq!(&cap[3], "Tit le");
    }

    #[test]
    fn test_cover_regex_matches() {
        let cap = COVER_LINE.captures("Covers: REQ-123 (Title )").unwrap();
        assert_eq!(&cap[1], "REQ-123");
        assert_eq!(&cap[2], "Title ");
    }

    #[test]
    fn test_markdown_parser() {
        todo!()
    }
}
