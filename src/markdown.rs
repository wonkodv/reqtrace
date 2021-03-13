use std::io::BufRead;
use std::path::PathBuf;
use std::{collections::hash_map::Entry, io};

use regex::Regex;
use std::mem;

use super::common::*;

use ParserError::*;

lazy_static! {
    static ref HEADING_LINE: Regex = Regex::new(r"^(#+)").unwrap();
    static ref REQUIREMENT_LINE: Regex =
        Regex::new(r"^(#+)\s*([A-Za-z][a-zA-Z0-9_]+[a-zA-Z0-9]):\s*(.+)\s*$").unwrap();
    static ref ATTRIBUTE_LINE: Regex = Regex::new(r"^([A-Z][a-z]+):\s(.*)$").unwrap();
    static ref REF_LINK_LINE: Regex =
        Regex::new(r"^*\s+([A-Za-z][a-zA-Z0-9_]+[a-zA-Z0-9]):\s*(.+)\s*$").unwrap();
}

#[derive(Debug, PartialEq)]
enum MarkdownParserState<'a> {
    LookForReq,
    LookForDesc,
    CollectDesc,
    LookForAttr,
    CollectTextAttr(&'a mut String),
    CollectRefLink(&'a mut Vec<Reference>),
}
use MarkdownParserState::*;

#[derive(Debug)]
pub struct MarkdownParser<'parsing, R: io::Read> {
    reader: io::BufReader<R>,
    path: PathBuf,

    line_number: u32,            // current line number
    line_buffer: Option<String>, // line to use

    req: Requirement,                     // the requirement being built
    level: usize,                         // Heading level of current requirement
    state: MarkdownParserState<'parsing>, // State of the parser
}

const KEEP_PARSING: Result<bool, ParserError> = Ok(false);
const DONE_PARSING: Result<bool, ParserError> = Ok(true);

impl<'parsing, R: io::Read> MarkdownParser<'parsing, R> {
    pub fn new(read: R, path: PathBuf) -> Self {
        Self {
            reader: io::BufReader::new(read),
            path,
            line_number: 0,
            line_buffer: None,
            level: 0,
            state: LookForReq,
            req: Requirement::default(),
        }
    }

    fn location(&self) -> Location {
        Location::new(self.path.clone(), self.line_number)
    }

    fn peek(&self) -> &str {
        self.line_buffer.as_ref().unwrap()
    }

    fn consume(&mut self) -> String {
        self.line_buffer.take().unwrap()
    }

    fn parse_paragraph(&'parsing mut self, dest: &mut String) -> Result<bool, ParserError> {
        let mut line = self.peek();
        if let Some(heading_line) = HEADING_LINE.captures(line) {
            if REQUIREMENT_LINE.is_match(&line) {
                return DONE_PARSING;
            }
            let level = heading_line[1].len();
            if self.level <= level {
                return DONE_PARSING;
            } else {
                line = line[level..].trim_start();
            }
        } else if ATTRIBUTE_LINE.is_match(line) {
            return self.parse_attr();
        }

        dest.push_str(line);
        self.consume();
        return KEEP_PARSING;
    }

    fn parse_attr(&'parsing mut self) -> Result<bool, ParserError> {
        let line = self.consume();

        if let Some(attr_line) = ATTRIBUTE_LINE.captures(&line) {
            let key = &attr_line[1];
            let val = &attr_line[2];

            match key {
                // TODO  "Tags" => self.parse_tags(val);
                "Depends" => self.parse_link_attr(&mut self.req.depends, val),
                "Covers" => self.parse_link_attr(&mut self.req.covers, val),
                _ => {
                    let e = self.req.attributes.entry(key.to_owned());

                    match e {
                        Entry::Occupied(_) => {
                            return Err(DuplicateAttribute(
                                Location::new(self.path.clone(), self.line_number),
                                key.to_owned(),
                            ));
                        }
                        Entry::Vacant(e) => {
                            self.state = CollectTextAttr(e.insert(val.to_owned()));
                            return Ok(false);
                        }
                    }
                }
            }
        } else {
            Err(FormatError(self.location(), "Expected an Attribute header"))
        }
    }

    fn parse_link_attr(
        &self,
        ref_vec: &'parsing mut Vec<Reference>,
        short_list: &str,
    ) -> Result<bool, ParserError> {
        for id in short_list.split(",") {
            id = id.trim();
            ref_vec.push(Reference {
                id: id.to_owned(),
                title: None,
            });
        }
        self.state = CollectRefLink(ref_vec);
        return KEEP_PARSING;
    }

    fn parse_ref_link(&self, refVec: &'parsing mut Vec<Reference>) -> Result<bool, ParserError> {
        let line = self.peek();
        if let Some(ref_link) = REF_LINK_LINE.captures(line) {
            let id = ref_link[1].to_owned();
            let title = &ref_link[2];

            let title = if title.is_empty() {
                None
            } else {
                Some(title.to_owned())
            };

            refVec.push(Reference { id, title });
        } else if line.trim().is_empty() {
            self.state = LookForAttr;
        } else {
            return Err(FormatError(
                self.location(),
                "Expected a Reference like `* REQ_ID: Title`",
            ));
        }

        self.consume();
        return KEEP_PARSING;
    }

    fn parse_next(&'parsing mut self) -> Result<Option<Requirement>, ParserError> {
        loop {
            if self.line_buffer.is_none() {
                let buffer = String::new();
                let bytes_read = self
                    .reader
                    .read_line(&mut buffer)
                    .map_err(|e| IOError(self.path.clone(), e))?;

                if bytes_read > 0 {
                    self.line_number += 1;
                    self.line_buffer = Some(buffer);
                } else {
                    if self.state == LookForReq {
                        return Ok(None);
                    } else {
                        return Ok(Some(self.req));
                    }
                }
            }

            let done_err = match self.state {
                LookForReq => {
                    let l = self.consume();
                    if let Some(req_line) = REQUIREMENT_LINE.captures(&l) {
                        self.level = req_line[1].len();
                        self.req.id = req_line[2].to_owned();
                        self.req.title = Some(req_line[3].trim().to_owned());
                        self.req.location = self.location();

                        self.state = LookForDesc;
                    }

                    KEEP_PARSING
                }
                LookForDesc => {
                    let l = self.consume();
                    if !l.trim().is_empty() {
                        self.req.description = Some(l);
                        self.state = CollectDesc;
                    }

                    KEEP_PARSING
                }
                CollectDesc => self.parse_paragraph(self.req.description.as_mut().unwrap()),
                CollectTextAttr(s) => self.parse_paragraph(s),
                CollectRefLink(vec) => self.parse_ref_link(vec),
                LookForAttr => {
                    if self.peek().is_empty() {
                        self.consume();
                        KEEP_PARSING
                    } else {
                        self.parse_attr()
                    }
                }
            };

            match done_err {
                Ok(done) => {
                    if done {
                        return Ok(Some(mem::take(&mut self.req)));
                    }
                }
                Err(e) => {
                    self.state = LookForReq;
                    return Err(e);
                }
            }
        }
    }
}

impl<'a, R: io::Read> Iterator for &'a MarkdownParser<'a, R> {
    type Item = Result<Requirement, ParserError>;

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
