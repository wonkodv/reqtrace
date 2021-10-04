use std::{
    collections::{HashMap, HashSet},
    error, fmt,
    rc::Rc,
};

use super::common::*;
use crate::errors::{Error, Result};
use Error::*;

#[derive(Debug)]
pub enum Coverage<'a> {
    Covered,
    CoveredWithTitle(&'a str),
    Depends,
    DependsWithTitle(&'a str),
}

#[derive(Debug, PartialEq)]
pub struct DerivedRequirement<'a> {
    pub req: &'a Rc<Requirement>,
    node: NodeIdx,
}

#[derive(Debug)]
pub struct TracedRequirement {
    pub requirement: Rc<Requirement>,
    pub upper: Vec<Rc<Requirement>>,
    pub lower: Vec<Rc<Requirement>>,
    pub artefact: NodeIdx,
}

#[derive(Debug)]
pub struct CoveredRequirement<'a> {
    pub upper: &'a Rc<Requirement>,
    pub lower: &'a Rc<Requirement>,
    pub coverage: Coverage<'a>,
    edge: (EdgeIdx, u16),
}

#[derive(Debug)]
pub struct UncoveredRequirement<'a> {
    pub req: &'a Rc<Requirement>,
    edges: Vec<(EdgeIdx, u16)>,
}

#[derive(Debug, Default)]
pub struct Tracing<'a> {
    edges: HashSet<(EdgeIdx, u16)>, // TODO: for Edge Groups not u16
    pub coverages: Vec<CoveredRequirement<'a>>,
    pub uncovered: HashMap<&'a str, UncoveredRequirement<'a>>,
    pub derived: HashMap<&'a str, DerivedRequirement<'a>>,
    pub errors: Vec<Error>,
}

impl<'a> Tracing<'a> {
    pub fn add(mut self, mut other: Tracing<'a>) -> Result<Self> {
        assert!(self.edges.is_disjoint(&other.edges));

        for e in other.edges {
            self.edges.insert(e);
        }

        self.errors.append(&mut other.errors);

        for cov in &self.coverages {
            other.derived.remove(cov.lower.id.as_str());
            other.uncovered.remove(cov.upper.id.as_str());
        }
        for cov in other.coverages {
            self.derived.remove(cov.lower.id.as_str());
            self.uncovered.remove(cov.upper.id.as_str());
            self.coverages.push(cov);
        }

        for (id, mut uncov) in other.uncovered {
            match self.uncovered.entry(id) {
                std::collections::hash_map::Entry::Vacant(e) => {
                    e.insert(uncov);
                }
                std::collections::hash_map::Entry::Occupied(mut e) => {
                    let old: &mut UncoveredRequirement = e.get_mut();
                    if uncov.req != old.req {
                        // if IDs equal, REQs should be equal
                        self.errors.push(DuplicateRequirement(
                            Rc::clone(uncov.req),
                            Rc::clone(old.req),
                        ));
                    } else {
                        old.edges.append(&mut uncov.edges);
                    }
                }
            }
        }

        for (id, der) in other.derived {
            self.derived.insert(id, der);
        }

        Ok(self)
    }

    //pub fn artefacts(&'a self, graph: &'a Graph) -> Vec<&'a Artefact> {
    //    // TODO: result lifetime hangs on graph not self ?
    //    let mut set: HashSet<NodeIdx> = HashSet::new();
    //    for (e,g) in &self.edges {
    //
    //        let e: &Edge = e.as_ref(graph);
    //        let from: NodeIdx = e.from;
    //        let to: NodeIdx = e.to[*g as usize];
    //        set.insert(from);
    //        set.insert(to);
    //    }
    //
    //    set.iter().map(|nid| &nid.as_ref(graph).artefact).collect()
    //}

    pub fn errors(&self) -> &[Error] {
        self.errors.as_slice()
    }

