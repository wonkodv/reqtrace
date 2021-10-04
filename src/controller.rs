use crate::{
    common::{Artefact, ArtefactConfig, Format},
    formatters,
    trace::Graph,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    convert::TryFrom,
    fs::{self, File},
    io::{self, Write},
    path::{Path, PathBuf},
};

use toml;

use crate::errors::{Error, Result};
use Error::*;

#[derive(Serialize, Deserialize, Debug)]
struct ArtefactConfigSerialized {
    paths: Vec<PathBuf>,
    parser: String,
    parser_options: Option<String>,
    version_provider: Option<String>,
}

impl ArtefactConfigSerialized {
    fn to_artefact_config<'c>(&'c self) -> Result<ArtefactConfig<'c>> {
        match self.parser.as_str() {
            "markdown" => {
                if self.paths.len() != 1 {
                    return Err(OnlyOnePathExpected);
                }
                if self.parser_options.is_some() {
                    todo!();
                }
                Ok(ArtefactConfig::Markdown(&self.paths[0]))
            }
            "rust_cov_marks" | "rust_unsafe" => {
                Ok(ArtefactConfig::PrePopulated(vec![])) // TODO
            }
            x => Err(UnknownArtefactType(x.into())),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct TraceConfig {
    upper: String,
    lower: Vec<String>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    artefact: HashMap<String, ArtefactConfigSerialized>,
    trace: Vec<TraceConfig>,
    job: Option<HashMap<String, Job>>,
    version_provider: Option<String>,
}

impl Config {
    pub fn from_toml_file<P: AsRef<Path>>(p: P) -> io::Result<Self> {
        let config: Config = toml::from_slice(fs::read(p)?.as_slice())?;
        Ok(config)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Query {
    /// Validate the Graph without parsing Artefacts
    ValidateGraph,

    /// Query Artefacts and Tracing Edges to see which are outdated
    CacheStatus,

    /// Parse a list of artefacts
    ParseArtefacts { artefacts: Vec<String> },

    /// Parse all Artefacts
    Parse,

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
    pub file: PathBuf,
}

#[derive(Debug)]
pub struct Controller<'config> {
    config: &'config Config,
    graph: Option<Graph<'config>>,
    success: bool,
}

impl<'c> Controller<'c> {
    pub fn new(config: &'c Config) -> Self {
        Self {
            config,
            graph: None,
            success: true,
        }
    }

    pub fn load(&mut self) -> Result<&Graph> {
        if self.graph.is_none() {
            let mut graph = Graph::new();

            for (id, ac) in &self.config.artefact {
                let ac = ac.to_artefact_config()?;
                let a = Artefact::new(id.as_str(), ac);
                graph.add_artefact(a)?;
            }

            for tc in &self.config.trace {
                graph.add_edge_group(&tc.upper, tc.lower.iter())?;
            }

            self.graph = Some(graph);
        }

        Ok(&self.graph.as_ref().unwrap())
    }

    pub fn find_job(&self, job: &str) -> Option<Job> {
        Some(self.config.job.as_ref()?.get(job)?.clone())
    }

    pub fn run(&mut self, job: &Job) -> Result<()> {
        let graph = self.load()?;
        let stdout = io::stdout();
        let mut out: Box<dyn io::Write>;

        if job.file.as_os_str() == "-" {
            out = Box::new(stdout.lock());
        } else {
            let file = File::create(&job.file).map_err(|e| Error::IoError(job.file.clone(), e))?;
            out = Box::new(file);
        }

        let mut success = true;

        let write_res = match &job.query {
            Query::Trace => {
                let t = graph.trace()?;
                if !t.errors().is_empty() {
                    success = false;
                }
                formatters::tracing(&t, &graph, &job.format, &mut out)
            }
            Query::Parse => {
                let reqs = graph.get_all_reqs();
                if let Some(_) = graph.get_parsing_errors().next() {
                    success = false;
                }
                formatters::requirements(reqs, &job.format, &mut out)
            }
            Query::ShowRequirement { id: _ } => todo!(),
            Query::ShowRequirementImpact { id: _ } => todo!(),
            Query::ValidateGraph => todo!(),
            Query::CacheStatus => todo!(),
            Query::ParseArtefacts { artefacts: _ } => todo!(),
        };

        write_res.map_err(|e| IoError(PathBuf::from("/dev/stdout"), e))?;

        self.success = success;

        Ok(())
    }

    pub fn success(&self) -> bool {
        self.success
    }
}
