use std::collections::HashMap;
use std::ops::Range;
use std::rc::Rc;

use crate::common::{Artefact, Requirement};
use crate::errors::{Error, Result};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Fork(usize);
impl Into<usize> for Fork {
    fn into(self) -> usize {
        self.0.into()
    }
}
impl From<usize> for Fork {
    fn from(val: usize) -> Self {
        Self(val)
    }
}

impl Fork {
    fn as_ref<'a>(self, graph: &'a Graph) -> &'a ForkData {
        let i: usize = self.into();
        &graph.forks[i]
    }

    pub fn from(self, graph: &Graph) -> NodeIdx {
        self.as_ref(graph).from
    }

    pub fn tines<'g>(self, graph: &'g Graph) -> impl Iterator<Item = Tine> {
        (0..self.as_ref(graph).to.len())
            .into_iter()
            .map(move |tine| Tine { fork: self, tine })
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Tine {
    pub fork: Fork,
    pub tine: usize,
}

impl Tine {
    pub fn to<'a>(self, graph: &'a Graph) -> NodeIdx {
        self.fork.as_ref(graph).to[self.tine]
    }
    pub fn from(self, graph: &Graph) -> NodeIdx {
        self.fork.as_ref(graph).from
    }
}

/// Index of a Node in the Graphs list of nodes. Used instead of a pointer
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct NodeIdx(usize);

impl From<usize> for NodeIdx {
    fn from(val: usize) -> Self {
        Self(val)
    }
}
impl Into<usize> for NodeIdx {
    fn into(self) -> usize {
        self.0.into()
    }
}
impl NodeIdx {
    fn as_mut<'a>(self, graph: &'a mut Graph<'a>) -> &'a mut Node {
        let i: usize = self.into();
        graph.nodes.get_mut(i).unwrap()
    }
    fn as_ref<'a>(self, graph: &'a Graph<'a>) -> &'a Node {
        let i: usize = self.into();
        &graph.nodes[i]
    }
    pub fn lower<'a>(self, graph: &'a Graph<'a>) -> &'a [Fork] {
        self.as_ref(graph).forks_up.as_slice()
    }
    pub fn upper<'a>(self, graph: &'a Graph<'a>) -> &'a [Fork] {
        self.as_ref(graph).forks_down.as_slice()
    }
    pub fn artefact<'a>(self, graph: &'a Graph<'a>) -> &'a Artefact<'a> {
        &self.as_ref(graph).artefact
    }
}

/// Not really an Edge but a fork. Has one start and multiple ends.
/// All Requirements in the start node must be covered by a requirement in one of the ends
#[derive(Debug)]
struct ForkData {
    from: NodeIdx,
    to: Vec<NodeIdx>,
}

/// An artefact in the graph
#[derive(Debug)]
struct Node<'a> {
    artefact: Artefact<'a>,
    forks_up: Vec<Fork>,
    forks_down: Vec<Fork>,
}

/// The tracing Graph
#[derive(Debug)]
pub struct Graph<'a> {
    nodes: Vec<Node<'a>>,
    forks: Vec<ForkData>,

    artefact_id_to_node: HashMap<&'a str, NodeIdx>,
}

impl<'a> Graph<'a> {
    pub fn new() -> Self {
        let nodes = Vec::new();
        let forks = Vec::new();
        let artefact_id_to_node = HashMap::new();
        Self {
            nodes,
            forks,
            artefact_id_to_node,
        }
    }

