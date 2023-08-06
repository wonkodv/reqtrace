use std::collections::BTreeMap;
use std::rc::Rc;

use crate::common::{Artefact, Requirement};
use crate::errors::{Error, Result};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Fork(usize);
impl From<Fork> for usize {
    fn from(val: Fork) -> Self {
        val.0
    }
}
impl From<usize> for Fork {
    fn from(val: usize) -> Self {
        Self(val)
    }
}

impl Fork {
    fn as_ref(self, graph: &Graph) -> &ForkData {
        let i: usize = self.into();
        &graph.forks[i]
    }

    pub fn from(self, graph: &Graph) -> NodeIdx {
        self.as_ref(graph).from
    }

    pub fn tines(self, graph: &Graph) -> impl Iterator<Item = Tine> {
        (0..self.as_ref(graph).to.len()).map(move |tine| Tine { fork: self, tine })
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Tine {
    pub fork: Fork,
    pub tine: usize,
}

impl Tine {
    pub fn to(self, graph: &Graph) -> NodeIdx {
        self.fork.as_ref(graph).to[self.tine]
    }
    pub fn from(self, graph: &Graph) -> NodeIdx {
        self.fork.as_ref(graph).from
    }
}

/// Index of a Node in the Graphs list of nodes. Used instead of a pointer
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct NodeIdx(usize);

impl From<usize> for NodeIdx {
    fn from(val: usize) -> Self {
        Self(val)
    }
}
impl From<NodeIdx> for usize {
    fn from(val: NodeIdx) -> Self {
        val.0
    }
}
impl NodeIdx {
    fn as_mut<'a>(self, graph: &'a mut Graph) -> &'a mut Node {
        let i: usize = self.into();
        graph.nodes.get_mut(i).unwrap()
    }
    fn as_ref<'a>(self, graph: &'a Graph) -> &'a Node {
        let i: usize = self.into();
        &graph.nodes[i]
    }
    pub fn lower<'a>(self, graph: &'a Graph) -> &'a [Fork] {
        self.as_ref(graph).forks_down.as_slice()
    }
    pub fn upper<'a>(self, graph: &'a Graph) -> &'a [Fork] {
        self.as_ref(graph).forks_up.as_slice()
    }
    pub fn artefact<'a>(self, graph: &'a Graph) -> &'a Artefact {
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
struct Node {
    artefact: Artefact,
    forks_up: Vec<Fork>,
    forks_down: Vec<Fork>,
}

/// The tracing Graph
#[derive(Debug)]
pub struct Graph {
    nodes: Vec<Node>,
    forks: Vec<ForkData>,

    artefact_id_to_node: BTreeMap<String, NodeIdx>,
}

impl Graph {
    pub fn new() -> Self {
        let nodes = Vec::new();
        let forks = Vec::new();
        let artefact_id_to_node = BTreeMap::new();
        Self {
            nodes,
            forks,
            artefact_id_to_node,
        }
    }

    fn node_mut(&mut self, idx: NodeIdx) -> &mut Node {
        let idx: usize = idx.into();
        self.nodes.get_mut(idx).unwrap()
    }

    fn node_ref(&self, idx: NodeIdx) -> &Node {
        let idx: usize = idx.into();
        self.nodes.get(idx).unwrap()
    }

    fn fork_ref(&self, idx: Fork) -> &ForkData {
        let idx: usize = idx.into();
        self.forks.get(idx).unwrap()
    }

    pub fn node_idx_by_id(&self, id: &str) -> Result<NodeIdx> {
        Ok(*self
            .artefact_id_to_node
            .get(id)
            .ok_or_else(|| Error::UnknownArtefact(id.into()))?)
    }

