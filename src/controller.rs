use std::collections::btree_map;
use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;
use std::rc::Rc;
use std::time::Instant;

use crate::formatters;
use crate::models::*;
use crate::parsers;

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

#[derive(Debug, Eq, PartialEq, serde::Deserialize)]
pub enum JobSuccess {
    /// All jobs were successfull
    Success,

    /// Some errors where detected
    ErrorsDetected,
}

fn glob_paths(paths: &Vec<String>, base_dir: &Path) -> Result<Vec<PathBuf>, Error> {
    let mut result = Vec::new();

    for path in paths {
        let glob = if Path::new(path).is_relative() {
            glob::glob(&format!("{}/{}", base_dir.display(), path))
        } else {
            glob::glob(path)
        };
        let glob = glob.map_err(|e| Error::Io(PathBuf::from(path), e.to_string()))?;
        for entry in glob {
            let entry = entry.map_err(|e| Error::Io(e.path().into(), e.to_string()))?;
            result.push(entry);
        }
    }

    Ok(result)
}

fn parse_single_file(
    config: &ArtefactConfig,
    base_dir: &Path,
) -> (Vec<PathBuf>, Vec<Rc<Requirement>>, Vec<Error>) {
    if config.paths.len() != 1 {
        let err = Error::Config(format!(
            "Expected only 1 file for artefact {} with parser {:?}, got {:?}",
            config.id, config.parser, config.paths,
        ));
        log::info!("found problem {err:#?}");
        return (vec![], vec![], vec![(err)]);
    }
    let path = base_dir.join(&config.paths[0]);

    requirement_covered!(DSG_ART_FILES);
    match fs::File::open(&path) {
        Err(err) => {
            let err = Error::Io(path.clone(), err.to_string());
            log::info!("found problem {:#?}", &err);
            (vec![path], vec![], vec![err])
        }
        Ok(file) => {
            let mut r = io::BufReader::new(file);

            let (reqs, errs) = match &config.parser {
                ArtefactParser::Markdown => parsers::markdown::parse(&mut r, &path),
                ArtefactParser::MonoRequirement => parsers::monoreq::parse(&mut r, &path),
                ArtefactParser::Json => match serde_json::from_reader(r) {
                    Ok(reqs) => (reqs, vec![]),
                    Err(err) => {
                        let err =
                            Error::Format(Location::new_with_no_pos(path.clone()), err.to_string());
                        log::info!("found problem {:#?}", &err);

                        (vec![], vec![err])
                    }
                },
                _ => panic!("unexpected {:?}", config.parser),
            };
            (vec![path], reqs, errs)
        }
    }
}

