use crate::{
    common::{Artefact, ArtefactConfig},
    trace::{errors::ConfigError, Graph},
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, io, path::PathBuf};

pub enum Query<'a> {
    Full,
    ValidateGraph,
    EdgeTrace { from: &'a str, to: &'a str },
    Parse { artefact: &'a str },
    PrintRequirements,
}

pub enum Format {
    MarkdownTmx,
    Ctags,
}

pub struct Job<'a> {
    pub query: Query<'a>,
    pub format: Format,
}

#[derive(Serialize, Deserialize)]
struct AC {
    paths: Vec<PathBuf>,
    parser: String,
    parser_options: Option<String>,
}

#[derive(Debug)]
pub enum ControllerLoadError<'a> {
    OnlyOnePathExpected,
    UnknownArtefactType(&'a str),
    ConfigError(ConfigError<'a>),
}

impl<'a> From<ConfigError<'a>> for ControllerLoadError<'a> {
    fn from(e: ConfigError<'a>) -> Self {
        ControllerLoadError::ConfigError(e)
    }
}

impl AC {
    fn to_artefact_config<'c>(&'c self) -> Result<ArtefactConfig<'c>, ControllerLoadError<'c>> {
        match self.parser.as_str() {
            "markdown" => {
                if self.paths.len() != 1 {
                    return Err(ControllerLoadError::OnlyOnePathExpected);
                }
                if self.parser_options.is_some() {
                    todo!();
                }
                Ok(ArtefactConfig::Markdown(&self.paths[0]))
            }
            "rust" | "rust_unsafe" => {
                Ok(ArtefactConfig::PrePopulated(vec![])) // TODO
            }
            x => Err(ControllerLoadError::UnknownArtefactType(x)),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    artefacts: HashMap<String, AC>,
}

pub struct Controller<'config> {
    config: &'config Config,
    graph: Graph<'config>,
    success: bool,
}

impl<'c> Controller<'c> {
    pub fn new(config: &'c Config) -> Self {
        let graph = Graph::new();
        Self {
            config,
            graph,
            success: true,
        }
    }

    pub fn load(&mut self) -> Result<(), ControllerLoadError> {
        for (id, ac) in &self.config.artefacts {
            let ac = ac.to_artefact_config()?;
            let a = Artefact::new(id.as_str(), ac);
            self.graph.add_artefact(a)?;
        }
        Ok(())
    }

    pub fn run(&mut self, job: &str) {
        match job {
            "tags" => {
                let r = self.graph.get_all_reqs();
                crate::formatters::tags::requirements_ctags(r, &mut io::stdout().lock()).unwrap();
            }
            "tmx" => {
                todo!();
            }
            _ => {
                self.success = false;
            }
        }
        print!("Running {}", job);
        todo!();
    }

    pub fn success(&self) -> bool {
        self.success
    }
}
