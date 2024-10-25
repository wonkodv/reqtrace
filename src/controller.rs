use crate::{formatters, models::*, parsers};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, fs, io, path::PathBuf, rc::Rc, time::Instant};

#[derive(
    thiserror::Error, Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize,
)]
pub enum ControllerError {
    #[error("Config Error: {0}")]
    Config(String),

    #[error("Unknown Job {0}")]
    UnknownJob(String),

    #[error("IO Error: {1} in {0}")]
    Io(PathBuf, String),
}

fn glob_paths(paths: &Vec<String>) -> Result<Vec<PathBuf>, Error> {
    let mut result = Vec::new();

    for path in paths {
        let glob = glob::glob(path);
        let glob = glob.map_err(|e| Error::Io(PathBuf::from(path), e.to_string()))?;
        for entry in glob {
            let entry = entry.map_err(|e| Error::Io(e.path().into(), e.to_string()))?;
            result.push(entry);
        }
    }

    Ok(result)
}

fn parse_single_file(config: &ArtefactConfig) -> (Vec<PathBuf>, Vec<Rc<Requirement>>, Vec<Error>) {
    if config.paths.len() != 1 {
        return (
            vec![],
            vec![],
            vec![Error::ArtefactConfig(format!(
                "Expected only 1 file for artefact {} with parser {:?}, got {:?}",
                config.id, config.parser, config.paths,
            ))],
        );
    }
    let path = PathBuf::from(config.paths[0].to_owned());

    match fs::File::open(&path) {
        Err(err) => (
            vec![path.clone()],
            vec![],
            vec![Error::Io(path.into(), err.to_string())],
        ),
        Ok(file) => {
            let mut r = io::BufReader::new(file);

            let (reqs, errs) = match &config.parser {
                ArtefactParser::Markdown => parsers::markdown::parse(&mut r, &path),
                ArtefactParser::Readme => parsers::readme::parse(&mut r, &path),
                _ => panic!("unexpected {:?}", config.parser),
            };
            (vec![path], reqs, errs)
        }
    }
}

fn parse_multiple_files(
    config: &ArtefactConfig,
) -> (Vec<PathBuf>, Vec<Rc<Requirement>>, Vec<Error>) {
    if config.paths.is_empty() {
        return (
            vec![],
            vec![],
            vec![Error::ArtefactConfig(format!(
                "No Paths for artefact {}",
                config.id
            ))],
        );
    }

    match glob_paths(&config.paths) {
        Err(err) => (
            config.paths.iter().map(PathBuf::from).collect(),
            vec![],
            vec![err],
        ),
        Ok(paths) => {
            let mut requirements = Vec::new();
            let mut errors = Vec::new();
            for path in &paths {
                match fs::File::open(&path) {
                    Err(err) => errors.push(Error::Io(path.into(), err.to_string())),
                    Ok(file) => {
                        let mut r = io::BufReader::new(file);

                        let (r, e) = match config.parser {
                            ArtefactParser::Rust => parsers::rust::parse(&mut r, &path),
                            _ => panic!("unexpected {:?}", config.parser),
                        };
                        requirements.extend(r);
                        errors.extend(e);
                    }
                }
            }
            (paths, requirements, errors)
        }
    }
}

pub fn parse_from_config(
    config: &ArtefactConfig,
) -> (Vec<PathBuf>, Vec<Rc<Requirement>>, Vec<Error>) {
    requirement_covered!(DSG_ART_PARSE_COLLECT_ERRORS);

    match config.parser {
        ArtefactParser::Rust => parse_multiple_files(config),
        ArtefactParser::Markdown | ArtefactParser::Readme => parse_single_file(config),
    }
}

#[derive(Debug)]
pub struct Controller {
    graph: Graph,
    traced_graph: TracedGraph,
    jobs: BTreeMap<String, Job>,
    default_jobs: Vec<String>,
}

impl Controller {
    pub fn new(config: Config) -> Self {
        let mut artefacts: BTreeMap<ArtefactId, Rc<Artefact>> = BTreeMap::new();

        for ac in config.artefacts {
            let ignore_derived_requirements = ac.ignore_derived_requirements.unwrap_or(false);
            let (files, requirements, errors) = parse_from_config(&ac);

            let a = Artefact {
                id: ac.id.clone(),
                files,
                requirements,
                errors,
                ignore_derived_requirements,
            };
            let a = Rc::new(a);
            artefacts.insert(ac.id, a);
        }

        let relations: Vec<Relation> = config.relations;

        let graph = Graph {
            artefacts,
            relations,
        };

        // TODO: do only when tracing is requested?
        let traced_graph = crate::trace::trace(&graph);

        Self {
            graph,
            traced_graph,
            jobs: config.jobs.unwrap_or_default(),
            default_jobs: config.default_jobs.unwrap_or_default(),
        }
    }

    pub fn find_job(&self, job: &str) -> Option<&Job> {
        self.jobs.get(job)
    }

    pub fn run_default_jobs(&self) -> Result<bool, ControllerError> {
        log::trace!("Running default jobs");
        if !self.default_jobs.is_empty() {
            self.run_jobs_by_name(&self.default_jobs)
        } else {
            Err(ControllerError::Config("no default_jobs configured".into()))
        }
    }

    pub fn run_jobs_by_name(&self, job_names: &[String]) -> Result<bool, ControllerError> {
        let mut jobs = Vec::new();
        for j in job_names {
            if let Some(job) = self.find_job(j) {
                jobs.push(job);
            } else {
                return Err(ControllerError::UnknownJob(j.clone()));
            }
        }
        self.run_jobs(&jobs, job_names)
    }

    pub fn run_jobs(&self, jobs: &[&Job], job_names: &[String]) -> Result<bool, ControllerError> {
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

    pub fn run(&self, job: &Job, job_name: &str) -> Result<bool, ControllerError> {
        log::trace!("Job {} {:?}", job_name, job);
        let stdout = io::stdout();
        let mut out: Box<dyn io::Write>;

        if job.file.as_os_str() == "-" {
            out = Box::new(stdout.lock());
            log::info!("writing {job_name} to stdout");
        } else {
            if let Some(p) = &job.file.parent() {
                std::fs::create_dir_all(p)
                    .map_err(|e| ControllerError::Io(p.to_path_buf(), e.to_string()))?;
            }

            let file = fs::File::create(&job.file)
                .map_err(|e| ControllerError::Io(job.file.clone(), e.to_string()))?;
            out = Box::new(file);
            log::info!("writing {} to {}", &job_name, job.file.display());
        }

        let mut success = true;

        let write_res = match &job.query {
            Query::Trace => {
                requirement_covered!(DSG_JOB_TRACE);

                let tg = &self.traced_graph;
                if !tg.errors.is_empty() {
                    success = false;
                }
                if tg.artefacts.values().any(|art| !art.errors.is_empty()) {
                    success = false;
                }
                formatters::tracing(tg, &job.format, &mut out)
            }
            Query::Parse => {
                requirement_covered!(DSG_JOB_PARSE);
                if self
                    .graph
                    .artefacts
                    .values()
                    .any(|art| !art.errors.is_empty())
                {
                    success = false;
                }
                formatters::requirements(&self.graph, &job.format, &mut out)
            }
            Query::ValidateGraph => todo!(),
        };

        write_res.map_err(|e| ControllerError::Io(job.file.clone(), e.to_string()))?;

        if success {
            log::info!("Job {} successful", job_name);
        } else {
            log::warn!("Job {} detected Errors", job_name);
        }

        Ok(success)
    }
}
