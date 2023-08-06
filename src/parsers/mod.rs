use std::collections::BTreeMap;
use std::fs;
use std::mem;
use std::path::PathBuf;
use std::rc::Rc;

use log::*;
use serde::Deserialize;
use serde::Serialize;

use crate::common::Requirement;
use crate::errors::Error;

use self::markdown::MarkdownParser;
use self::readme::ReadmeParser;
use self::rust::RustParser;

mod markdown;
mod readme;
mod rust;

#[derive(Serialize, Deserialize, Debug)]
pub struct ArtefactConfig {
    pub paths: Vec<String>,
    pub parser: String,
    pub parser_options: Option<BTreeMap<String, String>>,
    pub version_provider: Option<String>,
}

#[derive(Debug, Default)]
pub struct ArtefactData {
    pub requirements: Vec<Rc<Requirement>>,
    pub id_to_req: BTreeMap<String, u16>, // ID => Req  with  ID
    pub id_to_covering_req: BTreeMap<String, Vec<(u16, u16)>>, // ID => Reqs where ID in Req.Covers
    pub id_to_depending_req: BTreeMap<String, Vec<(u16, u16)>>, // ID => Reqs where ID in Req.Depends
    pub errors: Vec<Error>,
}

pub trait Parser: std::fmt::Debug {
    fn parse(&mut self) -> (Vec<Rc<Requirement>>, Vec<Error>);
}

#[derive(Debug)]
pub struct ArtefactParser {
    parser: Box<dyn Parser>,
}

#[derive(Debug, Default)]
pub struct PrePopulated {
    requirements: Vec<Rc<Requirement>>,
    errors: Vec<Error>,
}

impl PrePopulated {
    /// Turn into ArtefactParser, usefull to write Tests on Artefacts
    pub fn into_artefact_parser(self) -> ArtefactParser {
        ArtefactParser {
            parser: Box::new(self),
        }
    }
}

impl Parser for PrePopulated {
    fn parse(&mut self) -> (Vec<Rc<Requirement>>, Vec<Error>) {
        (
            mem::take(&mut self.requirements),
            mem::take(&mut self.errors),
        )
    }
}

impl ArtefactParser {
    fn try_from_config(config: ArtefactConfig) -> Result<Box<dyn Parser>, Error> {
        let parser: Box<dyn Parser> = match config.parser.as_str() {
            "markdown" => Box::new(MarkdownParser::from_config(config)?),
            "rust" => Box::new(RustParser::from_config(config)?),
            "readme" => Box::new(ReadmeParser::from_config(config)?),
            _ => {
                return Err(Error::UnknownArtefactType(config.parser));
            }
        };
        Ok(parser)
    }
    pub fn from_config(config: ArtefactConfig) -> Self {
        match Self::try_from_config(config) {
            Ok(parser) => Self { parser },
            Err(err) => Self {
                parser: Box::new(PrePopulated {
                    requirements: vec![],
                    errors: vec![err],
                }),
            },
        }
    }

    pub fn parse(mut self) -> ArtefactData {
        let mut data = ArtefactData::default();
        let (requirements, errors) = self.parser.parse();
        data.requirements = requirements;
        data.errors = errors;

        for (req_idx, req) in data.requirements.iter().enumerate() {
            let old = data.id_to_req.insert(req.id.to_owned(), req_idx as u16);
            if let Some(old_idx) = old {
                let old_idx: usize = old_idx.into();

                /* Covers:  REQ_UNIQUE_ID: Requirements have a unique Identifier */
                let err = Error::DuplicateRequirement(
                    Rc::clone(&data.requirements[old_idx]),
                    Rc::clone(req),
                );

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
