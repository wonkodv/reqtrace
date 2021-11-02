use std::{collections::HashMap, io, rc::Rc};
use std::{io::BufRead, path::Path};

use lazy_static::lazy_static;
use regex::{Captures, Regex};

use super::super::common::*;
use super::*;

use crate::errors::Error;
use Error::*;

lazy_static! {
    static ref HEADING_LINE: Regex = Regex::new(r"^(#+)").unwrap();
    static ref REQUIREMENT_LINE: Regex =
        Regex::new(r"^(#+)\s*(\p{XID_Start}\p{XID_Continue}+):\s*(.+)\s*$").unwrap();
    static ref ATTRIBUTE_LINE: Regex = Regex::new(r"^([A-Z][a-z]+):\s(.*)\s*$").unwrap();
    static ref REF_LINK_LINE: Regex =
        Regex::new(r"^*\s+(\p{XID_Start}\p{XID_Continue}+)(?::\s*(.+))?\s*$").unwrap();
    static ref BAD_HEADLINE_UNDERLINE: Regex = Regex::new(r"^(====*)|(----*)").unwrap(); // TODO: use
}

#[derive(Debug)]
struct Context<'a> {
    errors: Vec<Error>,
    requirements: Vec<Rc<Requirement>>,
    req_under_construction: Option<Box<Requirement>>,
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
    Eof,
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

pub fn markdown_parse<R: io::Read>(reader: R, path: &Path) -> (Vec<Rc<Requirement>>, Vec<Error>) {
    let mut context = Context {
        path,
        errors: Vec::new(),
        requirements: Vec::new(),
        req_under_construction: None,
        line_number: 0,
        level: 0,
    };

    let mut reader = io::BufReader::new(reader);
    let mut line = String::new();
    let mut state = State::LookForReq;
    loop {
        let evt: Event<'_>;
        let req_line;

        line.clear();
        context.line_number += 1;
        match reader.read_line(&mut line) {
            Err(e) => {
                context.errors.push(IoError(context.path.to_owned(), e));
                break;
            }
            Ok(0) => {
                evt = Event::Eof;
            }
            Ok(1) => {
                assert!(line.starts_with('\n'));
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

        if let Event::Eof = evt {
            break;
        }
    }

    (context.requirements, context.errors)
}

fn parse_states<'a>(state: State, context: &mut Context<'_>, evt: &'a Event<'_>) -> State {
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
            add_req(context, req_line)
        }

        Event::Heading => match state {
            State::LookForReq => state,
            State::LookForDesc => State::LookForReq,
            State::CollectDesc(desc) => {
                commit_attr(context, ATTR_DESCRIPTION.to_owned(), desc);
                State::LookForReq
            }
            State::CollectDescNl(desc) => {
                commit_attr(context, ATTR_DESCRIPTION.to_owned(), desc);
                State::LookForReq
            }
            State::LookForAttr => State::LookForReq,
            State::CollectTextAttr(attr, val) => {
                commit_attr(context, attr, val);
                State::LookForReq
            }
            State::CollectRefLink(attr, vec) => {
                commit_link_attr(context, attr, vec);
                State::LookForReq
            }
        },

        Event::Eof => {
            match state {
                State::LookForReq => {}
                State::LookForDesc => {
                    maybe_commit_req(context);
                }
                State::CollectDesc(desc) => {
                    commit_attr(context, ATTR_DESCRIPTION.to_owned(), desc);
                    maybe_commit_req(context);
                }
                State::CollectDescNl(desc) => {
                    commit_attr(context, ATTR_DESCRIPTION.to_owned(), desc);
                    maybe_commit_req(context);
                }
                State::LookForAttr => {
                    maybe_commit_req(context);
                }
                State::CollectTextAttr(attr, val) => {
                    commit_attr(context, attr, val);
                    maybe_commit_req(context);
                }
                State::CollectRefLink(attr, vec) => {
                    commit_link_attr(context, attr, vec);
                    maybe_commit_req(context);
                }
            }
            State::LookForReq // does not matter
        }

        Event::Line(line) => match state {
            State::LookForReq => state,
            State::LookForDesc => {
                if let Some(attr_line) = ATTRIBUTE_LINE.captures(line) {
                    start_attribute(context, &attr_line)
                } else {
                    State::CollectDesc(line.to_owned().to_owned())
                }
            }
            State::CollectDesc(mut desc) => {
                desc.push_str(line);
                State::CollectDesc(desc)
            }
            State::CollectDescNl(desc) => {
                if let Some(attr_line) = ATTRIBUTE_LINE.captures(line) {
                    commit_attr(context, ATTR_DESCRIPTION.to_owned(), desc);
                    start_attribute(context, &attr_line)
                } else {
                    State::CollectDesc(desc)
                }
            }
            State::LookForAttr => {
                if let Some(attr_line) = ATTRIBUTE_LINE.captures(line) {
                    start_attribute(context, &attr_line)
                } else {
                    context.errors.push(FormatError(
                        context.location(),
                        "Expected an Attribute line like `Comment:`".into(),
                    ));
                    State::LookForReq
                }
            }
            State::CollectTextAttr(attr, mut val) => {
                if let Some(attr_line) = ATTRIBUTE_LINE.captures(line) {
                    commit_attr(context, attr, val);
                    start_attribute(context, &attr_line)
                } else {
                    val.push_str(line);
                    State::CollectTextAttr(attr, val)
                }
            }
            State::CollectRefLink(attr, mut vec) => {
                if let Some(ref_link) = REF_LINK_LINE.captures(line) {
                    let id = ref_link[1].to_owned();
                    let title = ref_link.get(2).map(|m| m.as_str().to_owned());

                    vec.push(Reference { id, title });
                    State::CollectRefLink(attr, vec)
                } else if line.trim().is_empty() {
                    commit_link_attr(context, attr, vec);
                    State::LookForAttr
                } else {
                    commit_link_attr(context, attr, vec);
                    context.errors.push(FormatError(
                        context.location(),
                        "Expected a Reference like `* REQ_ID: Title`".into(),
                    ));
                    State::LookForReq
                }
            }
        },

        Event::Empty => match state {
            State::LookForReq => state,
            State::LookForDesc => state,
            State::CollectDesc(mut desc) => {
                desc.push('\n');
                State::CollectDescNl(desc)
            }
            State::CollectDescNl(mut desc) => {
                desc.push('\n');
                State::CollectDescNl(desc)
            }
            State::LookForAttr => state,
            State::CollectTextAttr(attr, val) => {
                commit_attr(context, attr, val);
                State::LookForAttr
            }
            State::CollectRefLink(attr, vec) => {
                commit_link_attr(context, attr, vec);
                State::LookForAttr
            }
        },
    }
}

