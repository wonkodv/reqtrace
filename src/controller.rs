use crate::{
    common::{Artefact, ArtefactConfig, Format},
    formatters,
    graph::Graph,
    trace::Tracing,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    convert::TryFrom,
    fmt,
    fs::{self, File},
    io::{self, Write},
    path::{Path, PathBuf},
    time::Instant,
};

use log::*;

use crate::errors::{Error, Result};
use Error::*;

#[derive(Serialize, Deserialize, Debug)]
struct ArtefactConfigSerialized {
    paths: Vec<PathBuf>,
    parser: String,
    parser_options: Option<HashMap<String, String>>,
    parse_against_self: Option<bool>,
    version_provider: Option<String>,
}

impl ArtefactConfigSerialized {
    fn to_artefact_config<'c>(&'c self) -> Result<ArtefactConfig<'c>> {
        match self.parser.as_str() {
            "markdown" => {
                if self.paths.len() != 1 {
                    return Err(ArtefactTypeOnlyAllowsOnePath(
                        self.parser.clone(),
                        self.paths.clone(),
                    ));
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
    default_jobs: Option<Vec<String>>,
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
    graph: Graph<'config>,
}

impl<'c> Controller<'c> {
    pub fn new(config: &'c Config) -> Result<Self> {
        let mut graph = Graph::new();

        for (id, ac) in &config.artefact {
            let ac = ac.to_artefact_config()?;
            let a = Artefact::new(id.as_str(), ac);
            graph.add_artefact(a)?;
        }

        for tc in &config.trace {
            graph.add_fork(&tc.upper, tc.lower.iter())?;
        }

        Ok(Self { config, graph })
    }

    pub fn find_job(&self, job: &str) -> Option<&Job> {
        Some(self.config.job.as_ref()?.get(job)?)
    }

    pub fn run_default_jobs(&self) -> Result<bool> {
        trace!("Running default jobs");
        if let Some(ref default_jobs) = self.config.default_jobs {
            self.run_jobs_by_name(default_jobs)
        } else {
            Err(ConfigError("no default_jobs configured".into()))
        }
    }

    pub fn run_jobs_by_name(&self, job_names: &[String]) -> Result<bool> {
        let mut jobs = Vec::new();
        for j in job_names {
            if let Some(job) = self.find_job(&j) {
                jobs.push(job)
            } else {
                return Err(Error::UnknownJob(j.clone()));
            }
        }
        self.run_jobs(&jobs, job_names)
    }

    pub fn run_jobs(&self, jobs: &[&Job], job_names: &[String]) -> Result<bool> {
        let start = Instant::now();
        let mut success = true;
        for (job, job_name) in jobs.iter().zip(job_names.iter()) {
            if !self.run(job, job_name)? {
                success = false;
            }
        }

        info!(
            "ran {} jobs in {}ms, result: {}",
            job_names.len(),
            start.elapsed().as_millis(),
            (if success { "Success" } else { "Fail" })
        );

        Ok(success)
    }

    pub fn run(&self, job: &Job, job_name: &str) -> Result<bool> {
        trace!("Job {} {:?}", job_name, job);
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
                let t = Tracing::from_graph(&self.graph);
                if !t.errors().is_empty() {
                    success = false;
                }
                formatters::tracing(&t, &self.graph, &job.format, &mut out)
            }
            Query::Parse => {
                let reqs = self.graph.get_all_reqs();
                if self.graph.get_parsing_errors().next().is_some() {
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

        write_res.map_err(|e| IoError(job.file.clone(), e))?;

        if success {
            info!("Job {} successful", job_name);
        } else {
            warn!("Job {} failed", job_name);
        }

        Ok(success)
    }
}
