use std::{
    collections::{HashMap, HashSet},
    error, fmt,
    rc::Rc,
};

use super::common::*;

pub mod errors {
    use super::*;
    #[derive(Debug)]
    pub struct UnknownArtefact<'a>(pub &'a str);

    impl<'a> fmt::Display for UnknownArtefact<'a> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "No Artefact with id {}", self.0)
        }
    }

    #[derive(Debug)]
    pub enum ConfigError<'a> {
        DuplicateArtefact(&'a str),
        UnknownArtefact(UnknownArtefact<'a>),
    }

    impl<'a> From<UnknownArtefact<'a>> for ConfigError<'a> {
        fn from(e: UnknownArtefact<'a>) -> Self {
            Self::UnknownArtefact(e)
        }
    }

    impl<'a> fmt::Display for ConfigError<'a> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                ConfigError::DuplicateArtefact(id) => write!(f, "Duplicate Artefact Id: {}", id),
                ConfigError::UnknownArtefact(ua) => ua.fmt(f),
            }
        }
    }

    impl<'a> error::Error for ConfigError<'a> {}

    #[derive(Debug)]
    pub enum TracingError<'a> {
        UnknownArtefact(UnknownArtefact<'a>),
        UnknownEdge(&'a str, &'a str),
        DuplicateRequirement(&'a Requirement, &'a Requirement),
        CoveredWithWrongTitle(&'a Requirement, &'a Requirement, &'a str),
        DependWithWrongTitle(&'a Requirement, &'a Requirement, &'a str),
        CombinedTracingsWithIntersectingEdges,
    }

    impl<'a> From<UnknownArtefact<'a>> for TracingError<'a> {
        fn from(e: UnknownArtefact<'a>) -> Self {
            Self::UnknownArtefact(e)
        }
    }

    impl<'a> fmt::Display for TracingError<'a> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                TracingError::UnknownArtefact(ua) => ua.fmt(f),
                TracingError::DuplicateRequirement(first, second) => write!(
                    f,
                    "{}: Duplicate Requirement {} previously defined at {}",
                    second.location, second.id, first.location
                ),
                TracingError::CoveredWithWrongTitle(upper, lower, title) => write!(
                    f,
                    "{}: Requirement {} covered with wrong title {} != {}",
                    lower.location,
                    upper.id,
                    title,
                    upper.title.as_deref().unwrap_or("<no title>")
                ),
                TracingError::DependWithWrongTitle(upper, lower, title) => write!(
                    f,
                    "{}: Requirement {} depended on with wrong title {} != {}",
                    upper.location,
                    lower.id,
                    title,
                    lower.title.as_deref().unwrap_or("<no title>")
                ),
                TracingError::CombinedTracingsWithIntersectingEdges => {
                    write!(f, "Combining Tracings which contain the same Edges")
                }
                TracingError::UnknownEdge(from, to) => {
                    write!(f, "No Edge from {} to {}", from, to)
                }
            }
        }
    }
    impl<'a> error::Error for TracingError<'a> {}
}
use errors::*;

#[derive(Debug)]
pub enum Coverage<'a> {
    Covered,
    CoveredWithTitle(&'a str),
    Depends,
    DependsWithTitle(&'a str),
}

#[derive(Debug, PartialEq)]
pub struct DerivedRequirement<'a> {
    pub req: &'a Requirement,
    node: NodeIdx,
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
    pub req: &'a Requirement,
    edges: Vec<(EdgeIdx, u16)>,
}

#[derive(Debug, Default)]
pub struct Tracing<'a> {
    edges: HashSet<(EdgeIdx, u16)>, // TODO: for Edge Groups not u16
    pub covered: Vec<CoveredRequirement<'a>>,
    pub uncovered: HashMap<&'a str, UncoveredRequirement<'a>>,
    pub derived: HashMap<&'a str, DerivedRequirement<'a>>,
    pub errors: Vec<TracingError<'a>>,
}

