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
    static ref ATTRIBUTE_LINE: Regex = Regex::new(r"^([A-Z][a-z]+):\s(.*)\s*$").unwrap();
    static ref REF_LINK_LINE: Regex =
        Regex::new(r"^*\s+([A-Za-z][a-zA-Z0-9_]+[a-zA-Z0-9])(?::\s*(.+))?\s*$").unwrap();
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

        line.clear();
        context.line_number += 1;
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
                assert!(line.starts_with("\n"));
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

        state = parse_states(state, &mut context, &evt);

        if let Event::EOF = evt {
            break;
        }
    }

    return (context.requirements, context.errors);
}

fn parse_states<'a>(state: State, context: &mut Context, evt: &'a Event) -> State {
    match evt {
        Event::Req(req_line) => {
            match state {
                State::CollectDesc(desc) => {
                    commit_attr(context, ATTR_DESCRIPTION.to_owned(), desc);
                }
                State::CollectDescNl(desc) => {
                    commit_attr(context, ATTR_DESCRIPTION.to_owned(), desc);
                }
                State::CollectTextAttr(attr, val) => {
                    commit_attr(context, attr, val);
                }
                State::CollectRefLink(attr, vec) => {
                    commit_link_attr(context, attr, vec);
                }
                _ => {}
            }
            return add_req(context, req_line);
        }

        Event::Heading => match state {
            State::LookForReq => {
                return state;
            }
            State::LookForDesc => {
                return State::LookForReq;
            }
            State::CollectDesc(desc) => {
                commit_attr(context, ATTR_DESCRIPTION.to_owned(), desc);
                return State::LookForReq;
            }
            State::CollectDescNl(desc) => {
                commit_attr(context, ATTR_DESCRIPTION.to_owned(), desc);
                return State::LookForReq;
            }
            State::LookForAttr => {
                return State::LookForReq;
            }
            State::CollectTextAttr(attr, val) => {
                commit_attr(context, attr, val);
                return State::LookForReq;
            }
            State::CollectRefLink(attr, vec) => {
                commit_link_attr(context, attr, vec);
                return State::LookForReq;
            }
        },

        Event::EOF => {
            match state {
                State::LookForReq => {}
                State::LookForDesc => {}
                State::CollectDesc(desc) => {
                    commit_attr(context, ATTR_DESCRIPTION.to_owned(), desc);
                }
                State::CollectDescNl(desc) => {
                    commit_attr(context, ATTR_DESCRIPTION.to_owned(), desc);
                }
                State::LookForAttr => {}
                State::CollectTextAttr(attr, val) => {
                    commit_attr(context, attr, val);
                }
                State::CollectRefLink(attr, vec) => {
                    commit_link_attr(context, attr, vec);
                }
            }
            return State::LookForReq; // does not matter
        }

        Event::Line(line) => match state {
            State::LookForReq => {
                return state;
            }
            State::LookForDesc => {
                if let Some(attr_line) = ATTRIBUTE_LINE.captures(line) {
                    return start_attribute(context, &attr_line);
                } else {
                    return State::CollectDesc(line.to_owned().to_owned());
                }
            }
            State::CollectDesc(mut desc) => {
                desc.push_str(line);
                return State::CollectDesc(desc);
            }
            State::CollectDescNl(desc) => {
                if let Some(attr_line) = ATTRIBUTE_LINE.captures(&line) {
                    commit_attr(context, ATTR_DESCRIPTION.to_owned(), desc);
                    return start_attribute(context, &attr_line);
                } else {
                    return State::CollectDesc(desc);
                }
            }
            State::LookForAttr => {
                if let Some(attr_line) = ATTRIBUTE_LINE.captures(&line) {
                    return start_attribute(context, &attr_line);
                } else {
                    context.errors.push(FormatError(
                        context.location(),
                        "Expected an Attribute line like `Comment:`",
                    ));
                    return State::LookForReq;
                }
            }
            State::CollectTextAttr(attr, mut val) => {
                if let Some(attr_line) = ATTRIBUTE_LINE.captures(&line) {
                    commit_attr(context, attr, val);
                    return start_attribute(context, &attr_line);
                } else {
                    val.push_str(line);
                    return State::CollectTextAttr(attr, val);
                }
            }
            State::CollectRefLink(attr, mut vec) => {
                if let Some(ref_link) = REF_LINK_LINE.captures(line) {
                    let id = ref_link[1].to_owned();
                    let title = ref_link.get(2).map(|m| m.as_str().to_owned());

                    vec.push(Reference { id, title });
                    return State::CollectRefLink(attr, vec);
                } else if line.trim().is_empty() {
                    commit_link_attr(context, attr, vec);
                    return State::LookForAttr;
                } else {
                    commit_link_attr(context, attr, vec);
                    context.errors.push(FormatError(
                        context.location(),
                        "Expected a Reference like `* REQ_ID: Title`",
                    ));
                    return State::LookForReq;
                }
            }
        },

        Event::Empty => match state {
            State::LookForReq => {
                return state;
            }
            State::LookForDesc => {
                return state;
            }
            State::CollectDesc(mut desc) => {
                desc.push_str("\n");
                return State::CollectDescNl(desc);
            }
            State::CollectDescNl(mut desc) => {
                desc.push_str("\n");
                return State::CollectDescNl(desc);
            }
            State::LookForAttr => {
                return state;
            }
            State::CollectTextAttr(attr, val) => {
                commit_attr(context, attr, val);
                return State::LookForAttr;
            }
            State::CollectRefLink(attr, vec) => {
                commit_link_attr(context, attr, vec);
                return State::LookForAttr;
            }
        },
    };
}