    /// Add [`Artefact`] to the graph
    ///
    /// # Errors
    /// Returns [`DuplicateArtefactId`] if an artefact with the same id was
    /// already registered
    pub fn add_artefact(&mut self, artefact: Artefact) -> Result<()> {
        let node_id: NodeIdx = self.nodes.len().into();

        let e = self.artefact_id_to_node.entry(artefact.id.clone());
        match e {
            std::collections::btree_map::Entry::Occupied(_) => {
                return Err(Error::DuplicateArtefact(artefact.id.clone()));
            }
            std::collections::btree_map::Entry::Vacant(e) => {
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
        self.node_mut(upper_idx).forks_down.push(fork_idx);

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

    pub fn get_artefact<'a, 'i>(&'a self, id: &'i str) -> Result<&'a Artefact> {
        let node_idx = self.node_idx_by_id(id)?;
        let node = self.node_ref(node_idx);
        Ok(&node.artefact)
    }

    pub fn find_tine_from_to(&self, from: &str, to: &str) -> Result<Tine> {
        let from_nid = if let Some(n) = self.artefact_id_to_node.get(from) {
            *n
        } else {
            return Err(Error::UnknownArtefact(from.into()));
        };

        let to_nid = if let Some(n) = self.artefact_id_to_node.get(to) {
            *n
        } else {
            return Err(Error::UnknownArtefact(to.into()));
        };

        for fork in from_nid.lower(self) {
            for tine in fork.tines(self) {
                if tine.to(self) == to_nid {
                    return Ok(tine);
                }
            }
        }
        Err(Error::UnknownFork(from.into(), to.into()))
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
        (0..self.forks.len()).map(Fork)
    }

    pub fn iter_nodes(&self) -> impl Iterator<Item = NodeIdx> {
        (0..self.nodes.len()).map(NodeIdx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{common::*, parsers};

    fn make_graph() -> Graph {
        let a_req = Artefact::new(
            "REQ".into(),
            parsers::PrePopulated::default().into_artefact_parser(),
        );
        let a_dsg = Artefact::new(
            "DSG".into(),
            parsers::PrePopulated::default().into_artefact_parser(),
        );
        let a_fmt = Artefact::new(
            "FORMAT".into(),
            parsers::PrePopulated::default().into_artefact_parser(),
        );
        let a_code = Artefact::new(
            "Code".into(),
            parsers::PrePopulated::default().into_artefact_parser(),
        );
        let a_dt = Artefact::new(
            "DTests".into(),
            parsers::PrePopulated::default().into_artefact_parser(),
        );
        let a_rt = Artefact::new(
            "RTests".into(),
            parsers::PrePopulated::default().into_artefact_parser(),
        );

        let mut g = Graph::new();
        g.add_artefact(a_req).unwrap();
        g.add_artefact(a_dsg).unwrap();
        g.add_artefact(a_fmt).unwrap();
        g.add_artefact(a_code).unwrap();
        g.add_artefact(a_dt).unwrap();
        g.add_artefact(a_rt).unwrap();

        g.add_fork("REQ", ["DSG", "FORMAT"].iter()).unwrap();
        g.add_fork("DSG", ["Code", "FORMAT"].iter()).unwrap();
        g.add_fork("FORMAT", ["Code"].iter()).unwrap();
        g.add_fork("DSG", ["DTests"].iter()).unwrap();
        g.add_fork("REQ", ["RTests"].iter()).unwrap();

        g
    }

    #[test]
    fn node_upper() {
        let g = &make_graph();

        let node = g.node_idx_by_id("FORMAT").unwrap();

        let lower = node.upper(g);

        assert_eq!(lower.len(), 2);
        assert_eq!("REQ", lower[0].from(g).artefact(g).id);
        assert_eq!("DSG", lower[1].from(g).artefact(g).id);
    }

    #[test]
    fn graph_get_lower() {
        let g = &make_graph();

        let node = g.node_idx_by_id("FORMAT").unwrap();

        let lower = node.lower(g);

        assert_eq!(lower.len(), 1);
        assert_eq!(
            "Code",
            lower[0].tines(g).next().unwrap().to(g).artefact(g).id
        );
    }

    #[test]
    fn graph_find_tine() {
        let g = &make_graph();

        let t = g.find_tine_from_to("DSG", "Code").unwrap();
        assert_eq!(t.from(g).as_ref(g).artefact.id, "DSG");
    }

    #[test]
    fn fork_tines() {
        let g = &make_graph();

        let dsg = g.node_idx_by_id("DSG").unwrap();
        let req_to_dsg = dsg.upper(g)[0];
        let dsg_siblings: Vec<_> = req_to_dsg.tines(g).map(|t| t.to(g).as_ref(g)).collect();

        assert_eq!(dsg_siblings.len(), 2);
        assert_eq!("DSG", dsg_siblings[0].artefact.id);
        assert_eq!("FORMAT", dsg_siblings[1].artefact.id);
    }
}
