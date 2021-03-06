use std::fmt;
use std::fs;
use std::io;
use std::io::BufRead;
use std::path::Path;
use std::path::PathBuf;

use regex::Regex;
use thiserror::Error;

use super::common::*;

lazy_static! {
    static ref HEADING_LINE: Regex = Regex::new(r"^(#+)").unwrap();
    static ref REQUIREMENT_LINE: Regex =
        Regex::new(r"^(#+)\s*([A-Za-z][a-zA-Z0-9_-]*):\s*(.+)\s*$").unwrap();
    static ref COVER_LINE: Regex =
        Regex::new(r"^Covers:\s*([A-Za-z][a-zA-Z0-9_-]*)\s*\(([^)]+)\)\s*$").unwrap();
    static ref BAD_HEADLINE_UNDERLINE: Regex = Regex::new(r"^(====*)|(----*)").unwrap();
}

#[allow(dead_code)]
#[derive(Error, Debug)] // TODO
pub enum MarkdownParserError {
    #[error("If `prefixes` is empty, require_prefix must be false")]
    DuplicateRequirement(Requirement,Requirement),

    #[error("If `prefixes` is empty, require_prefix must be false")]
    RequiredPrefixWithoutPrefix,

    #[error("Heading with Prefix not declaring a Requirement at {0}")]
    InvalidPrefix(Location),

    #[error("Nested Requirement at {0}")]
    IllegalNesting(Location),

    #[error("Bad Format: {1} at {0}")]
    FormatError(Location, &'static str),

    #[error("File Read error")]
    IOError(PathBuf, io::Error),
}

#[derive(Debug, PartialEq, Clone)]
pub struct MarkdownArtefact<'a> {
    path: &'a Path,
}

impl<'a> fmt::Display for MarkdownArtefact<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "MarkdownArtefact({})", self.path.display());
    }
}

impl<'a> MarkdownArtefact<'a> {
    pub fn new(path: &'a Path) -> Self {
        Self { path }
    }
}

impl Artefact for MarkdownArtefact<'_> {
    type ERROR = MarkdownParserError;

    fn parse(&self) -> Result<Requirements, Self::ERROR> {
        let p = self.path;
        let file = fs::File::open(p).map_err(|e| MarkdownParserError::IOError(p.into(), e))?;

        let reader = io::BufReader::new(file);

        let requirements = markdown_parse(reader, self.path.into())?;

        return Ok(requirements);
    }
}

fn markdown_parse<R: io::Read>(
    read: R,
    path: PathBuf,
) -> Result<Requirements, MarkdownParserError> {
    let mut reader = io::BufReader::new(read);
    let mut results: Requirements = Requirements::new();

    let mut line_number: u32 = 0;
    let mut line = String::new();

    let mut current_level: usize = 0;
    let mut req: Requirement = Requirement::default();
    /* TODO: figure out how ref works  and add req to list instead of cloning 
     *
     * Better yet, create an iterator on MarkdownArtefact that parses until it
     * finds, then yields.
     *
     *
     *
     * States:
     *
     * REQ         ---ReqHeading(new)-------> DESCRIPTION
     *
     * DESCRIPTION ---Text------------------> DESCRIPTION
     * DESCRIPTION ---Attr------------------> ATTRIBUTE
     *
     * DESCRIPTION ---Heading---------------> REQ
     * DESCRIPTION ---ReqHeading(emit,new)--> DESCRIPTION
     *
     * ATTRIBUTE   ---Attr------------------> ATTRIBUTE
     * ATTRIBUTE   ---Text(emit)------------> REQ
     * ATTR        ---Heading---------------> REQ
     * ATTR        ---ReqHeading(emit,new)--> DESCRIPTION
     *
     *
     *
     *
     **/

    req.location.file = path.clone();

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
                    if level == current_level {
                        results.push(req.clone());
                    } else if level > current_level {
                        /*  ### REQ
                         *  #### NESTED-REQ
                         */
                        return Err(MarkdownParserError::IllegalNesting(Location::new(
                            path,
                            line_number,
                        )));
                    } else if level < current_level {
                        /*  ### SORT-OF-NESTED-REQ
                         *  ## REQ
                         */
                        /* this is not strictly nesting but looks like it. needs a headline
                         * without req in between.
                         */
                        return Err(MarkdownParserError::IllegalNesting(Location::new(
                            path,
                            line_number,
                        )));
                    } else {
                        unreachable!();
                    }
                }

                req.title = title;
                req.id = id;
                req.description.clear();
                req.location.line = line_number;
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
                    if collecting {
                        collecting = false;
                        results.push(req.clone());
                    }
                }
            }
        } else if BAD_HEADLINE_UNDERLINE.is_match(&line) {
            return Err(MarkdownParserError::FormatError(
                Location::new(path, line_number),
                "Underlines for Headings confuse the parser",
            ));
        } else {
            /* TODO: if CoverLine.matches */
            req.description += &line;
        }

        // end of File
        if r == 0 {
            if collecting {
                results.push(req.clone());
            }
            break;
        }
    }

    return Ok(results);
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
