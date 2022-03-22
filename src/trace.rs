use log::*;

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::rc::Rc;

use super::common::*;
use crate::errors::Error;
use crate::graph::*;
use Error::*;

use std::collections::hash_map::Entry::Occupied;
use std::collections::hash_map::Entry::Vacant;

#[derive(Debug, Copy, Clone)]
pub struct Coverage<'reqs> {
    pub upper_requirement: &'reqs Rc<Requirement>,
    pub lower_requirement: &'reqs Rc<Requirement>,
    pub kind: CoverageKind<'reqs>,
    pub location: Option<&'reqs Location>,
    tine: Tine,
}

#[derive(Debug)]
pub struct TracedRequirement<'reqs> {
    pub requirement: &'reqs Rc<Requirement>,
    pub upper: BTreeMap<Fork, Vec<Coverage<'reqs>>>,
    pub lower: BTreeMap<Fork, Vec<Coverage<'reqs>>>,
    node: NodeIdx,
}

impl<'graph> TracedRequirement<'graph> {
    pub fn artefact(&self, graph: &'graph Graph) -> &'graph Artefact {
        self.node.artefact(graph)
    }
}

type TracedRequirementIdx = usize;

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Link {
    lower: String,
    upper: String,
}

/// Tracing info beeing build.
///
/// When adding a Fork (all tines at once) add all lower Reqs to
/// `derived`, add all  reqs to `requirements`. for all upper, find a lower that covers or is
/// depent on. if not found, add to uncovered
#[derive(Debug)]
pub struct Tracing<'graph> {
    requirements: Vec<TracedRequirement<'graph>>,
    requirements_by_id: HashMap<&'graph str, TracedRequirementIdx>,
    uncovered: BTreeSet<TracedRequirementIdx>,
    derived: BTreeSet<TracedRequirementIdx>,
    invalid_covers_links: Option<BTreeSet<Link>>,
    invalid_depends_links: Option<BTreeSet<Link>>,
    errors: Vec<Error>,
}

impl<'graph> Tracing<'graph> {
    pub fn from_graph(graph: &'graph Graph) -> Self {
        let mut trace = Tracing {
            requirements: Vec::new(),
            requirements_by_id: HashMap::new(),
            uncovered: BTreeSet::new(),
            derived: BTreeSet::new(),
            invalid_covers_links: Some(BTreeSet::new()),
            invalid_depends_links: Some(BTreeSet::new()),
            errors: Vec::new(),
        };

        for fork in graph.iter_forks() {
            trace.add_fork(fork, graph)
        }

        trace.validate();

        trace
    }

    pub fn errors(&self) -> &[Error] {
        self.errors.as_slice()
    }

    pub fn uncovered<'s>(&'s self) -> impl Iterator<Item = &'s TracedRequirement<'graph>> {
        self.uncovered
            .iter()
            .map(move |idx| &self.requirements[*idx])
    }

    pub fn derived<'s>(&'s self) -> impl Iterator<Item = &'s TracedRequirement<'graph>> {
        self.derived.iter().map(move |idx| &self.requirements[*idx])
    }

    pub fn requirements(&self) -> &[TracedRequirement<'graph>] {
        self.requirements.as_slice()
    }

    pub fn requirement_by_id<'s>(&'s self, id: &str) -> Option<&'s TracedRequirement<'graph>> {
        self.requirements.get(*self.requirements_by_id.get(id)?)
    }
}