    fn node_mut(&mut self, idx: NodeIdx) -> &mut Node<'a> {
        let idx: usize = idx.into();
        self.nodes.get_mut(idx).unwrap()
    }
    fn node_ref(&self, idx: NodeIdx) -> &Node<'a> {
        let idx: usize = idx.into();
        self.nodes.get(idx).unwrap()
    }

    fn fork_ref(&self, idx: Fork) -> &ForkData {
        let idx: usize = idx.into();
        self.forks.get(idx).unwrap()
    }

    pub fn node_idx_by_id(&self, id: &str) -> Result<NodeIdx> {
        return Ok(*self
            .artefact_id_to_node
            .get(id)
            .ok_or(Error::UnknownArtefact(id.into()))?);
    }

    /// Add [`Artefact`] to the graph
    ///
    /// # Errors
    /// Returns [`DuplicateArtefactId`] if an artefact with the same id was
    /// already registered
    pub fn add_artefact(&mut self, artefact: Artefact<'a>) -> Result<()> {
        let node_id: NodeIdx = self.nodes.len().into();

        let e = self.artefact_id_to_node.entry(artefact.id);
        match e {
            std::collections::hash_map::Entry::Occupied(_) => {
                return Err(Error::DuplicateArtefact(artefact.id.into()));
            }
            std::collections::hash_map::Entry::Vacant(e) => {
                e.insert(node_id);
            }
        }

        let node = Node {
            artefact,
            forks_up: Vec::new(),
            forks_down: Vec::new(),
        };
        self.nodes.push(node);
        Ok(())
    }

    /// Add a Fork
    ///
    /// Every Requirement in the artefact `upper` must be covered by a requirement in one of the
    /// artefacts of `lower`.
    ///
    /// # Arguments
    ///
    /// *   `upper` is the id of a previously added [`Artefact`]
    /// *   `lower` is a list of ids of previously added [`Artefact`]s
    pub fn add_fork<'r, S: AsRef<str>, I: Iterator<Item = S>>(
        &mut self,
        upper: &'r str,
        lower: I,
    ) -> Result<()> {
        let fork_idx: Fork = self.forks.len().into();
        let mut lower_indexes: Vec<NodeIdx> = vec![];
        let upper_idx: NodeIdx = self.node_idx_by_id(upper)?;
        self.node_mut(upper_idx).forks_up.push(fork_idx);

        for lower_artefact_id in lower {
            let lower_artefact_id: &str = lower_artefact_id.as_ref();
            let lower_idx: NodeIdx = self.node_idx_by_id(lower_artefact_id)?;
            lower_indexes.push(lower_idx);
            self.node_mut(lower_idx).forks_up.push(fork_idx);
        }

        self.forks.push(ForkData {
            from: upper_idx,
            to: lower_indexes,
        });
        Ok(())
    }

    pub fn get_artefact<'i>(&'a self, id: &'i str) -> Result<&'a Artefact<'a>> {
        let node_idx = self.node_idx_by_id(id)?;
        let node = self.node_ref(node_idx);
        Ok(&node.artefact)
    }

    pub fn get_upper<'i>(&'a self, id: &'i str) -> Result<Vec<&'a Artefact<'a>>> {
        let node_idx = self.node_idx_by_id(id)?;
        let node = self.node_ref(node_idx);
        let r = node
            .forks_up
            .iter()
            .map(|fork_idx| self.fork_ref(*fork_idx).from)
            .map(|node_idx| &self.node_ref(node_idx).artefact)
            .collect();
        Ok(r)
    }

    pub fn get_lower<'i>(&'a self, id: &'i str) -> Result<Vec<Vec<&'a Artefact<'a>>>> {
        let node_idx = self.node_idx_by_id(id)?;
        let node = self.node_ref(node_idx);
        let r = node
            .forks_up
            .iter()
            .map(|fork_idx| {
                self.fork_ref(*fork_idx)
                    .to
                    .iter()
                    .map(|node_idx| &self.node_ref(*node_idx).artefact)
                    .collect()
            })
            .collect();
        Ok(r)
    }

    pub fn find_tine_from_to(&self, from: &'a str, to: &'a str) -> Result<Tine> {
        let root_node = if let Some(n) = self.artefact_id_to_node.get(from) {
            *n
        } else {
            return Err(Error::UnknownArtefact(from.into()));
        };

        for fork in root_node.lower(self) {
            for tine in fork.tines(self) {
                if tine.to(self).artefact(self).id == to {
                    return Ok(tine);
                }
            }
        }
        return Err(Error::UnknownFork(from.into(), to.into()));
    }

    pub fn get_parsing_errors<'r>(&'r self) -> impl Iterator<Item = &'r Error> {
        self.nodes
            .iter()
            .flat_map(|node| node.artefact.get_errors())
    }

    pub fn get_all_reqs<'r>(&'r self) -> impl Iterator<Item = &'r Rc<Requirement>> {
        self.nodes
            .iter()
            .flat_map(|node| node.artefact.get_requirements())
    }

    pub fn iter_forks(&self) -> impl Iterator<Item = Fork> {
        (0..self.forks.len()).into_iter().map(|id| Fork(id))
    }

    pub fn iter_nodes(&self) -> impl Iterator<Item = NodeIdx> {
        (0..self.nodes.len()).into_iter().map(|id| NodeIdx(id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::*;

    fn make_graph<'a>() -> Graph<'a> {
        let a_req = Artefact::new("REQ", ArtefactConfig::PrePopulated(vec![]));
        let a_dsg = Artefact::new("DSG", ArtefactConfig::PrePopulated(vec![]));
        let a_fmt = Artefact::new("FORMAT", ArtefactConfig::PrePopulated(vec![]));
        let a_code = Artefact::new("Code", ArtefactConfig::PrePopulated(vec![]));
        let a_dt = Artefact::new("DTests", ArtefactConfig::PrePopulated(vec![]));
        let a_rt = Artefact::new("RTests", ArtefactConfig::PrePopulated(vec![]));

        let mut g = Graph::new();
        g.add_artefact(a_req).unwrap();
        g.add_artefact(a_dsg).unwrap();
        g.add_artefact(a_fmt).unwrap();
        g.add_artefact(a_code).unwrap();
        g.add_artefact(a_dt).unwrap();
        g.add_artefact(a_rt).unwrap();

        g.add_edge_group("REQ", ["DSG", "FORMAT"].iter()).unwrap();
        g.add_edge_group("DSG", ["Code", "FORMAT"].iter()).unwrap();
        g.add_edge_group("FORMAT", ["Code"].iter()).unwrap();
        g.add_edge_group("DSG", ["DTests"].iter()).unwrap();
        g.add_edge_group("REQ", ["RTests"].iter()).unwrap();

        g
    }

    #[test]
    fn test_graph_traversal() {
        let g = make_graph();

        let r = g.get_upper("Code").unwrap();
        assert_eq!(r.len(), 2);
        assert_eq!("DSG", r[0].id);
        assert_eq!("FORMAT", r[1].id);
        let r = g.get_lower("REQ").unwrap();
        assert_eq!(r.len(), 2);
        assert_eq!("DSG", r[0][0].id);
        assert_eq!("FORMAT", r[0][1].id);
        assert_eq!("RTests", r[1][0].id);
    }
}
