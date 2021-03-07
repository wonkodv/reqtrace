use std::fs;
use std::io;
use std::io::BufRead;
use std::path::Path;
use std::path::PathBuf;
use std::{fmt, mem};

use regex::Regex;
use thiserror::Error;

use super::common::*;

lazy_static! {
    static ref HEADING_LINE: Regex = Regex::new(r"^(#+)").unwrap();
    static ref REQUIREMENT_LINE: Regex =
        Regex::new(r"^(#+)\s*([A-Za-z][a-zA-Z0-9_]+[a-zA-Z0-9]):\s*(.+)\s*$").unwrap();
    static ref ATTRIBUTE_LINE: Regex = Regex::new(r"^([A-Z][a-z]+):\s(.*)$").unwrap();
    static ref REF_LINK_LINE: Regex = 
        Regex::new(r"^*\s+([A-Za-z][a-zA-Z0-9_]+[a-zA-Z0-9]):\s*(.+)\s*$").unwrap();
    static ref BAD_HEADLINE_UNDERLINE: Regex = Regex::new(r"^(====*)|(----*)").unwrap(); // TODO: use
}

#[allow(dead_code)] // TODO
#[derive(Error, Debug)]
pub enum MarkdownParserError {
    #[error("If `prefixes` is empty, require_prefix must be false")]
    DuplicateRequirement(Requirement, Requirement),

    #[error("If `prefixes` is empty, require_prefix must be false")]
    RequiredPrefixWithoutPrefix,

    #[error("Heading with Prefix not declaring a Requirement at {0}")]
    InvalidPrefix(Location),

    #[error("Nested Requirement at {0}")]
    IllegalNesting(Location),

    #[error("Bad Format: {1} at {0}")]
    FormatError(Location, &'static str),

    #[error("Duplicate Attribute: {1} at {0}")]
    DiplicateAttribute(Location, String),

    #[error("File Read error")]
    IOError(PathBuf, io::Error),

    #[error("End of File internal Error")]
    EOF,
}
use MarkdownParserError::*;

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
    type Error = MarkdownParserError;
    type Parser = MarkdownParser<fs::File>;

    fn get_parser(&self) -> Result<Self::Parser, Self::Error> {
        let p = self.path;
        let file = fs::File::open(p).map_err(|e| MarkdownParserError::IOError(p.into(), e))?;

        Ok(Self::Parser::new(file, p.into()))
    }
}

#[derive(Debug, PartialEq)]
enum MarkdownParserState<'a> {
    LookForReq,
    LookForDesc,
    CollectDesc,
    LookForAttr,
    CollectTextAttr(&mut String),
    CollectRefLink(&mut Vec),
}
use MarkdownParserState::*;

#[derive(Debug)]
pub struct MarkdownParser<R: io::Read> {
    reader: io::BufReader<R>,
    path: PathBuf,
    line_number: u32,
    line_buffer: Option<String>,

    req : Requirement,
    level:usize = 0,
    state:MarkdownParserState,
}



impl<R: io::Read> MarkdownParser<R> {
    fn new(read: R, path: PathBuf) -> Self {
        Self {
            reader: io::BufReader::new(read),
            path,
            line_number: 0,
            line: None,
        }
    }

    fn location(&self) -> Location {
        Location::new(self.path.clone(), self.line_number),
    }

    fn peek(&mut self) -> &String {
        self.line_buffer.as_ref().unwrap()
    }

    fn consume(&mut self) -> String {
        self.line_buffer.take().unwrap()
    }


    fn parse_paragraph(&self, dest: &mut String) -> Result<bool, MarkdownParserError> {
        let line = self.peek();
        if let Some(heading_line) = HEADING_LINE.captures(line) {
            if REQUIREMENT_LINE.is_match(&line) {
                return true;
            }
            self.line_level = heading_line[1].len();
            if self.line_level <= level {
                return true;
            } else {
                line = line[level..].trim_start();
            }
        } else if ATTRIBUTE_LINE.is_match(line) {
            return self.parse_attr();
        }

        dest += line;
        self.consume();
        return false
    }