/// Computing Tracing Data
impl<'graph> Tracing<'graph> {
    /// Compute Tracing for one upper and some lower artefacts
    fn add_fork(&mut self, fork: Fork, graph: &'graph Graph) {
        let upper_artefact = &fork.from(graph).artefact(graph);

        trace!(
            "Trace {} against {}",
            upper_artefact.id,
            fork.tines(graph)
                .map(|t| t.to(graph).artefact(graph).id.clone())
                .collect::<Vec<_>>()
                .join(", ")
        );

        for tine in fork.tines(graph) {
            let lower_artefact = &tine.to(graph).artefact(graph);

            for r in lower_artefact.get_requirements() {
                self.add_lower_req(r, tine, graph);
            }
        }

        for upper_requirement in upper_artefact.get_requirements() {
            let upper_requirement_idx = self.add_upper_req(upper_requirement, fork, graph);
            let mut is_covered = false;
            for depends in &upper_requirement.depends {
                // Covers: DSG_TRACE_DOWNWARDS: Trace downwards using Depends attribute
                for tine in fork.tines(graph) {
                    let lower_artefact = &tine.to(graph).artefact(graph);

                    if let Some(lower_requirement) =
                        lower_artefact.get_requirement_with_id(&depends.id)
                    {
                        is_covered = true;
                        let kind = if let Some(title) = depends.title.as_ref() {
                            CoverageKind::DependsWithTitle(title)
                        } else {
                            CoverageKind::Depends
                        };
                        let cov = Coverage {
                            upper_requirement,
                            lower_requirement,
                            tine,
                            kind,
                            location: depends.location.as_ref(),
                        };

                        self.add_coverage(cov);
                    }
                }
            }

            for tine in fork.tines(graph) {
                let lower_artefact = &tine.to(graph).artefact(graph);

                for (lower_requirement, reference) in
                    lower_artefact.get_requirements_that_cover(&upper_requirement.id)
                {
                    is_covered = true;
                    let kind = if let Some(title) = reference.title.as_ref() {
                        CoverageKind::CoveredWithTitle(title)
                    } else {
                        CoverageKind::Covered
                    };
                    let cov = Coverage {
                        upper_requirement,
                        lower_requirement,
                        tine,
                        kind,
                        location: reference.location.as_ref(),
                    };

                    self.add_coverage(cov);
                }
            }
            if !is_covered {
                self.uncovered.insert(upper_requirement_idx);
            }
        }
    }

    fn add_lower_req(&mut self, req: &'graph Rc<Requirement>, tine: Tine, graph: &Graph) {
        let (added, idx) = self.add_req(req, tine.to(graph));
        if added {
            self.derived.insert(idx);
        }
        self.requirements
            .get_mut(idx)
            .unwrap()
            .upper
            .insert(tine.fork, vec![]);
    }

    fn add_upper_req(
        &mut self,
        req: &'graph Rc<Requirement>,
        fork: Fork,
        graph: &Graph,
    ) -> TracedRequirementIdx {
        let (_added, idx) = self.add_req(req, fork.from(graph));

        self.requirements
            .get_mut(idx)
            .unwrap()
            .lower
            .insert(fork, vec![]);

        idx
    }

    /// Ensure Requirement is in `requirements`.
    ///
    /// Returns
    ///     -   added:  true if `req` was added, false if it was already there
    ///     -   idx:    the index of req
    fn add_req(
        &mut self,
        req: &'graph Rc<Requirement>,
        node: NodeIdx,
    ) -> (bool, TracedRequirementIdx) {
        match self.requirements_by_id.entry(&req.id) {
            Occupied(e) => {
                let already_there_idx = *e.get();
                let already_there = &self.requirements.get_mut(already_there_idx).unwrap();
                if req != already_there.requirement {
                    let err =
                        DuplicateRequirement(Rc::clone(already_there.requirement), Rc::clone(req));
                    warn!("{}", err);
                    self.errors.push(err);
                }
                assert_eq!(already_there.node, node);

                (false, already_there_idx)
            }
            Vacant(e) => {
                for cov in &req.covers {
                    // Covers: DSG_TRACE_COVERS_EXIST
                    self.invalid_covers_links.as_mut().unwrap().insert(Link {
                        lower: req.id.clone(),
                        upper: cov.id.clone(),
                    });
                }
                for dep in &req.depends {
                    // Covers: DSG_TRACE_COVERS_EXIST
                    self.invalid_depends_links.as_mut().unwrap().insert(Link {
                        lower: dep.id.clone(),
                        upper: req.id.clone(),
                    });
                }

                let t = TracedRequirement {
                    requirement: req,
                    upper: Default::default(),
                    lower: Default::default(),
                    node,
                };
                let idx = self.requirements.len();
                self.requirements.push(t);
                e.insert(idx);

                (true, idx)
            }
        }
    }

