use std::io;
use std::io::BufRead;
use std::mem;
use std::path::PathBuf;

use regex::Regex;

use super::common::*;

use ParserError::*;

lazy_static! {
    static ref HEADING_LINE: Regex = Regex::new(r"^(#+)").unwrap();
    static ref REQUIREMENT_LINE: Regex =
        Regex::new(r"^(#+)\s*([A-Za-z][a-zA-Z0-9_]+[a-zA-Z0-9]):\s*(.+)\s*$").unwrap();
    static ref ATTRIBUTE_LINE: Regex = Regex::new(r"^([A-Z][a-z]+):\s(.*)$").unwrap();
    static ref BAD_HEADLINE_UNDERLINE: Regex = Regex::new(r"^(====*)|(----*)").unwrap(); // TODO: use
}

#[derive(Debug, PartialEq)]
enum MarkdownParserState {
    LookForReq,
    CollectDesc,
    CollectAttr,
}
use MarkdownParserState::*;

#[derive(Debug)]
pub struct MarkdownParser<R: io::Read> {
    reader: io::BufReader<R>,
    path: PathBuf,
    line_number: u32,
    line: Option<String>,
}

impl<R: io::Read> MarkdownParser<R> {
    pub fn new(read: R, path: PathBuf) -> Self {
        Self {
            reader: io::BufReader::new(read),
            path,
            line_number: 0,
            line: None,
        }
    }

    fn parse_next(&mut self) -> Result<Option<Requirement>, ParserError> {
        let mut req = Requirement::default();
        let mut level = 0;
        let mut line_buffer: String = String::default();
        let mut state = LookForReq;

        let mut currently_appending_to: Option<&mut String> = None;

        let consume = loop {
            if let Some(l) = self.line.take() {
                line_buffer = l;
            } else {
                line_buffer.clear();
                let bytes_read = self
                    .reader
                    .read_line(&mut line_buffer)
                    .map_err(|e| IOError(self.path.clone(), e))?;
                if bytes_read == 0 {
                    break false;
                }
                self.line_number += 1;
            }

            let mut line = line_buffer.as_str();

            if state == LookForReq {
                if let Some(req_line) = REQUIREMENT_LINE.captures(line) {
                    level = req_line[1].len();
                    req.id = req_line[2].to_owned();
                    req.title = Some(req_line[3].trim().to_owned());
                    req.location.file = self.path.clone();
                    req.location.line = self.line_number;
                    state = CollectDesc;

                    currently_appending_to = None;
                }
            } else {
                if let Some(heading_line) = HEADING_LINE.captures(line) {
                    if REQUIREMENT_LINE.is_match(&line) {
                        break false;
                    }
                    let line_level = heading_line[1].len();
                    if line_level <= level {
                        break true;
                    } else {
                        line = line[level..].trim_start();
                    }
                }

                if let Some(attr_line) = ATTRIBUTE_LINE.captures(line) {
                    let key = &attr_line[1];
                    let val = &attr_line[2];

                    if key == "Tags" {
                        req.tags = val.split(",").map(|s| s.trim().to_owned()).collect();
                    } else if key == "Covers" {
                        req.covers = val
                            .split(",")
                            .map(|s| Reference {
                                id: s.trim().to_owned(),
                                title: None,
                            })
                            .collect();
                    } else if key == "Depends" {
                        req.depends = val
                            .split(",")
                            .map(|s| Reference {
                                id: s.trim().to_owned(),
                                title: None,
                            })
                            .collect();
                    } else {
                        let e = req.attributes.entry(key.to_owned());

                        match e {
                            std::collections::hash_map::Entry::Occupied(_) => {
                                return Err(DuplicateAttribute(
                                    Location::new(self.path.clone(), self.line_number),
                                    key.to_owned(),
                                ));
                            }
                            std::collections::hash_map::Entry::Vacant(_) => {
                                currently_appending_to = Some(e.or_insert(val.to_owned()));
                            }
                        }
                    }
                    state = CollectAttr;
                } else {
                    if let Some(ap) = currently_appending_to.as_mut() {
                        if ap.trim().is_empty() || !line.is_empty() {
                            ap.push_str(line);
                        }
                    } else {
                        assert!(state == CollectDesc);
                        if !line.is_empty() {
                            req.description = Some(mem::take(&mut line_buffer));
                            currently_appending_to = req.description.as_mut();
                        }
                    }
                }
            }
        };

        if !consume {
            self.line = Some(line_buffer);
        }

        if state != LookForReq {
            return Ok(Some(req));
        } else {
            return Ok(None);
        }
    }
}

impl<R: io::Read> Iterator for MarkdownParser<R> {
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