fn commit_attr(context: &mut Context<'_>, attr: String, val: String) {
    context
        .req_under_construction
        .as_mut()
        .unwrap()
        .attributes
        .insert(attr, val);
}

fn commit_link_attr(context: &mut Context<'_>, attr: String, vec: Vec<Reference>) {
    if attr == ATTR_COVERS {
        context.req_under_construction.as_mut().unwrap().covers = vec;
    } else if attr == ATTR_DEPENDS {
        context.req_under_construction.as_mut().unwrap().depends = vec;
    } else {
        panic!();
    }
}

fn maybe_commit_req(context: &mut Context<'_>) {
    if let Some(box_req) = context.req_under_construction.take() {
        context.requirements.push(Rc::from(box_req));
    }
}

fn add_req<'a>(context: &mut Context<'_>, req_line: &Captures<'a>) -> State {
    maybe_commit_req(context);
    context.level = req_line[1].len();
    let id = req_line[2].to_owned();
    let title = Some(req_line[3].trim().to_owned());
    let location = context.location();
    let r = Requirement {
        id,
        location,
        title,
        ..Requirement::default()
    };
    context.req_under_construction = Some(Box::new(r));

    State::LookForDesc
}

fn start_attribute<'a>(context: &mut Context<'_>, attr_line: &Captures<'a>) -> State {
    let attr = &attr_line[1];
    let first_line = &attr_line[2];
    match attr {
        // TODO  ATTR_TAGS => self.parse_tags(val);
        ATTR_COVERS => {
            if !context
                .req_under_construction
                .as_ref()
                .unwrap()
                .covers
                .is_empty()
            {
                context
                    .errors
                    .push(DuplicateAttribute(context.location(), attr.to_owned()));
            }
            parse_link_attr(attr, first_line)
        }
        ATTR_DEPENDS => {
            if !context
                .req_under_construction
                .as_ref()
                .unwrap()
                .depends
                .is_empty()
            {
                context
                    .errors
                    .push(DuplicateAttribute(context.location(), attr.to_owned()));
            }
            parse_link_attr(attr, first_line)
        }
        _ => {
            if context
                .req_under_construction
                .as_ref()
                .unwrap()
                .attributes
                .get(attr)
                .is_some()
            {
                context
                    .errors
                    .push(DuplicateAttribute(context.location(), attr.to_owned()));
            }
            State::CollectTextAttr(attr.to_owned(), first_line.to_owned())
        }
    }
}

