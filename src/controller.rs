use crate::{
    common::{Artefact, Format},
    formatters,
    graph::Graph,
    parsers::{self, ArtefactConfig},
    trace::Tracing,
};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, fs, io, path::PathBuf, time::Instant};

use crate::errors::{Error, Result};
use Error::*;

#[derive(Serialize, Deserialize, Debug)]
struct TraceConfig {
    upper: String,
    lower: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    artefact: BTreeMap<String, ArtefactConfig>,
    trace: Vec<TraceConfig>,
    job: Option<BTreeMap<String, Job>>,
    version_provider: Option<String>,
    default_jobs: Option<Vec<String>>,
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
    pub set_return_code: Option<bool>,
}

#[derive(Debug)]
pub struct Controller {
    jobs: BTreeMap<String, Job>,
    default_jobs: Vec<String>,
    graph: Graph,
}

impl Controller {
    pub fn new(config: Config) -> Result<Self> {
        let mut graph = Graph::new();

        for (id, ac) in config.artefact {
            let ignore_derived = ac.ignore_derived_requirements;
            let parser = parsers::ArtefactParser::from_config(ac);
            let a = Artefact::new(id, parser, ignore_derived.unwrap_or(false));
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
        self.jobs.get(job)
    }

    pub fn run_default_jobs(&self) -> Result<bool> {
        log::trace!("Running default jobs");
        if !self.default_jobs.is_empty() {
            self.run_jobs_by_name(&self.default_jobs)
        } else {
            Err(Config("no default_jobs configured".into()))
        }
    }

    pub fn run_jobs_by_name(&self, job_names: &[String]) -> Result<bool> {
        let mut jobs = Vec::new();
        for j in job_names {
            if let Some(job) = self.find_job(j) {
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
            if !self.run(job, job_name)? && job.set_return_code.unwrap_or(true) {
                requirement_covered!(DSG_JOB_RETURN_CODE);
                success = false;
            }
        }

        log::info!(
            "ran {} jobs in {}ms, result: {}",
            job_names.len(),
            start.elapsed().as_millis(),
            (if success { "Success" } else { "Fail" })
        );

        Ok(success)
    }

    pub fn run(&self, job: &Job, job_name: &str) -> Result<bool> {
        log::trace!("Job {} {:?}", job_name, job);
        let stdout = io::stdout();
        let mut out: Box<dyn io::Write>;

        if job.file.as_os_str() == "-" {
            out = Box::new(stdout.lock());
            log::info!("writing {job_name} to stdout");
        } else {
            if let Some(p) = &job.file.parent() {
                std::fs::create_dir_all(p).map_err(|e| Error::Io(p.to_path_buf(), e))?;
            }

            let file = fs::File::create(&job.file).map_err(|e| Error::Io(job.file.clone(), e))?;
            out = Box::new(file);
            log::info!("writing {} to {}", &job_name, job.file.display());
        }

        let mut success = true;

        let write_res = match &job.query {
            Query::Trace => {
                requirement_covered!(DSG_JOB_TRACE);
                // TODO: 3 jobs will compute tracing 3 times !
                let t = Tracing::from_graph(&self.graph);
                if !t.errors().is_empty() {
                    success = false;
                }
                formatters::tracing(&t, &self.graph, &job.format, &mut out)
            }
            Query::Parse => {
                requirement_covered!(DSG_JOB_PARSE);
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
            Query::ParseArtefacts { artefacts: _ } => {
                requirement_covered!(DSG_JOB_PARSE_SOME, "Parse a set of Artefacts");
                todo!()
            }
        };

        write_res.map_err(|e| Io(job.file.clone(), e))?;

        if success {
            log::info!("Job {} successful", job_name);
        } else {
            log::warn!("Job {} detected Errors", job_name);
        }

        Ok(success)
    }
}