    fn parse_attr(&self) -> Result<bool, MarkdownParserError> {
        let line = self.consume();

        if let Some(attr_line) = ATTRIBUTE_LINE.captures(&line) {
            let key = &attr_line[1];
            let val = &attr_line[2];

            match key {
              // TODO  "Tags" => self.parse_tags(val);
                "Depends" => self.parse_link_attr(&mut self.depends, val);
                "Covers" => self.parse_link_attr(&mut self.covers, val);
                _ => {
                    let e = self.req.attributes.entry(key.to_owned());

                    match e {
                        std::collections::hash_map::Entry::Occupied(_) => {
                            return Err(DiplicateAttribute(
                                    Location::new(self.path.clone(), self.line_number),
                                    key.to_owned(),
                            ));
                        }
                        std::collections::hash_map::Entry::Vacant(_) => {
                            self.state = CollectTextAttr(e.or_insert(val.to_owned()));
                        }
                    }
                }
        } else {
            Err(FormatError(self.location(), "Expected an Attribute header"));
        }
    }

    fn parse_link_attr(&self, &mut refVec, shortList:String) -> Result<bool, MarkdownParserError> {
        state = CollectRefLink(refVec);

        for id in shortList.split(",") {
            id = id.trim();
            refVec.push(Reference{id, title:None});
        }
        return Ok(false);
    }

    fn parse_ref_link(&self, &mut refVec) -> Result<bool, MarkdownParserError> {
        let line = self.peek();
        if let Some(ref_link) = REF_LINK_LINE.captures(line) {
            let id = ref_link[1].to_owned();
            let title = ref_link[2];

            let title = if title.is_empty {
                None
            } else {
                Some(title)
            }

            refVec.push(Reference{...});
        }
    }

    fn parse_next(&mut self) -> Result<Option<Requirement>, MarkdownParserError> {
        loop {
            let line : &str;
            if let Some(l) = self.line_buffer.as_ref() {
                line = l;
            } else {
                let buffer = String::new();
                let bytes_read = self
                    .reader
                    .read_line(&mut buffer)
                    .map_err(|e| IOError(self.path.clone(), e))?;
                if bytes_read > 0 {
                    self.line_number += 1;
                    self.line_buffer = Some(buffer);
                    line = self.line_buffer.as_ref().unwrap());
                } else {
                    break true;
                }
            }

            let done = match state {
                LookForReq =>  {
                    let l = self.consume();
                    if let Some(req_line) = REQUIREMENT_LINE.captures(&l) {
                        self.level = req_line[1].len();
                        req.id = req_line[2].to_owned();
                        req.title = Some(req_line[3].trim().to_owned());
                        req.location = self.location();

                        self.state = LookForDesc;
                    }

                    false
                },
                LookForDesc => {
                    let l = self.consume()
                    if !l.trim().is_empty() {
                        self.description = Some(l);
                        self.state =  CollectDesc;
                    }

                    false
                },
                CollectDesc => self.parse_paragraph(self.description.as_mut().unwrap())?,
                CollectTextAttr(s) => self.parse_paragraph(s)?,
                CollectRefLink(vec) => self.parse_ref_link(vec)?,
                _ => {},
            }

            if done {
                return Ok(Some(req));
            }
        };
    }
}

impl<R: io::Read> Iterator for MarkdownParser<R> {
    type Item = Result<Requirement, MarkdownParserError>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.parse_next() {
            Ok(Some(r)) => Some(Ok(r)),
            Ok(None) => None,
            Err(e) => Some(Err(e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_req_regex_matches() {
        let cap = REQUIREMENT_LINE
            .captures("## REQ_VCS: Allow Version Control\n")
            .unwrap();
        assert_eq!(&cap[1], "##");
        assert_eq!(&cap[2], "REQ_VCS");
        assert_eq!(&cap[3], "Allow Version Control");
    }

    #[test]
    fn test_markdown_parser() {
        todo!()
    }
}
