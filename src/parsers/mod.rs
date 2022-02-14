use std::collections::HashMap;
use std::path::PathBuf;
use std::rc::Rc;
use std::{fs, io};

use log::*;
use serde::Deserialize;
use serde::Serialize;

use crate::common::Requirement;
use crate::errors::Error::{self, *};
use crate::parsers::markdown::markdown_parse;
use crate::util::glob_paths;

pub mod markdown;
pub mod rust;

#[derive(Serialize, Deserialize, Debug)]
pub struct ArtefactConfig {
    pub paths: Vec<String>,
    pub parser: String,
    pub parser_options: Option<HashMap<String, String>>,
    pub version_provider: Option<String>,
}

#[derive(Debug, Default)]
pub struct ArtefactData {
    pub requirements: Vec<Rc<Requirement>>,
    pub id_to_req: HashMap<String, u16>, // ID => Req  with  ID
    pub id_to_covering_req: HashMap<String, Vec<(u16, u16)>>, // ID => Reqs where ID in Req.Covers
    pub id_to_depending_req: HashMap<String, Vec<(u16, u16)>>, // ID => Reqs where ID in Req.Depends
    pub errors: Vec<Error>,
}

#[derive(Debug)]
pub enum ArtefactParser {
    Markdown(PathBuf),
    Rust(Vec<PathBuf>),
    PrePopulated(Vec<Rc<Requirement>>),
    Error(Error),
}

impl ArtefactParser {
    pub fn from_config(config: ArtefactConfig) -> Self {
        let mut config = config;
        match config.parser.as_str() {
            "markdown" => {
                if config.paths.len() != 1 {
                    return ArtefactParser::Error(ArtefactTypeOnlyAllowsOnePath(
                        config.parser,
                        config.paths,
                    ));
                }
                if config.parser_options.is_some() {
                    return ArtefactParser::Error(ConfigError(
                        "markdown parser does not support options".into(),
                    ));
                }
                ArtefactParser::Markdown(PathBuf::from(config.paths.remove(0)))
            }
            "rust" => {
                let paths = glob_paths(&config.paths);
                match paths {
                    Ok(paths) => ArtefactParser::Rust(paths),
                    Err(err) => ArtefactParser::Error(err),
                }
            }
            _ => {
                error!("Unknown Artefact Type {}", &config.parser);
                ArtefactParser::Error(UnknownArtefactType(config.parser))
            }
        }
    }

    pub fn parse(self) -> ArtefactData {
        let mut data = ArtefactData::default();
        match self {
            ArtefactParser::Markdown(path) => {
                let file = fs::File::open(&path).map_err(|e| IoError((&path).into(), e));
                match file {
                    Err(err) => {
                        warn!("{}", err);
                        data.errors.push(err);
                    }
                    Ok(file) => {
                        let mut r = io::BufReader::new(file);
                        let (mut s, mut e) = markdown_parse(&mut r, &path);
                        trace!(
                            "parsed {}: {} errors, {} requirements",
                            path.display(),
                            e.len(),
                            s.len()
                        );
                        data.errors.append(&mut e);
                        data.requirements.append(&mut s);
                    }
                }
            }

            ArtefactParser::Rust(paths) => {
                for path in &paths {
                    let file = fs::File::open(path).map_err(|e| IoError(path.into(), e));
                    match file {
                        Err(err) => {
                            warn!("{}", err);
                            data.errors.push(err);
                        }
                        Ok(file) => {
                            let mut r = io::BufReader::new(file);
                            let (mut s, mut e) = rust::parse(&mut r, &path);
                            trace!(
                                "parsed {}: {} errors, {} requirements",
                                path.display(),
                                e.len(),
                                s.len()
                            );
                            data.errors.append(&mut e);
                            data.requirements.append(&mut s);
                        }
                    }
                }
            }

            ArtefactParser::PrePopulated(vec) => {
                data.requirements = vec;
            }
            ArtefactParser::Error(err) => {
                data.errors.push(err);
            }
        }

        for (req_idx, req) in data.requirements.iter().enumerate() {
            let old = data.id_to_req.insert(req.id.to_owned(), req_idx as u16);
            if let Some(old_idx) = old {
                let old_idx: usize = old_idx.into();

                /* Covers:  REQ_UNIQUE_ID: Requirements have a unique Identifier */
                let err =
                    DuplicateRequirement(Rc::clone(&data.requirements[old_idx]), Rc::clone(req));

                warn!("{}", err);

                data.errors.push(err);
            }

            for (cov_idx, cov) in req.covers.iter().enumerate() {
                data.id_to_covering_req
                    .entry(cov.id.to_owned())
                    .or_default()
                    .push((req_idx as u16, cov_idx as u16))
            }
            for (dep_idx, dep) in req.depends.iter().enumerate() {
                data.id_to_depending_req
                    .entry(dep.id.to_owned())
                    .or_default()
                    .push((req_idx as u16, dep_idx as u16))
            }
        }

        data
    }
}