fn parse_link_attr(attr: &str, short_list: &str) -> State {
    let mut vec = Vec::new();
    let short_list = short_list.trim();
    for id in short_list.split(',') {
        let id = id.trim();
        if !id.is_empty() {
            vec.push(Reference {
                id: id.to_owned(),
                title: None,
            });
        }
    }
    State::CollectRefLink(attr.to_owned(), vec)
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
    fn test_req_regex_unicode() {
        let cap = REQUIREMENT_LINE
            .captures("## ÄÅÉËÞÜÚÍÓÖÁÐFGHÏ: Allow Unicode IDs\n")
            .unwrap();
        assert_eq!(&cap[1], "##");
        assert_eq!(&cap[2], "ÄÅÉËÞÜÚÍÓÖÁÐFGHÏ");
    }

    #[test]
    fn test_req_regex_no_match_dash() {
        let cap = REQUIREMENT_LINE.captures("## REQ-ID: No Dash\n");
        assert!(cap.is_none());
    }

    #[test]
    fn test_req_regex_no_match_pound() {
        let cap = REQUIREMENT_LINE.captures("## REQ#ID: No Dash\n");
        assert!(cap.is_none());
    }

    #[test]
    fn test_req_regex_no_match_symbols() {
        let cap = REQUIREMENT_LINE.captures("## REQ🍔: No Burgers in requirement ids\n");
        assert!(cap.is_none());
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
*   COV3: Title 3
*   COV4

Depends:
*   DEP
*   DEP_WT: T
        "#;

        let p = Path::new("Test.md");
        let (reqs, errs) = markdown_parse(s.as_bytes(), &p);

        assert!(errs.is_empty());

        assert_eq!(reqs.len(), 1);

        let req = &reqs[0];

        assert_eq!(req.id, "REQ");
        assert_eq!(req.title, Some("Title Title".to_owned()));
        assert_eq!(req.covers.len(), 4);
        assert_eq!(req.covers[0].id, "COV");
        assert_eq!(req.covers[1].id, "Cov");
        assert_eq!(req.covers[2].id, "COV3");
        assert_eq!(req.covers[2].title, Some("Title 3".into()));
        assert_eq!(req.covers[3].id, "COV4");
        assert_eq!(req.depends.len(), 2);
        assert_eq!(req.depends[0].id, "DEP");
        assert_eq!(req.depends[1].id, "DEP_WT");
        assert_eq!(req.depends[1].title, Some("T".into()));
    }

    /// Regression test, Used to parse the rest of the line into 1 Requirement with empty ID
    #[test]
    fn test_markdown_parser_reflinks() {
        let s = r#"
## REQ: Title Title
Covers:
        "#;

        let p = Path::new("Test.md");
        let (reqs, errs) = markdown_parse(s.as_bytes(), &p);

        assert!(errs.is_empty());

        assert_eq!(reqs.len(), 1);

        let req = &reqs[0];

        assert_eq!(req.id, "REQ");
        assert_eq!(req.covers, vec![]);
    }
}
