use std::io;
use std::{io::BufRead, path::Path};

use regex::{Captures, Regex};

use super::common::*;

#[allow(unused_imports)]
use ParserError::*;

lazy_static! {
    static ref HEADING_LINE: Regex = Regex::new(r"^(#+)").unwrap();
    static ref REQUIREMENT_LINE: Regex =
        Regex::new(r"^(#+)\s*([A-Za-z][a-zA-Z0-9_]+[a-zA-Z0-9]):\s*(.+)\s*$").unwrap();
    static ref ATTRIBUTE_LINE: Regex = Regex::new(r"^([A-Z][a-z]+):\s(.*)$").unwrap();
    static ref REF_LINK_LINE: Regex =
        Regex::new(r"^*\s+([A-Za-z][a-zA-Z0-9_]+[a-zA-Z0-9]):\s*(.+)\s*$").unwrap();
    static ref BAD_HEADLINE_UNDERLINE: Regex = Regex::new(r"^(====*)|(----*)").unwrap(); // TODO: use
}

#[derive(Debug)]
struct Context<'a> {
    errors: Vec<ParserError>,
    requirements: Vec<Requirement>,
    path: &'a Path,
    line_number: u32, // current line number
    level: usize,     // Heading level of current requirement
}

impl<'a> Context<'a> {
    fn location(&self) -> Location {
        Location::new(self.path.to_owned(), self.line_number)
    }
}

#[derive(Debug)]
enum Event<'a> {
    EOF,
    Req(&'a Captures<'a>),
    Heading,
    Line(&'a str),
    Empty,
}

#[derive(Debug, PartialEq)]
enum State {
    LookForReq,
    LookForDesc,
    CollectDesc(String),
    CollectDescNl(String),
    LookForAttr,
    CollectTextAttr(String, String),
    CollectRefLink(String, Vec<Reference>),
}

pub fn markdown_parse<R: io::Read>(reader: R, path: &Path) -> (Vec<Requirement>, Vec<ParserError>) {
    let mut context = Context {
        path,
        errors: Vec::new(),
        requirements: Vec::new(),
        line_number: 0,
        level: 0,
    };

    let mut reader = io::BufReader::new(reader);
    let mut line = String::new();
    let mut state = State::LookForReq;
    loop {
        let evt: Event;
        let req_line;

        match reader.read_line(&mut line) {
            Err(e) => {
                context
                    .errors
                    .push(ParserError::IOError(context.path.to_owned(), e));
                break;
            }
            Ok(0) => {
                evt = Event::EOF;
            }
            Ok(1) => {
                evt = Event::Empty;
            } /* TODO: Win line Endings */
            Ok(_) => {
                if let Some(r) = REQUIREMENT_LINE.captures(&line) {
                    req_line = r;
                    evt = Event::Req(&req_line);
                } else if HEADING_LINE.is_match(&line) {
                    evt = Event::Heading;
                } else {
                    evt = Event::Line(&line);
                }
            }
        }

        parse_states(&mut state, &mut context, &evt);
        if let Event::EOF = evt {
            break;
        }
    }

    return (context.requirements, context.errors);
}

fn parse_states<'a>(state:&mut State, context: &mut Context, evt: &'a Event){
    match state {
        State::LookForReq => {
            match evt {
                Event::Req(req_line) => {
                    add_req(state, context, req_line);
                }
                _ => { /* ignore */ }
            }
        }
        State::LookForDesc => {
            match evt {
                Event::Req(req_line) => {
                    add_req(state, context, req_line);
                }
                Event::Heading => {
                    *state = State::LookForReq;
                }
                Event::Empty | Event::EOF => { /* ignore */ }
                Event::Line(line) => {
                    if let Some(attr_line) = ATTRIBUTE_LINE.captures(line) {
                        start_attribute(state, context, &attr_line);
                    } else {
                        *state = State::CollectDesc(line.to_owned().to_owned());
                    }
                }
            }
        }
        State::CollectDesc(ref mut desc) => match evt {
            Event::Req(req_line) => {
                commit_attr(context, ATTR_DESCRIPTION.to_owned(), *desc);
                add_req(state, context, req_line);
            }
            Event::Heading | Event::EOF => {
                commit_attr(context, ATTR_DESCRIPTION.to_owned(), *desc);
                *state = State::LookForReq;
            }
            Event::Line(line) => {
                desc.push_str(line);
            }
            Event::Empty => {
                desc.push_str("\n");
                *state = State::CollectDescNl(*desc);
            }
        },
        State::CollectDescNl(ref mut desc) => match evt {
            Event::Req(req_line) => {
                commit_attr(context, ATTR_DESCRIPTION.to_owned(), *desc);
                add_req(state, context, req_line);
            }
            Event::Heading | Event::EOF => {
                commit_attr(context, ATTR_DESCRIPTION.to_owned(), *desc);
                *state = State::LookForReq;
            }
            Event::Line(line) => {
                if let Some(attr_line) = ATTRIBUTE_LINE.captures(&line) {
                    commit_attr(context, ATTR_DESCRIPTION.to_owned(), *desc);
                    start_attribute(state, context, &attr_line);
                } else {
                    *state = State::CollectDesc(*desc);
                }
            }
            Event::Empty => {
                desc.push_str("\n");
            }
        },

        State::CollectTextAttr(ref attr, ref mut val) => match evt {
            Event::Req(req_line) => {
                commit_attr(context, *attr, *val);
                add_req(state, context, req_line);
            }
            Event::Heading | Event::EOF => {
                commit_attr(context, *attr, *val);
                *state = State::LookForReq;
            }
            Event::Line(line) => {
                if let Some(attr_line) = ATTRIBUTE_LINE.captures(&line) {
                    commit_attr(context, *attr, *val);
                    start_attribute(state, context, &attr_line);
                } else {
                    val.push_str(line);
                }
            }
            Event::Empty => {
                commit_attr(context, *attr, *val);
                *state = State::LookForAttr;
            }
        },
        State::CollectRefLink(ref attr, ref mut vec) => match evt {
            Event::Req(req_line) => {
                commit_link_attr(context, *attr, *vec);
                add_req(state, context, req_line);
            }
            Event::Heading | Event::EOF => {
                commit_link_attr(context, *attr, *vec);
                *state = State::LookForReq;
            }
            Event::Line(line) => {
                if let Some(ref_link) = REF_LINK_LINE.captures(line) {
                    let id = ref_link[1].to_owned();
                    let title = &ref_link[2];

                    let title = if title.is_empty() {
                        None
                    } else {
                        Some(title.to_owned())
                    };

                    vec.push(Reference { id, title });
                } else if line.trim().is_empty() {
                    commit_link_attr(context, *attr, *vec);
                    *state = State::LookForAttr;
                } else {
                    commit_link_attr(context, *attr, *vec);
                    context.errors.push(FormatError(
                        context.location(),
                        "Expected a Reference like `* REQ_ID: Title`",
                    ));
                    *state = State::LookForReq;
                }
            }
            Event::Empty => {
                commit_link_attr(context, *attr, *vec);
                *state = State::LookForAttr;
            }
        },

        State::LookForAttr => {
            match evt {
                Event::Req(req_line) => {
                    add_req(state, context, req_line);
                }
                Event::Heading | Event::EOF => {
                    *state = State::LookForReq;
                }
                Event::Line(line) => {
                    if let Some(attr_line) = ATTRIBUTE_LINE.captures(&line) {
                        start_attribute(state, context, &attr_line);
                    } else {
                        context.errors.push(FormatError(
                            context.location(),
                            "Expected an Attribute line like `Comment:`",
                        ));
                        *state = State::LookForReq;
                    }
                }
                Event::Empty => { /* ignore */ }
            }
        }
    };
}