impl<'a> Tracing<'a> {
    pub fn add(mut self, mut other: Tracing<'a>) -> Result<Self, TracingError> {
        if !self.edges.is_disjoint(&other.edges) {
            return Err(TracingError::CombinedTracingsWithIntersectingEdges);
        }
        for e in other.edges {
            self.edges.insert(e);
        }

        self.errors.append(&mut other.errors);

        for cov in &self.covered {
            other.derived.remove(cov.lower.id.as_str());
            other.uncovered.remove(cov.upper.id.as_str());
        }
        for cov in other.covered {
            self.derived.remove(cov.lower.id.as_str());
            self.uncovered.remove(cov.upper.id.as_str());
            self.covered.push(cov);
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
                        self.errors
                            .push(TracingError::DuplicateRequirement(uncov.req, old.req));
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

    fn node_idx_by_id<'i>(&self, id: &'i str) -> Result<NodeIdx, UnknownArtefact<'i>> {
        return Ok(*self
            .artefact_id_to_node
            .get(id)
            .ok_or(UnknownArtefact(id))?);
    }

    /// Add [`Artefact`] to the graph
    ///
    /// # Errors
    /// Returns [`DuplicateArtefactId`] if an artefact with the same id was
    /// already registered
    pub fn add_artefact(&mut self, artefact: Artefact<'a>) -> Result<(), ConfigError<'a>> {
        let node_id: NodeIdx = self.nodes.len().into();

        let e = self.artefact_id_to_node.entry(artefact.id);
        match e {
            std::collections::hash_map::Entry::Occupied(_) => {
                return Err(ConfigError::DuplicateArtefact(&artefact.id));
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
    pub fn add_edge_group<'r>(
        &mut self,
        upper: &'r str,
        lower: &[&'r str],
    ) -> Result<(), ConfigError<'r>> {
        let edge_idx: EdgeIdx = self.edges.len().into();
        let mut lower_indexes: Vec<NodeIdx> = vec![];
        let upper_idx: NodeIdx = self.node_idx_by_id(upper)?;
        self.node_mut(upper_idx).edges_up.push(edge_idx);

        for lower_artefact_id in lower {
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

    pub fn get_artefact<'i>(
        &'a self,
        id: &'i str,
    ) -> Result<&'a Artefact<'a>, UnknownArtefact<'i>> {
        let node_idx = self.node_idx_by_id(id)?;
        let node = self.node_ref(node_idx);
        Ok(&node.artefact)
    }

    pub fn get_upper<'i>(
        &'a self,
        id: &'i str,
    ) -> Result<Vec<&'a Artefact<'a>>, UnknownArtefact<'i>> {
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

    pub fn get_lower<'i>(
        &'a self,
        id: &'i str,
    ) -> Result<Vec<Vec<&'a Artefact<'a>>>, UnknownArtefact<'i>> {
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

    fn find_edge_from_to(
        &self,
        from: &'a str,
        to: &'a str,
    ) -> Result<(EdgeIdx, u16), TracingError> {
        let root_node = if let Some(n) = self.artefact_id_to_node.get(from) {
            *n
        } else {
            return Err(TracingError::from(UnknownArtefact(from)));
        };

        for edge in root_node.lower(self) {
            for (group, lower_node) in edge.to(self).iter().enumerate() {
                if lower_node.as_ref(self).artefact.id == to {
                    return Ok((*edge, group as u16));
                }
            }
        }
        return Err(TracingError::UnknownEdge(from, to));
    }

    pub fn trace_edge(&'a self, from: &'a str, to: &'a str) -> Tracing<'a> {
        match self.find_edge_from_to(from, to) {
            Err(_) => {
                let t = Tracing::default();
                return Tracing {
                    errors: vec![TracingError::from(UnknownArtefact(from))],
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
                            errors.push(TracingError::DependWithWrongTitle(ur, lr, title));
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
                        errors.push(TracingError::CoveredWithWrongTitle(ur, lr, title));
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
                    errors.push(TracingError::DuplicateRequirement(old.req, ur));
                }
            }
        }

        Tracing {
            edges,
            covered,
            uncovered,
            derived,
            errors,
        }
    }

    pub fn get_parsing_errors<'r>(&'r self) -> impl Iterator<Item = &'r ParserError> {
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

        g.add_edge_group("REQ", &["DSG", "FORMAT"]).unwrap();
        g.add_edge_group("DSG", &["Code", "FORMAT"]).unwrap();
        g.add_edge_group("FORMAT", &["Code"]).unwrap();
        g.add_edge_group("DSG", &["DTests"]).unwrap();
        g.add_edge_group("REQ", &["RTests"]).unwrap();

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
