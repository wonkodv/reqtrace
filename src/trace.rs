use std::collections::btree_map;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::rc::Rc;

use Error::CoveredWithWrongTitle;
use Error::CoversUnknownRequirement;
use Error::DependOnUnknownRequirement;
use Error::DependWithWrongTitle;
use Error::DuplicateRequirement;

use crate::models::*;

pub fn trace(graph: &Graph) -> TracedGraph {
    Tracer::new(graph).data()
}

#[derive(Debug)]
struct TracedArtefact {
    artefact: Rc<Artefact>,

    requirements_by_id: BTreeMap<RequirementId, Rc<Requirement>>,

    requirements_that_cover: BTreeMap<RequirementId, Vec<(Rc<Requirement>, Reference)>>,

    derived: BTreeSet<RequirementId>,
}

/// Tracer walks a graph and builds TracingData
///
/// When adding a Fork (all tines at once) add all lower Reqs to
/// `derived`, add all  reqs to `requirements`. for all upper, find a lower that covers or is
/// depent on. if not found, add to uncovered
#[derive(Debug)]
struct Tracer {
    /// Artefacts indexed by their name
    traced_artefacts: BTreeMap<ArtefactId, TracedArtefact>,

    /// Relations
    relations: Vec<TracedRelation>,

    /// Errors while Tracing
    errors: Vec<Error>,

    /// Map all requirement IDs to the Requirement and Artefact they come from
    req_by_id: BTreeMap<RequirementId, (ArtefactId, Rc<Requirement>)>,

    /// List of all depends-links that were not covered yet. later turned into errors
    invalid_depends_links: BTreeSet<(RequirementId, RequirementId)>,
    /// List of all covers-links that were not covered yet. later turned into errors
    invalid_covers_links: BTreeSet<(RequirementId, RequirementId)>,
}

impl Tracer {
    fn new(graph: &Graph) -> Self {
        let mut tracer = Tracer {
            traced_artefacts: BTreeMap::new(),
            relations: Vec::new(),
            errors: Vec::new(),
            req_by_id: BTreeMap::new(),
            invalid_depends_links: BTreeSet::new(),
            invalid_covers_links: BTreeSet::new(),
        };

        for a in graph.artefacts.values() {
            tracer.add_artefact(a);
        }
        for rel in &graph.relations {
            tracer.trace_relation(rel);
        }

        tracer.validate();
        tracer
    }
    fn data(self) -> TracedGraph {
        let Self {
            traced_artefacts: artefacts,
            relations,
            errors,
            ..
        } = self;

        let (artefacts, derived) = artefacts
            .into_iter()
            .map(|(id, ta)| {
                (
                    (id.clone(), ta.artefact),
                    (id, ta.derived.into_iter().collect()),
                )
            })
            .unzip();

        TracedGraph {
            artefacts,
            traced_relations: relations,
            derived,
            errors,
        }
    }

    fn add_artefact(&mut self, artefact: &Rc<Artefact>) {
        let mut derived = BTreeSet::new();

        let mut requirements_by_id = BTreeMap::new();
        let mut requirements_that_cover = BTreeMap::new();

        for req in artefact.requirements.values() {
            match self.req_by_id.entry(req.id.clone()) {
                btree_map::Entry::Occupied(e) => {
                    requirement_covered!(DSG_TRACE_DETECT_DUPLICATE);
                    let (_old_art, old_req) = e.get();
                    let err = DuplicateRequirement(Rc::clone(old_req), Rc::clone(req));
                    self.errors.push(err);
                }
                btree_map::Entry::Vacant(e) => {
                    e.insert((artefact.id.clone(), Rc::clone(req)));
                    // mark the new requirement's covers and depends links as invalid;
                    // later, in add_coverage each covered connection is removed.
                    for cov in &req.covers {
                        requirement_covered!(DSG_TRACE_REFERENCE_EXIST);
                        self.invalid_covers_links
                            .insert((cov.id.clone(), req.id.clone()));
                    }
                    for dep in &req.depends {
                        requirement_covered!(DSG_TRACE_REFERENCE_EXIST);
                        self.invalid_depends_links
                            .insert((req.id.clone(), dep.id.clone()));
                    }

                    if !artefact.ignore_derived_requirements {
                        derived.insert(req.id.clone());
                    }
                    requirements_by_id.insert(req.id.clone(), Rc::clone(req));
                    for cov in &req.covers {
                        match requirements_that_cover.entry(cov.id.clone()) {
                            std::collections::btree_map::Entry::Vacant(e) => {
                                e.insert(vec![(Rc::clone(req), cov.clone())]);
                            }
                            std::collections::btree_map::Entry::Occupied(mut e) => {
                                e.get_mut().push((Rc::clone(req), cov.clone()))
                            }
                        }
                    }
                }
            }
        }

        let ta = TracedArtefact {
            artefact: Rc::clone(artefact),
            requirements_by_id,
            requirements_that_cover,
            derived,
        };

        self.traced_artefacts.insert(ta.artefact.id.clone(), ta);
    }