fn commit_attr(context: &mut Context, attr: String, val: String) {
    context.requirements.last().unwrap().attributes.insert(attr, val);
}

fn commit_link_attr(context: &mut Context, attr: String, vec: Vec<Reference>) {
    if attr == ATTR_COVERS {
        context.requirements.last().unwrap().covers = vec;
    } else if attr == ATTR_DEPENDS {
        context.requirements.last().unwrap().depends = vec;
    } else {
        panic!();
    }
}

fn add_req<'a>(state:&mut State, context: &mut Context, req_line: &Captures<'a>) {
    context.level = req_line[1].len();
    let mut r = Requirement::default();
    r.id = req_line[2].to_owned();
    r.title = Some(req_line[3].trim().to_owned());
    r.location = context.location();
    context.requirements.push(r);
    *state = State::LookForDesc;
}

fn start_attribute<'a>(state:&mut State, context: &mut Context, attr_line: &Captures<'a>) {
    let attr = &attr_line[1];
    let first_line = &attr_line[2];
    match attr {
        // TODO  ATTR_TAGS => self.parse_tags(val);
        ATTR_COVERS => {
            if !context.requirements.last().unwrap().covers.is_empty() {
                context.errors.push(ParserError::DuplicateAttribute(
                    context.location(),
                    attr.to_owned(),
                ));
            }
            parse_link_attr(state, attr, first_line);
        }
        ATTR_DEPENDS => {
            if !context.requirements.last().unwrap().depends.is_empty() {
                context.errors.push(ParserError::DuplicateAttribute(
                    context.location(),
                    attr.to_owned(),
                ));
            }
            parse_link_attr(state, attr, first_line);
        }
        _ => {
            if let Some(_) = context.requirements.last().unwrap().attributes.get(attr) {
                context.errors.push(ParserError::DuplicateAttribute(
                    context.location(),
                    attr.to_owned(),
                ));
            }
            *state = State::CollectTextAttr(attr.to_owned(), first_line.to_owned());
        }
    }
}

fn parse_link_attr<'a>(state:&mut State, attr: &str, short_list: &str) {
    let vec = Vec::new();
    for id in short_list.split(",") {
        let id = id.trim();
        vec.push(Reference {
            id: id.to_owned(),
            title: None,
        });
    }
    *state = State::CollectRefLink(attr.to_owned(), vec);
}





#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

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
    fn test_attr_regex_matches() {
        let cap = ATTRIBUTE_LINE.captures("Covers: COV, Cov\n").unwrap();
        assert_eq!(&cap[1], "Covers");
        assert_eq!(&cap[2], "COV, Cov");
    }

    #[test]
    fn test_markdown_parser() {
        let s = r#"
## REQ: Title Title

Description

Descriotion

Covers: COV, Cov

Depends: DEP
        "#;
        let mut parser = MarkdownParser::new(s.as_bytes(), PathBuf::from("test"));

        let req = parser.next().unwrap().unwrap();
        assert_eq!(req.id, "REQ");
        assert_eq!(req.title, Some("Title Title".to_owned()));
        assert_eq!(req.covers.len(), 2);
        assert_eq!(req.covers[0].id, "COV");
        assert_eq!(req.covers[1].id, "Cov");
    }
}
