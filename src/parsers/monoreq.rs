use std::collections::BTreeMap;
use std::io;
use std::path::Path;
use std::rc::Rc;

use lazy_static::lazy_static;
use regex::Regex;

use crate::models::Error;
use crate::models::Location;
use crate::models::Reference;
use crate::models::Requirement;

lazy_static! {
    static ref REF_LINK_LINE: Regex =
        Regex::new(r"=>\s*(\p{XID_Start}\p{XID_Continue}+)(?::\s*(.+?))?\s*$").unwrap();
}

pub fn parse<R: io::BufRead>(reader: R, path: &Path) -> (Vec<Rc<Requirement>>, Vec<Error>) {
    let mut errors = Vec::<Error>::new();
    let mut title = None;
    let mut depends = Vec::<Reference>::new();

    requirement_covered!(FMT_IMPORT_MONO_REQ, "Single Requirement Per File");

    for (no, line) in reader.lines().enumerate() {
        match line {
            Ok(line) => {
                if title.is_none() {
                    title = Some(line);
                } else if let Some(ref_link) = REF_LINK_LINE.captures(&line) {
                    let id = ref_link[1].into();
                    let referenced_title = ref_link.get(2).map(|m| m.as_str().to_owned());
                    let location = Location::new_with_line_no(path.to_path_buf(), no + 1);

                    let reference = Reference {
                        id,
                        title: referenced_title,
                        location,
                    };
                    depends.push(reference);
                }
            }

            Err(_) => todo!(),
        }
    }

    let id = {
        if let Some(stem) = path.file_stem() {
            stem.to_string_lossy().to_string().into()
        } else {
            errors.push(Error::ArtefactConfig(format!(
                "file for 'MonoReq' parser `{} has no stem",
                path.display(),
            )));

            "README".into()
        }
    };
    let location = Location::new_with_no_pos(path.to_path_buf());
    let requirement = Requirement {
        id,
        title,
        location,
        depends,
        covers: vec![],
        tags: vec![],
        attributes: BTreeMap::new(),
    };
    let requirements = vec![Rc::new(requirement)];

    (requirements, errors)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

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

        let (requirements, errors) = parse(text.as_bytes(), &PathBuf::from("path/to/Read_Me.md"));
        assert!(errors.is_empty());
        assert!(requirements.len() == 1);
        let r = &requirements[0];
        assert!(r.id == "Read_Me".into()); // from file name
        assert!(r.title == Some("First Line".to_owned()));
        assert!(r.depends.len() == 2);
        assert!(r.depends[0].id == "REQ_1".into());
        assert!(r.depends[0].title.is_none());
        assert!(r.depends[1].id == "REQ_2".into());
        assert!(r.depends[1].title == Some("title".to_owned()));
    }
}