    /// Compute Tracing for one upper and some lower artefacts
    fn trace_relation(&mut self, relation: &Relation) {
        log::trace!("Trace {:?}", relation);

        let mut any_req_coverd = false;
        let mut covered: Vec<Coverage> = Vec::new();
        let mut uncovered: Vec<RequirementId> = Vec::new();

        let upper_artefact = Rc::clone(&self.traced_artefacts[&relation.upper].artefact);

        for upper_requirement in upper_artefact.requirements.values() {
            let mut is_covered = false;
            for depends in &upper_requirement.depends {
                requirement_covered!(
                    DSG_TRACE_DOWNWARDS,
                    "Trace downwards using `depends` attribute"
                );
                for lower in &relation.lower {
                    let lower_artefact = self
                        .traced_artefacts
                        .get_mut(lower)
                        .expect("artefacts of relations exist");

                    if let Some(lower_requirement) =
                        lower_artefact.requirements_by_id.get(&depends.id)
                    {
                        is_covered = true;
                        if let Some(title) = depends.title.as_ref() {
                            if Some(title) != lower_requirement.title.as_ref() {
                                self.errors.push(DependWithWrongTitle {
                                    upper: Rc::clone(upper_requirement),
                                    lower: Rc::clone(lower_requirement),
                                    wrong_title: title.into(),
                                    location: depends.location.clone(),
                                });
                            }
                        }
                        lower_artefact.derived.remove(&lower_requirement.id);
                        covered.push(Coverage {
                            upper: upper_requirement.id.clone(),
                            lower: lower_requirement.id.clone(),
                            location: depends.location.clone(),
                            direction: CoverageDirection::Downwards,
                        });

                        // TODO: avoid clones ?
                        self.invalid_depends_links
                            .remove(&(upper_requirement.id.clone(), lower_requirement.id.clone()));
                        requirement_covered!(DSG_TRACE_REFERENCE_EXIST);
                    }
                }
            }

            for lower in &relation.lower {
                let lower_artefact = self
                    .traced_artefacts
                    .get_mut(lower)
                    .expect("artefacts of relations exist");

                if let Some(lower) = lower_artefact
                    .requirements_that_cover
                    .get(&upper_requirement.id)
                {
                    for (lower_requirement, covers) in lower {
                        requirement_covered!(
                            DSG_TRACE_UPWARDS,
                            "Trace upwards using `covers` attribute"
                        );

                        is_covered = true;

                        if let Some(title) = covers.title.as_ref() {
                            if Some(title) != upper_requirement.title.as_ref() {
                                self.errors.push(CoveredWithWrongTitle {
                                    upper: Rc::clone(upper_requirement),
                                    lower: Rc::clone(lower_requirement),
                                    wrong_title: title.into(),
                                    location: covers.location.clone(),
                                });
                            }
                        }
                        lower_artefact.derived.remove(&lower_requirement.id);
                        covered.push(Coverage {
                            upper: upper_requirement.id.clone(),
                            lower: lower_requirement.id.clone(),
                            location: covers.location.clone(),
                            direction: CoverageDirection::Upwards,
                        });
                        self.invalid_covers_links // TODO: avoid clones
                            .remove(&(upper_requirement.id.clone(), lower_requirement.id.clone()));
                        requirement_covered!(DSG_TRACE_REFERENCE_EXIST);
                    }
                }
            }
            if !is_covered {
                uncovered.push(upper_requirement.id.clone());
            }
            any_req_coverd = any_req_coverd || is_covered;
        }

        if !any_req_coverd {
            self.errors.push(Error::UnusedRelation(relation.clone()));
        }

        self.relations.push(TracedRelation {
            relation: relation.clone(),
            covered,
            uncovered,
        });
    }

    fn validate(&mut self) {
        self.validate_upwards();
        self.validate_downwards();
    }

    fn validate_upwards(&mut self) {
        for cov in &self.invalid_covers_links {
            requirement_covered!(DSG_TRACE_REFERENCE_EXIST);
            let (upper_id, lower_id) = cov;

            let (art_id, req) = &self.req_by_id[&lower_id];

            let reference: &Reference = req
                .covers
                .iter()
                .find(|r| &r.id == upper_id)
                .expect("invalid link exists");
            let location = reference.location.clone();

            let err = Error::CoversUnknownRequirement(Rc::clone(req), upper_id.clone(), location);
            log::trace!("{}", err);
            self.errors.push(err);
        }
    }

    fn validate_downwards(&mut self) {
        for dep in &self.invalid_depends_links {
            requirement_covered!(DSG_TRACE_REFERENCE_EXIST);
            let (upper_id, lower_id) = dep;

            let (art_id, req) = &self.req_by_id[&upper_id];

            let reference = req.depends.iter().find(|r| &r.id == lower_id);

            if reference.is_none() {
                panic!("");
            }

            let location = reference.unwrap().location.clone();

            let err = Error::DependOnUnknownRequirement(Rc::clone(req), lower_id.clone(), location);
            log::trace!("{}", err);
            self.errors.push(err);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {}
}
