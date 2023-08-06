use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;
use std::rc::Rc;

use lazy_static::lazy_static;
use log::warn;
use regex::Regex;

use crate::common::Location;
use crate::common::Reference;
use crate::common::Requirement;
use crate::errors::Error;

use super::ArtefactConfig;

lazy_static! {
    static ref REF_LINK_LINE: Regex =
        Regex::new(r"=>\s*(\p{XID_Start}\p{XID_Continue}+)(?::\s*(.+?))?\s*$").unwrap();
}

#[derive(Debug)]
pub struct ReadmeParser {
    path: PathBuf,
}

impl super::Parser for ReadmeParser {
    fn parse(&mut self) -> (Vec<Rc<Requirement>>, Vec<Error>) {
        let file = fs::File::open(&self.path).map_err(|e| Error::Io((&self.path).into(), e));
        match file {
            Err(err) => {
                warn!("{}", err);
                (vec![], vec![err])
            }
            Ok(file) => {
                let r = io::BufReader::new(file);
                parse(&self.path, r)
            }
        }
    }
}

impl ReadmeParser {
    pub fn from_config(mut config: ArtefactConfig) -> Result<Self, Error> {
        assert!(config.parser == "readme");

        if config.paths.len() != 1 {
            return Err(Error::ArtefactTypeOnlyAllowsOnePath(
                config.parser,
                config.paths,
            ));
        }
        if config.parser_options.is_some() {
            return Err(Error::Config(
                "readme parser does not support options".into(),
            ));
        }

        let path = config.paths.remove(0).into();

        Ok(Self { path })
    }
}

pub fn parse<R: io::BufRead>(path: &Path, reader: R) -> (Vec<Rc<Requirement>>, Vec<Error>) {
    let mut errors = Vec::<Error>::new();
    let mut title = None;
    let mut depends = Vec::<Reference>::new();

    requirement_covered!(FMT_README, "Single Requirement Per File");

    for (no, line) in reader.lines().enumerate() {
        match line {
            Ok(line) => {
                if title.is_none() {
                    title = Some(line)
                } else if let Some(ref_link) = REF_LINK_LINE.captures(&line) {
                    let id = ref_link[1].to_owned();
                    let title = ref_link.get(2).map(|m| m.as_str().to_owned());
                    let location = Some(Location::new_with_line_no(path.to_path_buf(), no + 1));

                    let reference = Reference {
                        id,
                        title,
                        location,
                    };
                    depends.push(reference);
                }
            }

            Err(_) => todo!(),
        }
    }

    let id: String = {
        if let Some(stem) = path.file_stem() {
            stem.to_string_lossy().to_string()
        } else {
            errors.push(Error::Config("file for 'readme' parser has no stem".into()));
            "README".to_string()
        }
    };
    let location = Location::new_with_no_pos(path.to_path_buf());
    let requirement = Requirement {
        id,
        title,
        depends,
        location,
        ..Requirement::default()
    };
    let requirements = vec![Rc::new(requirement)];

    (requirements, errors)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_req_regex_matches_no_title() {
        let cap = REF_LINK_LINE.captures("Some text => REQ_ID").unwrap();
        assert_eq!(&cap[1], "REQ_ID");
        assert_eq!(cap.get(2), None);
    }
    #[test]
    fn test_req_regex_matches_title() {
        let cap = REF_LINK_LINE
            .captures("Some text => REQ_ID:     Req Title        ")
            .unwrap();
        assert_eq!(&cap[1], "REQ_ID");
        assert_eq!(&cap[2], "Req Title");
    }

    #[test]
    fn test_readme() {
        let text = "First Line
            second line
            some text => REQ_1
            some more text
            another requirement =>REQ_2: title
            ";

        let (requirements, errors) = parse(&PathBuf::from("path/to/Read_Me.md"), text.as_bytes());
        assert!(errors.is_empty());
        assert!(requirements.len() == 1);
        let r = &requirements[0];
        assert!(r.id == "Read_Me"); // from file name
        assert!(r.title == Some("First Line".to_owned()));
        dbg!(&r.depends);
        assert!(r.depends.len() == 2);
        assert!(r.depends[0].id == "REQ_1");
        assert!(r.depends[0].title.is_none());
        assert!(r.depends[1].id == "REQ_2");
        assert!(r.depends[1].title == Some("title".to_owned()));
    }
}
