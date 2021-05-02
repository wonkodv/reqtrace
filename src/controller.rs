use crate::{
    common::{Artefact, ArtefactConfig, Format},
    formatters,
    trace::{errors::ConfigError, Graph},
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, convert::TryFrom, io, path::PathBuf};

use anyhow::bail;
use anyhow::Error;

#[derive(Serialize, Deserialize)]
struct ArtefactConfigSerialized {
    paths: Vec<PathBuf>,
    parser: String,
    parser_options: Option<serde_json::Value>,
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

impl ArtefactConfigSerialized {
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
            "rust_cov_marks" | "rust_unsafe" => {
                Ok(ArtefactConfig::PrePopulated(vec![])) // TODO
            }
            x => Err(ControllerLoadError::UnknownArtefactType(x)),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    artefacts: HashMap<String, ArtefactConfigSerialized>,
    tracing: Vec<(String, Vec<String>)>,
    jobs: Option<HashMap<String, Job>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Query {
    /// Validate the Graph without parsing Artefacts
    ValidateGraph,

    /// Query Artefacts and Tracing Edges to see which are outdated
    CacheStatus,

    /// Parse a list of artefacts or all artefacts if empty
    Parse { artefacts: Vec<String> },

    /// Trace all Edges in Graph
    Trace,

    /// Analise a Single Requirement
    ShowRequirement { id: String },

    /// Show all Requirements which are below the passed one
    ShowRequirementImpact { id: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Job {
    pub query: Query,
    pub format: Format,
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

    pub fn find_job(&self, job: &str) -> Option<Job> {
        Some(self.config.jobs.as_ref()?.get(job)?.clone())
    }

    pub fn run(&mut self, job: &Job) -> anyhow::Result<()> {
        let stdout = io::stdout();
        let mut stdout = stdout.lock();

        self.success = false;
        match &job.query {
            Query::ValidateGraph => {}
            Query::CacheStatus => {}
            Query::Parse { artefacts: _ } => {}
            Query::Trace => {
                let r = self.graph.get_all_reqs();
                formatters::requirements(r, &job.format, &mut stdout)?;
                let e = self.graph.get_parsing_errors();
                formatters::errors(e, &job.format, &mut stdout)?;
            }
            Query::ShowRequirement { id: _ } => {}
            Query::ShowRequirementImpact { id: _ } => {}
        }

        Ok(())
    }

    pub fn success(&self) -> bool {
        self.success
    }
}