    /// Record coverage info for upper and lower requirement.
    /// remove lower from `derived`
    /// check that coverage with title used correct title
    fn add_coverage(&mut self, cov: Coverage<'graph>) {
        let lower_idx = self.requirements_by_id[cov.lower_requirement.id.as_str()];
        self.derived.remove(&lower_idx);

        self.requirements
            .get_mut(lower_idx)
            .unwrap()
            .upper
            .get_mut(&cov.tine.fork)
            .unwrap()
            .push(cov);

        let upper_idx = self.requirements_by_id[cov.upper_requirement.id.as_str()];
        self.requirements
            .get_mut(upper_idx)
            .unwrap()
            .lower
            .get_mut(&cov.tine.fork)
            .unwrap()
            .push(cov);

        match cov.kind {
            CoverageKind::Covered | CoverageKind::CoveredWithTitle(_) => {
                self.invalid_covers_links.as_mut().unwrap().remove(&Link {
                    lower: cov.lower_requirement.id.clone(),
                    upper: cov.upper_requirement.id.clone(),
                });
            }
            CoverageKind::Depends | CoverageKind::DependsWithTitle(_) => {
                self.invalid_depends_links.as_mut().unwrap().remove(&Link {
                    lower: cov.lower_requirement.id.clone(),
                    upper: cov.upper_requirement.id.clone(),
                });
            }
        }

        match cov.kind {
            CoverageKind::CoveredWithTitle(title) => {
                if Some(title) != cov.upper_requirement.title.as_deref() {
                    let err = CoveredWithWrongTitle {
                        upper: Rc::clone(cov.upper_requirement),
                        lower: Rc::clone(cov.lower_requirement),
                        wrong_title: title.into(),
                        location: cov.location.cloned(),
                    };
                    warn!("{}", err);
                    self.errors.push(err);
                }
            }
            CoverageKind::DependsWithTitle(title) => {
                if Some(title) != cov.lower_requirement.title.as_deref() {
                    let err = DependWithWrongTitle {
                        upper: Rc::clone(cov.upper_requirement),
                        lower: Rc::clone(cov.lower_requirement),
                        wrong_title: title.into(),
                        location: cov.location.cloned(),
                    };
                    warn!("{}", err);
                    self.errors.push(err);
                }
            }
            _ => {}
        }
    }

    fn validate(&mut self) {
        self.validate_upwards();
        self.validate_downwards();
    }

    fn validate_downwards(&mut self) {
        for cov in self.invalid_covers_links.take().unwrap() {
            let req = self
                .requirement_by_id(&cov.lower)
                .expect("requirement with wrong link exists")
                .requirement;
            let req = Rc::clone(req);
            let reference: &Reference = req
                .covers
                .iter()
                .filter(|r: &&Reference| r.id == cov.upper)
                .next()
                .expect("invalid link exists");
            let location = reference.location.clone();

            // Covers: DSG_TRACE_COVERS_EXIST
            let err = Error::CoversUnknownRequirement(req, cov.upper, location);
            warn!("{}", err);
            self.errors.push(err);
        }
    }

    fn validate_upwards(&mut self) {
        for dep in self.invalid_depends_links.take().unwrap() {
            // Covers: DSG_TRACE_DEPENDS_EXIST
            let req = self
                .requirement_by_id(&dep.upper)
                .expect("requirement with wrong link exists")
                .requirement;
            let req = Rc::clone(req);
            let reference = req
                .depends
                .iter()
                .filter(|r: &&Reference| r.id == dep.lower)
                .next()
                .expect("invalid link exists");
            let location = reference.location.clone();

            let err = Error::DependOnUnknownRequirement(
                Rc::clone(self.requirement_by_id(&dep.upper).unwrap().requirement),
                dep.lower,
                location,
            );
            warn!("{}", err);
            self.errors.push(err);
        }
    }
}
#[derive(Debug, Copy, Clone)]
pub enum CoverageKind<'a> {
    Covered,
    CoveredWithTitle(&'a str),
    Depends,
    DependsWithTitle(&'a str),
}

/// Derived Requirement in Node
#[derive(Debug, PartialEq)]
pub struct DerivedRequirement<'a> {
    pub req: &'a Rc<Requirement>,
    node: NodeIdx,
}

#[cfg(test)]
mod tests {
    // TODO: tests
}