fn commit_attr(context: &mut Context, attr: String, val: String) {
    context
        .requirements
        .last_mut()
        .unwrap()
        .attributes
        .insert(attr, val);
}

fn commit_link_attr(context: &mut Context, attr: String, vec: Vec<Reference>) {
    if attr == ATTR_COVERS {
        context.requirements.last_mut().unwrap().covers = vec;
    } else if attr == ATTR_DEPENDS {
        context.requirements.last_mut().unwrap().depends = vec;
    } else {
        panic!();
    }
}

fn add_req<'a>(context: &mut Context, req_line: &Captures<'a>) -> State {
    context.level = req_line[1].len();
    let mut r = Requirement::default();
    r.id = req_line[2].to_owned();
    r.title = Some(req_line[3].trim().to_owned());
    r.location = context.location();
    context.requirements.push(r);

    return State::LookForDesc;
}

fn start_attribute<'a>(context: &mut Context, attr_line: &Captures<'a>) -> State {
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
            return parse_link_attr(attr, first_line);
        }
        ATTR_DEPENDS => {
            if !context.requirements.last().unwrap().depends.is_empty() {
                context.errors.push(ParserError::DuplicateAttribute(
                    context.location(),
                    attr.to_owned(),
                ));
            }
            return parse_link_attr(attr, first_line);
        }
        _ => {
            if let Some(_) = context.requirements.last().unwrap().attributes.get(attr) {
                context.errors.push(ParserError::DuplicateAttribute(
                    context.location(),
                    attr.to_owned(),
                ));
            }
            return State::CollectTextAttr(attr.to_owned(), first_line.to_owned());
        }
    }
}

fn parse_link_attr<'a>(attr: &str, short_list: &str) -> State {
    let mut vec = Vec::new();
    for id in short_list.split(",") {
        let id = id.trim();
        vec.push(Reference {
            id: id.to_owned(),
            title: None,
        });
    }
    return State::CollectRefLink(attr.to_owned(), vec);
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
    fn test_attr_regex_matches() {
        let cap = ATTRIBUTE_LINE.captures("Covers: COV, Cov\n").unwrap();
        assert_eq!(&cap[1], "Covers");
        assert_eq!(&cap[2], "COV, Cov");
    }

    #[test]
    fn test_reflink_regex_matches() {
        let cap = REF_LINK_LINE.captures("*   REQ: Title of req \n").unwrap();
        assert_eq!(&cap[1], "REQ");
        assert_eq!(&cap[2], "Title of req ");

        let cap = REF_LINK_LINE.captures("*   REQ \n").unwrap();
        assert_eq!(&cap[1], "REQ");
        assert_eq!(cap.get(2), None);
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

        let p = Path::new("Test.md");
        let (reqs, errs) = markdown_parse(s.as_bytes(), &p);

        assert!(errs.is_empty());

        assert_eq!(reqs.len(), 1);

        let req = &reqs[0];

        assert_eq!(req.id, "REQ");
        assert_eq!(req.title, Some("Title Title".to_owned()));
        assert_eq!(req.covers.len(), 2);
        assert_eq!(req.covers[0].id, "COV");
        assert_eq!(req.covers[1].id, "Cov");
    }
}