    pub fn uncovered<'r>(&'r self) -> impl Iterator<Item = &'r Rc<Requirement>> {
        self.uncovered.iter().map(|(_, r)| r.req)
    }

    pub fn derived<'r>(&'r self) -> impl Iterator<Item = &'r Rc<Requirement>> {
        self.derived.iter().map(|(_, r)| r.req)
    }

    pub fn coverages<'r>(
        &'r self,
    ) -> impl Iterator<Item = (&'r Rc<Requirement>, &'r Rc<Requirement>)> {
        self.coverages.iter().map(|r| (r.upper, r.lower))
    }

    pub fn traced_requiremests(&self) -> Vec<TracedRequirement> {
        todo!();
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct EdgeIdx(u16);
impl Into<usize> for EdgeIdx {
    fn into(self) -> usize {
        self.0.into()
    }
}
impl From<usize> for EdgeIdx {
    fn from(val: usize) -> Self {
        Self(val as u16)
    }
}

impl EdgeIdx {
    fn as_ref<'a>(self, graph: &'a Graph) -> &'a Edge {
        let i: usize = self.into();
        &graph.edges[i]
    }
    fn to<'a>(self, graph: &'a Graph) -> &'a [NodeIdx] {
        self.as_ref(graph).to.as_slice()
    }
    fn from(self, graph: &Graph) -> NodeIdx {
        self.as_ref(graph).from
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct NodeIdx(u16);

impl From<usize> for NodeIdx {
    fn from(val: usize) -> Self {
        Self(val as u16)
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
    fn lower<'a>(self, graph: &'a Graph<'a>) -> &'a [EdgeIdx] {
        self.as_ref(graph).edges_up.as_slice()
    }
    fn upper<'a>(self, graph: &'a Graph<'a>) -> &'a [EdgeIdx] {
        self.as_ref(graph).edges_down.as_slice()
    }
}

#[derive(Debug)]
struct Edge {
    from: NodeIdx,
    to: Vec<NodeIdx>,
}

#[derive(Debug)]
struct Node<'a> {
    artefact: Artefact<'a>,
    edges_up: Vec<EdgeIdx>,
    edges_down: Vec<EdgeIdx>,
}

#[derive(Debug)]
pub struct Graph<'a> {
    nodes: Vec<Node<'a>>,
    edges: Vec<Edge>,

    artefact_id_to_node: HashMap<&'a str, NodeIdx>,
}

impl<'a> Graph<'a> {
    pub fn new() -> Self {
        let nodes = Vec::new();
        let edges = Vec::new();
        let artefact_id_to_node = HashMap::new();
        Self {
            nodes,
            edges,
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

    fn edge_ref(&self, idx: EdgeIdx) -> &Edge {
        let idx: usize = idx.into();
        self.edges.get(idx).unwrap()
    }

    fn node_idx_by_id(&self, id: &str) -> Result<NodeIdx> {
        return Ok(*self
            .artefact_id_to_node
            .get(id)
            .ok_or(UnknownArtefact(id.into()))?);
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
                return Err(DuplicateArtefact(artefact.id.into()));
            }
            std::collections::hash_map::Entry::Vacant(e) => {
                e.insert(node_id);
            }
        }

        let node = Node {
            artefact,
            edges_up: Vec::new(),
            edges_down: Vec::new(),
        };
        self.nodes.push(node);
        Ok(())
    }

