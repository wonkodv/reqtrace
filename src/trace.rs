use std::collections::HashMap;

use super::common::*;

#[derive(Debug, PartialEq)]
pub enum ConfigError<'a> {
    DuplicateArtefactId(&'a str),
    UnknownArtefactId(&'a str),
}

#[derive(Debug, PartialEq)]
pub enum TracingError<'a> {
    UnknownArtefactId(&'a str),
    DuplicateRequirement(&'a Requirement, &'a Requirement),
    CoveredWithWrongTitle(&'a Requirement, &'a Requirement, &'a str),
    DependWithWrongTitle(&'a Requirement, &'a Requirement, &'a str),
}

pub enum Coverage<'a> {
    Covered,
    CoveredWithTitle(&'a str),
    Depends,
    DependsWithTitle(&'a str),
}

pub struct Tracing<'a> {
    pub covered: Vec<(&'a Requirement, &'a Requirement, Coverage<'a>)>,
    pub uncovered: Vec<&'a Requirement>,
    pub derived: Vec<&'a Requirement>,
    pub errors: Vec<TracingError<'a>>,
}

struct Node<'a> {
    artefact: Artefact<'a>,
    upper_nodes: Vec<u16>,
    lower_nodes: Vec<u16>,
}

pub struct Graph<'a> {
    nodes: Vec<Node<'a>>,
    artefact_id_to_node: HashMap<String, u16>,
}

impl<'a> Graph<'a> {
    pub fn new() -> Self {
        let nodes = Vec::new();
        let artefact_id_to_node = HashMap::new();
        Self {
            nodes,
            artefact_id_to_node,
        }
    }

    pub fn add_artefact(&mut self, artefact: Artefact<'a>) -> Result<(), ConfigError<'a>> {
        let node_id = self.nodes.len() as u16;

        let e = self.artefact_id_to_node.entry(artefact.id.to_owned());
        match e {
            std::collections::hash_map::Entry::Occupied(_) => {
                return Err(ConfigError::DuplicateArtefactId(&artefact.id));
            }
            std::collections::hash_map::Entry::Vacant(e) => {
                e.insert(node_id);
            }
        }

        let node = Node {
            artefact,
            upper_nodes: Vec::new(),
            lower_nodes: Vec::new(),
        };
        self.nodes.push(node);
        Ok(())
    }

    pub fn add_edge<'r>(&mut self, upper: &'r str, lower: &'r str) -> Result<(), ConfigError<'r>> {
        let upper = *self
            .artefact_id_to_node
            .get(upper)
            .ok_or(ConfigError::UnknownArtefactId(upper))?;
        let lower = *self
            .artefact_id_to_node
            .get(lower)
            .ok_or(ConfigError::UnknownArtefactId(lower))?;

        self.nodes
            .get_mut(upper as usize)
            .unwrap()
            .lower_nodes
            .push(lower);
        self.nodes
            .get_mut(lower as usize)
            .unwrap()
            .upper_nodes
            .push(upper);

        Ok(())
    }

    fn get_node(&'a self, id: &str) -> Option<&'a Node<'a>> {
        if let Some(idx) = self.artefact_id_to_node.get(id) {
            return Some(self.nodes.get(*idx as usize).unwrap());
        }
        return None;
    }

    pub fn get_artefact(&'a self, id: &str) -> Option<&'a Artefact<'a>> {
        if let Some(node) = self.get_node(id) {
            return Some(&node.artefact);
        }
        return None;
    }

    pub fn get_upper(&'a self, id: &str) -> Option<Vec<&'a Artefact<'a>>> {
        if let Some(node) = self.get_node(id) {
            let r = node
                .upper_nodes
                .iter()
                .map(|idx| &self.nodes[*idx as usize].artefact)
                .collect();
            return Some(r);
        }
        return None;
    }

    pub fn get_lower(&'a self, id: &str) -> Option<Vec<&'a Artefact<'a>>> {
        if let Some(node) = self.get_node(id) {
            let r = node
                .lower_nodes
                .iter()
                .map(|idx| &self.nodes[*idx as usize].artefact)
                .collect();
            return Some(r);
        }
        return None;
    }

    pub fn trace_shallow(&'a mut self, id: &'a str) -> Tracing<'a> {
        let root_node;
        if let Some(n) = self.artefact_id_to_node.get(id) {
            root_node = n;
        } else {
            return Tracing {
                covered: vec![],
                uncovered: vec![],
                derived: vec![],
                errors: vec![TracingError::UnknownArtefactId(id)],
            };
        }

        let lower_node_ids;
        {
            let node = self.nodes.get_mut(*root_node as usize).unwrap();
            node.artefact.load();
            lower_node_ids = node.lower_nodes.clone();
        }
        for id in &lower_node_ids {
            self.nodes.get_mut(*id as usize).unwrap().artefact.load();
        }

        let root_artefact = &self.nodes.get(*root_node as usize).unwrap().artefact;
        let mut lower_artefacts: Vec<_> = Vec::new();
        for idx in lower_node_ids {
            lower_artefacts.push(&self.nodes.get(idx as usize).unwrap().artefact);
        }
        let lower_artefacts = lower_artefacts;

        let mut errors: Vec<TracingError> = Vec::new();
        let mut derived: HashMap<&String, &Requirement> = HashMap::new();
        let mut covered: Vec<(&Requirement, &Requirement, Coverage)> = Vec::new();
        let mut uncovered: Vec<&Requirement> = Vec::new();

        for la in &lower_artefacts {
            for lr in la.get_requirements() {
                let old = derived.insert(&lr.id, &lr);
                if let Some(old) = old {
                    errors.push(TracingError::DuplicateRequirement(old, lr));
                }
            }
        }

        for ur in root_artefact.get_requirements() {
            let mut is_covered = false;
            for depends in &ur.depends {
                for la in &lower_artefacts {
                    if let Some(lr) = la.get_requirement_with_id(&depends.id) {
                        is_covered = true;
                        if let Some(title) = depends.title.as_ref() {
                            covered.push((ur, lr, Coverage::DependsWithTitle(&title)));
                        } else {
                            covered.push((ur, lr, Coverage::Depends));
                        }
                    }
                }
            }
            for la in &lower_artefacts {
                for (lr, title) in la.get_requirements_that_cover(&ur.id) {
                    is_covered = true;
                    if let Some(title) = title {
                        covered.push((ur, lr, Coverage::CoveredWithTitle(title)));
                    } else {
                        covered.push((ur, lr, Coverage::Covered));
                    }
                }
            }
            if !is_covered {
                uncovered.push(ur);
            }
        }

        for (ur, lr, t) in &covered {
            derived.remove(&lr.id);
            if let Coverage::CoveredWithTitle(ref_title) = t {
                if let Some(upper_title) = &ur.title {
                    if upper_title != *ref_title {
                        errors.push(TracingError::CoveredWithWrongTitle(ur, lr, ref_title));
                    }
                } else {
                    errors.push(TracingError::CoveredWithWrongTitle(ur, lr, ref_title));
                }
            } else if let Coverage::DependsWithTitle(ref_title) = t {
                if let Some(upper_title) = &ur.title {
                    if upper_title != *ref_title {
                        errors.push(TracingError::DependWithWrongTitle(ur, lr, ref_title));
                    }
                } else {
                    errors.push(TracingError::DependWithWrongTitle(ur, lr, ref_title));
                }
            }
        }

        let derived = derived.values().map(|r| *r).collect();

        Tracing {
            covered,
            uncovered,
            derived,
            errors,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_graph<'a>() -> Graph<'a> {
        let a_req = Artefact::new("REQ", ArtefactConfig::PrePopulated(vec![]));
        let a_dsg = Artefact::new("DSG", ArtefactConfig::PrePopulated(vec![]));
        let a_code = Artefact::new("Code", ArtefactConfig::PrePopulated(vec![]));
        let a_dt = Artefact::new("DTests", ArtefactConfig::PrePopulated(vec![]));
        let a_rt = Artefact::new("RTests", ArtefactConfig::PrePopulated(vec![]));

        let mut g = Graph::new();
        g.add_artefact(a_req).unwrap();
        g.add_artefact(a_dsg).unwrap();
        g.add_artefact(a_code).unwrap();
        g.add_artefact(a_dt).unwrap();
        g.add_artefact(a_rt).unwrap();

        g.add_edge("REQ", "DSG").unwrap();
        g.add_edge("DSG", "Code").unwrap();
        g.add_edge("DSG", "DTests").unwrap();
        g.add_edge("REQ", "RTests").unwrap();

        g
    }

    #[test]
    fn test_graph_traversal() {
        let g = make_graph();

        let r = g.get_upper("Code").unwrap();
        assert_eq!(r.len(), 1);
        assert_eq!("DSG", r[0].id);
        let r = g.get_lower("REQ").unwrap();
        assert_eq!(r.len(), 2);
        assert_eq!("DSG", r[0].id);
        assert_eq!("RTests", r[1].id);
    }
}