fn parse_multiple_files(
    config: &ArtefactConfig,
    base_dir: &Path,
) -> (Vec<PathBuf>, Vec<Rc<Requirement>>, Vec<Error>) {
    if config.paths.is_empty() {
        let err = Error::Config(format!("No Paths for artefact {}", config.id));
        log::info!("found problem {:#?}", &err);
        return (vec![], vec![], vec![err]);
    }

    match glob_paths(&config.paths, base_dir) {
        Err(err) => (
            config.paths.iter().map(PathBuf::from).collect(),
            vec![],
            vec![err],
        ),
        Ok(paths) => {
            let mut requirements = Vec::new();
            let mut errors = Vec::new();
            for path in &paths {
                requirement_covered!(DSG_ART_FILES);
                match fs::File::open(path) {
                    Err(err) => {
                        let err = Error::Io(path.into(), err.to_string());
                        log::info!("found problem {:#?}", &err);
                        errors.push(err)
                    }
                    Ok(file) => {
                        let mut r = io::BufReader::new(file);

                        let (r, e) = match config.parser {
                            ArtefactParser::Rust => parsers::rust::parse(&mut r, path),
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
    base_dir: &Path,
) -> (
    Vec<PathBuf>,
    BTreeMap<RequirementId, Rc<Requirement>>,
    Vec<Error>,
) {
    requirement_covered!(DSG_ART_PARSE_COLLECT_ERRORS);

    let (paths, requirement_vec, mut errors) = match config.parser {
        ArtefactParser::Rust => parse_multiple_files(config, base_dir),
        ArtefactParser::Json | ArtefactParser::Markdown | ArtefactParser::MonoRequirement => {
            parse_single_file(config, base_dir)
        }
    };

    let mut requirements = BTreeMap::new();
    for req in requirement_vec {
        match requirements.entry(req.id.clone()) {
            btree_map::Entry::Occupied(e) => {
                requirement_covered!(DSG_CTRL_DETECT_DUPLICATE_REQS);
                let err = Error::DuplicateRequirement(Rc::clone(e.get()), req);
                log::info!("found problem {:#?}", &err);
                errors.push(err);
            }
            btree_map::Entry::Vacant(e) => {
                e.insert(req);
            }
        }
    }

    (paths, requirements, errors)
}

#[derive(Debug)]
pub struct Controller {
    base_dir: PathBuf,
    graph: Graph,
    traced_graph: TracedGraph,
    jobs: BTreeMap<String, Job>,
    default_jobs: Vec<String>,
}

impl Controller {
    pub fn new(config: Config, base_dir: &Path) -> Self {
        let mut artefacts: BTreeMap<ArtefactId, Rc<Artefact>> = BTreeMap::new();

        for ac in config.artefacts {
            let ignore_derived_requirements = ac.ignore_derived_requirements.unwrap_or(false);
            let (files, requirements, errors) = parse_from_config(&ac, &base_dir);

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

        for r in &relations {
            if artefacts.get(&r.upper).is_none() {
                let err = Error::Config(format!(
                    "Relation.upper references unknown artefact {}",
                    &r.upper
                ));
                todo!("log error {err}");
            }
            for lower in &r.lower {
                if artefacts.get(&lower).is_none() {
                    let err = Error::Config(format!(
                        "Relation.lower references unknown artefact {}",
                        &lower
                    ));
                    todo!("log error {err}");
                }
            }
        }

        let graph = Graph {
            artefacts,
            relations,
        };

        // TODO: do only when tracing is requested?
        let traced_graph = crate::trace::trace(&graph);

        Self {
            base_dir: base_dir.to_owned(),
            graph,
            traced_graph,
            jobs: config.jobs.unwrap_or_default(),
            default_jobs: config.default_jobs.unwrap_or_default(),
        }
    }

    pub fn find_job(&self, job: &str) -> Option<&Job> {
        self.jobs.get(job)
    }

    pub fn run_default_jobs(&self) -> Result<JobSuccess, ControllerError> {
        log::trace!("Running default jobs");
        if !self.default_jobs.is_empty() {
            self.run_jobs_by_name(&self.default_jobs)
        } else {
            let err = ControllerError::Config("no default_jobs configured".into());
            log::info!("found problem {:#?}", &err);
            Err(err)
        }
    }

    pub fn run_jobs_by_name(&self, job_names: &[String]) -> Result<JobSuccess, ControllerError> {
        let mut jobs = Vec::new();
        for j in job_names {
            if let Some(job) = self.find_job(j) {
                jobs.push(job);
            } else {
                let err = ControllerError::UnknownJob(j.clone());
                log::info!("found problem {:#?}", &err);
                return Err(err);
            }
        }
        self.run_jobs(&jobs, job_names)
    }

    pub fn run_jobs(
        &self,
        jobs: &[&Job],
        job_names: &[String],
    ) -> Result<JobSuccess, ControllerError> {
        let start = Instant::now();
        let mut success = JobSuccess::Success;
        for (job, job_name) in jobs.iter().zip(job_names.iter()) {
            match self.run(job, job_name)? {
                JobSuccess::Success => {}
                JobSuccess::ErrorsDetected => {
                    if job.set_return_code.unwrap_or(true) {
                        requirement_covered!(DSG_CTRL_RETURN_CODE);
                        success = JobSuccess::ErrorsDetected;
                    }
                }
            }
        }

        log::info!(
            "ran {} jobs in {}ms, result: {:?}",
            job_names.len(),
            start.elapsed().as_millis(),
            success,
        );

        Ok(success)
    }

    pub fn run(&self, job: &Job, job_name: &str) -> Result<JobSuccess, ControllerError> {
        log::trace!("Job {} {:?}", job_name, job);
        let stdout = io::stdout();
        let mut out: Box<dyn io::Write>;

        if job.file.as_os_str() == "-" {
            out = Box::new(stdout.lock());
            log::info!("writing {job_name:?} to stdout");
        } else {
            let path = if job.file.is_absolute() {
                &job.file
            } else {
                &self.base_dir.join(&job.file)
            };
            if let Some(p) = &path.parent() {
                std::fs::create_dir_all(p)
                    .map_err(|e| ControllerError::Io(p.to_path_buf(), e.to_string()))?;
            }

            let file = fs::File::create(&path)
                .map_err(|e| ControllerError::Io(path.clone(), e.to_string()))?;
            out = Box::new(file);
            log::info!("writing {} to {}", &job_name, path.display());
        }

        let mut success = JobSuccess::Success;

        let write_res = match &job.query {
            Query::Trace => {
                requirement_covered!(DSG_CTRL_TRACE);
                let tg = &self.traced_graph;
                // TODO: only create traced_graph lazily
                if !tg.errors.is_empty() {
                    success = JobSuccess::ErrorsDetected;
                    log::debug!("Job did not succeed because: Tracing Errors");
                }
                if tg.artefacts.values().any(|art| !art.errors.is_empty()) {
                    success = JobSuccess::ErrorsDetected;
                    log::debug!("Job did not succeed because: Artefact Parsing Errors");
                }
                if tg
                    .traced_relations
                    .iter()
                    .any(|rel| !rel.uncovered.is_empty())
                {
                    success = JobSuccess::ErrorsDetected;
                    log::debug!("Job did not succeed because: uncovered relations");
                }

                if tg.derived.values().any(|a| !a.is_empty()) {
                    success = JobSuccess::ErrorsDetected;
                    log::debug!(
                        "Job did not succeed because: derived Requirements: {:#?}",
                        tg.derived
                    );
                }
                requirement_covered!(DSG_CTRL_FORMAT);
                formatters::tracing(tg, &job.format, &mut out)
            }
            Query::Parse => {
                requirement_covered!(DSG_CTRL_PARSE);
                requirement_covered!(DSG_CTRL_GRAPH);
                if self
                    .graph
                    .artefacts
                    .values()
                    .any(|art| !art.errors.is_empty())
                {
                    success = JobSuccess::ErrorsDetected;
                    log::debug!("Job did not succeed because: Artefact Parsing Errors");
                }
                requirement_covered!(DSG_CTRL_FORMAT);
                formatters::requirements(&self.graph, &job.format, &mut out)
            }
        };

        write_res.map_err(|e| ControllerError::Io(job.file.clone(), e.to_string()))?;

        match success {
            JobSuccess::Success => {
                log::info!("Job {} successful", job_name);
            }
            JobSuccess::ErrorsDetected => {
                log::warn!("Job {} detected Errors", job_name);
            }
        }
        requirement_covered!(DSG_CTRL_RETURN_CODE);

        Ok(success)
    }
}