    /// Add an EdgeGroup.
    ///
    /// Every Requirement in the artefact `upper` must be covered by a requirement in one of the
    /// artefacts of `lower`.
    ///
    /// # Arguments
    ///
    /// *   `upper` is the id of a previously added [`Artefact`]
    /// *   `lower` is a list of ids of previously added [`Artefact`]s
    pub fn add_edge_group<'r, S: AsRef<str>, I: Iterator<Item = S>>(
        &mut self,
        upper: &'r str,
        lower: I,
    ) -> Result<()> {
        let edge_idx: EdgeIdx = self.edges.len().into();
        let mut lower_indexes: Vec<NodeIdx> = vec![];
        let upper_idx: NodeIdx = self.node_idx_by_id(upper)?;
        self.node_mut(upper_idx).edges_up.push(edge_idx);

        for lower_artefact_id in lower {
            let lower_artefact_id: &str = lower_artefact_id.as_ref();
            let lower_idx: NodeIdx = self.node_idx_by_id(lower_artefact_id)?;
            lower_indexes.push(lower_idx);
            self.node_mut(lower_idx).edges_up.push(edge_idx);
        }

        self.edges.push(Edge {
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
            .edges_up
            .iter()
            .map(|edge_idx| self.edge_ref(*edge_idx).from)
            .map(|node_idx| &self.node_ref(node_idx).artefact)
            .collect();
        Ok(r)
    }

    pub fn get_lower<'i>(&'a self, id: &'i str) -> Result<Vec<Vec<&'a Artefact<'a>>>> {
        let node_idx = self.node_idx_by_id(id)?;
        let node = self.node_ref(node_idx);
        let r = node
            .edges_up
            .iter()
            .map(|edge_idx| {
                self.edge_ref(*edge_idx)
                    .to
                    .iter()
                    .map(|node_idx| &self.node_ref(*node_idx).artefact)
                    .collect()
            })
            .collect();
        Ok(r)
    }

    fn find_edge_from_to(&self, from: &'a str, to: &'a str) -> Result<(EdgeIdx, u16)> {
        let root_node = if let Some(n) = self.artefact_id_to_node.get(from) {
            *n
        } else {
            return Err(UnknownArtefact(from.into()));
        };

        for edge in root_node.lower(self) {
            for (group, lower_node) in edge.to(self).iter().enumerate() {
                if lower_node.as_ref(self).artefact.id == to {
                    return Ok((*edge, group as u16));
                }
            }
        }
        return Err(UnknownEdge(from.into(), to.into()));
    }

    pub fn trace(&'a self) -> Result<Tracing<'a>> {
        let mut i = self.edges.iter().enumerate().flat_map(|(edix, e)| {
            e.to.iter()
                .enumerate()
                .map(move |(gidx, _g)| self.trace_edge_(edix.into(), gidx as u16))
        });

        if let Some(first_tracing) = i.next() {
            let mut t = first_tracing;

            for n in i {
                t = t.add(n)?;
            }

            return Ok(t);
        } else {
            return Err(EmptyGraph);
        }
    }

    pub fn trace_edge(&'a self, from: &'a str, to: &'a str) -> Tracing<'a> {
        match self.find_edge_from_to(from, to) {
            Err(_) => {
                let t = Tracing::default();
                return Tracing {
                    errors: vec![UnknownArtefact(from.into())],
                    ..t
                };
            }
            Ok((e, g)) => self.trace_edge_(e, g),
        }
    }

    fn trace_edge_(&'a self, edge: EdgeIdx, group: u16) -> Tracing<'a> {
        let upper_node_idx = edge.from(self);
        let lower_node_idx = edge.to(self)[group as usize];

        //TODO
        //  {
        //      let u : usize = upper_node_idx.into();
        //      self.nodes.get_mut(u).unwrap().artefact.load();
        //      let l:usize = lower_node_idx.into();
        //      self.nodes.get_mut(l).unwrap().artefact.load();
        //  }

        let upper_artefact = &upper_node_idx.as_ref(self).artefact;
        let lower_artefact = &lower_node_idx.as_ref(self).artefact;

        let mut edges: HashSet<(EdgeIdx, u16)> = HashSet::new();
        edges.insert((edge, group));
        let mut covered = Vec::new();
        let mut uncovered = HashMap::new();
        let mut errors = Vec::new();
        let mut derived: HashMap<_, _> = lower_artefact
            .get_requirements()
            .iter()
            .map(|r| {
                (
                    r.id.as_str(),
                    DerivedRequirement {
                        req: r,
                        node: lower_node_idx,
                    },
                )
            })
            .collect();

        for ur in upper_artefact.get_requirements() {
            let mut is_covered = false;
            for depends in &ur.depends {
                if let Some(lr) = lower_artefact.get_requirement_with_id(&depends.id) {
                    is_covered = true;
                    derived.remove(lr.id.as_str());
                    if let Some(title) = depends.title.as_ref() {
                        covered.push(CoveredRequirement {
                            upper: ur,
                            lower: lr,
                            edge: (edge, group),
                            coverage: Coverage::DependsWithTitle(&title),
                        });

                        let ok = if let Some(upper_title) = &ur.title {
                            upper_title == title
                        } else {
                            false
                        };
                        if !ok {
                            errors.push(DependWithWrongTitle(
                                Rc::clone(ur),
                                Rc::clone(lr),
                                title.into(),
                            ));
                        }
                    } else {
                        covered.push(CoveredRequirement {
                            upper: ur,
                            lower: lr,
                            edge: (edge, group),
                            coverage: Coverage::Depends,
                        });
                    }
                }
            }
            for (lr, title) in lower_artefact.get_requirements_that_cover(&ur.id) {
                is_covered = true;
                derived.remove(lr.id.as_str());
                if let Some(title) = title {
                    covered.push(CoveredRequirement {
                        upper: ur,
                        lower: lr,
                        coverage: Coverage::CoveredWithTitle(title),
                        edge: (edge, group),
                    });
                    let ok = if let Some(upper_title) = &ur.title {
                        upper_title == title
                    } else {
                        false
                    };
                    if !ok {
                        errors.push(CoveredWithWrongTitle(
                            Rc::clone(ur),
                            Rc::clone(lr),
                            title.into(),
                        ));
                    }
                } else {
                    covered.push(CoveredRequirement {
                        upper: ur,
                        lower: lr,
                        coverage: Coverage::Covered,
                        edge: (edge, group),
                    });
                }
            }
            if !is_covered {
                let old = uncovered.insert(
                    ur.id.as_str(),
                    UncoveredRequirement {
                        req: ur,
                        edges: vec![(edge, group)],
                    },
                );
                if let Some(old) = old {
                    errors.push(DuplicateRequirement(Rc::clone(old.req), Rc::clone(ur)));
                }
            }
        }

        Tracing {
            edges,
            coverages: covered,
            uncovered,
            derived,
            errors,
        }
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
}

#[cfg(test)]
mod tests {
    use super::*;

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
