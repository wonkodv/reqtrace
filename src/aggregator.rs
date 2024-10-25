use std::collections::BTreeMap;

use crate::models::*;

#[derive(Debug )]
pub struct RequirementTrace<'g> {
    pub up_coverage: BTreeMap<&'g Relation, Vec<&'g Coverage>>,
    pub down_coverage: BTreeMap<&'g Relation, Vec<&'g Coverage>>,
}

#[derive(Debug)]
pub struct ArtefactRelations<'g>{
    pub upper:Vec<&'g Relation>,
    pub lower:Vec<&'g Relation>,
}

#[derive(Debug)]
pub struct Aggregator<'g> {
    traced_graph: &'g TracedGraph,

    artefact_relations: BTreeMap<ArtefactId, ArtefactRelations>,

    requirement_trace: BTreeMap<&'g RequirementId, RequirementTrace<'g>>,
}

impl<'g> Aggregator<'g> {
    pub fn new(traced_graph: &'g TracedGraph) -> Self {

        for rel in traced_graph.relations {
            
        }

        let requirement_trace = traced_graph
            .artefacts
            .values()
            .flat_map(|a| a.requirements.iter())
            .map(|r| (&r.id, RequirementTrace{
                up_coverage: traced_graph.relations.iter.map(|rel| (&rel, Vec::new()),
                down_coverage: traced_graph.relations.iter.map(|rel| (&rel, Vec::new()),
            }))
            .collect();

        for rel in  {
            for rt in requirement_trace {
                rt.up_coverage.
            }
        }

        Self {
            traced_graph,
            requirement_trace,
        }
    }
}
