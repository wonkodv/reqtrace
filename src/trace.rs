use std::{
    collections::{BTreeSet, HashMap, HashSet},
    error, fmt,
    rc::Rc,
};

use super::common::*;
use crate::errors::{Error, Result};
use crate::graph::*;
use Error::*;

use std::collections::hash_map::Entry::{Occupied, Vacant};

#[derive(Debug, Copy, Clone)]
pub struct Coverage<'a> {
    pub upper_requirement: &'a Rc<Requirement>,
    pub lower_requirement: &'a Rc<Requirement>,
    pub kind: CoverageKind<'a>,
    tine: Tine,
}

#[derive(Debug)]
pub struct TracedRequirement<'a> {
    pub requirement: &'a Rc<Requirement>,
    pub upper: HashMap<Fork, Vec<Coverage<'a>>>,
    pub lower: HashMap<Fork, Vec<Coverage<'a>>>,
    node: NodeIdx,
}

type TracedReqIdx = usize;

/// Tracing info beeing build.
///
/// When adding a Fork (all tines at once) add all lower Reqs to
/// `derived`, add all  reqs to `requirements`. for all upper, find a lower that covers or is
/// depent on. if not found, add to uncovered
#[derive(Debug, Default)]
pub struct Tracing<'a> {
    requirements: Vec<TracedRequirement<'a>>,
    requirements_by_id: HashMap<&'a str, TracedReqIdx>,
    uncovered: HashSet<TracedReqIdx>,
    derived: HashSet<TracedReqIdx>,
    errors: Vec<Error>,
}

impl<'a> Tracing<'a> {
    pub fn from_graph(graph: &'a Graph<'a>) -> Self {
        let mut trace = Tracing::default();

        for fork in graph.iter_forks() {
            trace.add_fork(fork, graph)
        }

        trace
    }

    pub fn errors(&self) -> &[Error] {
        self.errors.as_slice()
    }

    pub fn uncovered<'s>(&'s self) -> impl Iterator<Item = &'s TracedRequirement<'a>> {
        self.uncovered
            .iter()
            .map(move |idx| &self.requirements[*idx])
    }

    pub fn derived<'s>(&'s self) -> impl Iterator<Item = &'s TracedRequirement<'a>> {
        self.derived.iter().map(move |idx| &self.requirements[*idx])
    }

    pub fn requirements(&self) -> &[TracedRequirement<'a>] {
        self.requirements.as_slice()
    }

    pub fn requirement_by_id<'s>(&'s self, id: &str) -> Option<&'s TracedRequirement> {
        self.requirements.get(*self.requirements_by_id.get(id)?)
    }
}

/// Computing Tracing Data
impl<'a> Tracing<'a> {
    /// Compute Tracing for one upper and some lower artefacts
    fn add_fork(&mut self, fork: Fork, graph: &'a Graph<'a>) {
        let upper_artefact = &fork.from(graph).artefact(graph);

        for tine in fork.tines(graph) {
            let lower_artefact = &tine.to(graph).artefact(graph);

            for r in lower_artefact.get_requirements() {
                self.add_lower_req(r, tine, graph)
            }
        }

        for upper_requirement in upper_artefact.get_requirements() {
            let upper_requirement_idx = self.add_upper_req(upper_requirement, fork, graph);
            let mut is_covered = false;
            for depends in &upper_requirement.depends {
                for tine in fork.tines(graph) {
                    let lower_artefact = &tine.to(graph).artefact(graph);

                    if let Some(lower_requirement) =
                        lower_artefact.get_requirement_with_id(&depends.id)
                    {
                        is_covered = true;
                        let kind = if let Some(title) = depends.title.as_ref() {
                            CoverageKind::DependsWithTitle(&title)
                        } else {
                            CoverageKind::Depends
                        };
                        let cov = Coverage {
                            upper_requirement,
                            lower_requirement,
                            tine,
                            kind,
                        };

                        self.add_coverage(cov);
                    }
                }
            }
            for tine in fork.tines(graph) {
                let lower_artefact = &tine.to(graph).artefact(graph);

                for (lower_requirement, referenced_title) in
                    lower_artefact.get_requirements_that_cover(&upper_requirement.id)
                {
                    is_covered = true;
                    let kind = if let Some(title) = referenced_title.as_ref() {
                        CoverageKind::CoveredWithTitle(&title)
                    } else {
                        CoverageKind::Covered
                    };
                    let cov = Coverage {
                        upper_requirement,
                        lower_requirement,
                        tine,
                        kind,
                    };

                    self.add_coverage(cov);
                }
            }
            if !is_covered {
                self.uncovered.insert(upper_requirement_idx);
            }
        }
    }

    fn add_lower_req(&mut self, req: &'a Rc<Requirement>, tine: Tine, graph: &Graph) {
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
        req: &'a Rc<Requirement>,
        fork: Fork,
        graph: &Graph,
    ) -> TracedReqIdx {
        let (_added, idx) = self.add_req(req, fork.from(graph));

        self.requirements
            .get_mut(idx)
            .unwrap()
            .lower
            .insert(fork, vec![]);

        idx
    }

    /// Ensure Requirement is in `requirements`.
    fn add_req(&mut self, req: &'a Rc<Requirement>, node: NodeIdx) -> (bool, TracedReqIdx) {
        match self.requirements_by_id.entry(&req.id) {
            Occupied(e) => {
                let already_there_idx = *e.get();
                let already_there = &self.requirements.get_mut(already_there_idx).unwrap();
                if req != already_there.requirement {
                    self.errors.push(DuplicateRequirement(
                        Rc::clone(already_there.requirement),
                        Rc::clone(req),
                    ));
                }
                assert_eq!(already_there.node, node);

                (false, already_there_idx)
            }
            Vacant(e) => {
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
    fn add_coverage(&mut self, cov: Coverage<'a>) {
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
            CoverageKind::CoveredWithTitle(title) => {
                if Some(title) != cov.upper_requirement.title.as_deref() {
                    self.errors.push(DependWithWrongTitle(
                        Rc::clone(cov.upper_requirement),
                        Rc::clone(cov.lower_requirement),
                        title.into(),
                    ));
                }
            }
            CoverageKind::DependsWithTitle(_) => todo!(),
            _ => {}
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
    use super::*;
}
