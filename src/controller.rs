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

fn glob_paths(paths: &Vec<String>) -> Result<Vec<PathBuf>> {
    let mut result = Vec::new();

    for path in paths {
        let glob = glob::glob(path);
        let glob = glob.map_err(|e| Error::ConfigError(format!("can not glob {path:?}: {e:?}")))?;
        for path in glob {
            let path = path.map_err(|e| Error::IoError(e.path().into(), e.into_error()))?;
            result.push(path);
        }
    }

    Ok(result)
}

#[derive(Serialize, Deserialize, Debug)]
struct ArtefactConfigSerialized {
    paths: Vec<String>,
    parser: String,
    parser_options: Option<HashMap<String, String>>,
    parse_against_self: Option<bool>,
    version_provider: Option<String>,
}

impl ArtefactConfigSerialized {
    fn into_artefact_config(mut self) -> Result<ArtefactConfig> {
        match self.parser.as_str() {
            "markdown" => {
                if self.paths.len() != 1 {
                    return Err(ArtefactTypeOnlyAllowsOnePath(self.parser, self.paths));
                }
                if self.parser_options.is_some() {
                    todo!();
                }
                Ok(ArtefactConfig::Markdown(PathBuf::from(
                    self.paths.remove(0),
                )))
            }
            "rust" => {
                let paths = glob_paths(&self.paths)?;
                Ok(ArtefactConfig::Rust(paths))
            }
            _ => {
                error!("Unknown Artefact Type {}", &self.parser);
                Err(UnknownArtefactType(self.parser))
            }
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
pub struct Controller {
    jobs: HashMap<String, Job>,
    default_jobs: Vec<String>,
    graph: Graph,
}

impl Controller {
    pub fn new(config: Config) -> Result<Self> {
        let mut graph = Graph::new();

        for (id, ac) in config.artefact {
            let ac = ac.into_artefact_config()?;
            let a = Artefact::new(id, ac);
            graph.add_artefact(a)?;
        }

        for tc in &config.trace {
            graph.add_fork(&tc.upper, tc.lower.iter())?;
        }
        let jobs = config.job.unwrap_or_default();
        let default_jobs = config.default_jobs.unwrap_or_default();

        Ok(Self {
            jobs,
            graph,
            default_jobs,
        })
    }

    pub fn find_job(&self, job: &str) -> Option<&Job> {
        Some(self.jobs.get(job)?)
    }

    pub fn run_default_jobs(&self) -> Result<bool> {
        trace!("Running default jobs");
        if !self.default_jobs.is_empty() {
            self.run_jobs_by_name(&self.default_jobs)
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
                error!("Unknown Job {j}");
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
