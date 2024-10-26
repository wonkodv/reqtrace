use std::collections::BTreeMap;

use crate::models::*;

/// Bundle with a requirement and its upper/lower relations and referenced requiremnts
#[derive(Debug)]
pub struct RequirementTrace<'g> {
    /// for all relations  above the requirement's artefact, a list
    /// of requirements that self covers along the relation and the location of the reference
    /// Empty lists are not an error
    pub covers: BTreeMap<&'g Relation, Vec<(&'g Requirement, &'g Location)>>,

    /// for all relations below the requirement's artefact, a list
    /// of requirements that cover self along the relation and the location of the reference
    /// an empty list means self is not covered along that relation
    pub depends: BTreeMap<&'g Relation, Vec<(&'g Requirement, &'g Location)>>,

    /// The requirement that this bundle describes
    pub requirement: &'g Requirement,
}

/// Bundle with an artefact and its upper/lower Relations
#[derive(Debug)]
pub struct ArtefactRelations<'g> {
    /// All Relations above self
    pub upper: Vec<&'g TracedRelation>,

    /// All Relations above self
    pub lower: Vec<&'g TracedRelation>,

    /// The artefact that this bundle describes
    pub artefact: &'g Artefact,
}

/// Where TracedGraph contains only deduplicated information, this holds all the cross references
/// to find all the info for an object directly
#[derive(Debug)]
pub struct AggregatedGraph<'g> {
    /// The graph these data are collected from
    pub traced_graph: &'g TracedGraph,

    /// All info on Artefact by artefact id
    pub artefacts: BTreeMap<ArtefactId, ArtefactRelations<'g>>,

    /// all info on requirement by requirement id
    pub requirements: BTreeMap<&'g RequirementId, RequirementTrace<'g>>,
}

impl<'g> AggregatedGraph<'g> {
    pub fn new(traced_graph: &'g TracedGraph) -> Self {
        let mut artefacts: BTreeMap<ArtefactId, ArtefactRelations<'g>> = traced_graph
            .artefacts
            .iter()
            .map(|(id, art)| {
                (
                    id.clone(),
                    ArtefactRelations {
                        upper: Vec::new(),
                        lower: Vec::new(),
                        artefact: art,
                    },
                )
            })
            .collect();

        let mut requirements: BTreeMap<&'g RequirementId, RequirementTrace<'g>> = traced_graph
            .artefacts
            .values()
            .flat_map(|a| a.requirements.iter())
            .map(|(id, req)| {
                (
                    id,
                    RequirementTrace {
                        covers: BTreeMap::new(),
                        depends: BTreeMap::new(),
                        requirement: req,
                    },
                )
            })
            .collect();

        for rel in &traced_graph.traced_relations {
            artefacts
                .get_mut(&rel.relation.upper)
                .unwrap()
                .lower
                .push(rel);
            for req_id in traced_graph.artefacts[&rel.relation.upper]
                .requirements
                .keys()
            {
                requirements
                    .get_mut(req_id)
                    .unwrap()
                    .depends
                    .insert(&rel.relation, vec![]);
            }
            for lower_art_id in &rel.relation.lower {
                artefacts.get_mut(lower_art_id).unwrap().upper.push(rel);
                for req_id in traced_graph.artefacts[lower_art_id].requirements.keys() {
                    requirements
                        .get_mut(req_id)
                        .unwrap()
                        .covers
                        .insert(&rel.relation, vec![]);
                }
            }
        }

        for rel in &traced_graph.traced_relations {
            for cov in &rel.covered {
                let upper_req = requirements[&cov.upper].requirement;
                let lower_req = requirements[&cov.lower].requirement;

                requirements
                    .get_mut(&cov.upper)
                    .unwrap()
                    .depends
                    .get_mut(&rel.relation)
                    .unwrap()
                    .push((lower_req, &cov.location));

                requirements
                    .get_mut(&cov.lower)
                    .unwrap()
                    .covers
                    .get_mut(&rel.relation)
                    .unwrap()
                    .push((upper_req, &cov.location));
            }
        }

        Self {
            traced_graph,
            requirements,
            artefacts,
        }
    }
}
