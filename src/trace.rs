use std::collections::HashMap;

use super::common::*;

enum ConfigError {
    DuplicateArtefactId(String),
}

struct Node<'a> {
    artefacts: Vec<Artefact<'a>>,
}

struct Edge {
    from: u16,
    to: u16,
    must_cover: bool,  // Every Requirement in `from` must be covered by one in `to`
    must_follow: bool, // Every Requirement in `to` must cover one in `to`
}

struct NodeWithEdges<'a>{
    node:Node<'a>,
    edges_out: Vec<Edge>,
    edges_in: Vec<(u16, u16),
}

pub struct Graph<'a> {
    nodes: Vec<NodeWithEdges>,
    edges_down: Vec<Vec<Edge>>,
    edges_up: Vec<Vec<(u16, u16)>>,

    artefact_id_to_node: HashMap<String, (u16, u16)>,
}

impl<'a> Graph<'a> {
    fn new() -> Self {
        let nodes = Vec::new();
        let artefact_id_to_node = HashMap::new();
        Self {
            nodes,
            artefact_id_to_node,
        }
    }

   pub fn add_node(&mut self, artefacts: Vec<Artefact<'a>>) -> Result<u16, ConfigError> {
        let node_id = self.nodes.len() as u16;

        for (i, a) in artefacts.iter().enumerate() {
            let e = self.artefact_id_to_node.entry(a.id.to_owned());
            match e {
                std::collections::hash_map::Entry::Occupied(_) => {
                    return Err(ConfigError::DuplicateArtefactId(a.id.to_owned()));
                }
                std::collections::hash_map::Entry::Vacant(e) => {
                    e.insert((node_id, i as u16));
                }
            }
        }

        let node = Node {artefacts};
        let node = NodeWithEdges{ node, edges_out:Vec::new(), edges_in:Vec::new(), };
        self.nodes.push(node);
        Ok(node_id)
    }

   pub fn add_edge(&mut self, from:u16, to:u16, must_cover:bool, must_follow:bool) ->Result<(), ConfigError> {
        let e = Edge{from ,to, must_cover, must_follow};

        let offset = self.nodes[from].edges_out.len();
        self.nodes[from].edges_out.push(e);
        self.nodes[to].edges_in.push((from, offset));

        Ok(())
   }
}

#[allow(dead_code)]
pub fn trace(_up: &[Requirement], _down: &[Requirement]) {
    todo!();
}
