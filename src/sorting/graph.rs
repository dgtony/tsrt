use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;

/// Representation of sparse graphs (|E| << \V\^2)
#[derive(Debug)]
pub struct SparseGraph<'a, T: Hash + Eq> {
    sources: HashMap<&'a T, HashSet<&'a T>>, // src_v -> [dst_v]
    destinations: HashMap<&'a T, HashSet<&'a T>>, // dst_v -> [src_v]
    num_edges: usize,
}

impl<'a, T> SparseGraph<'a, T>
where
    T: Hash + Eq,
{
    pub fn new() -> Self {
        SparseGraph {
            destinations: HashMap::new(),
            sources: HashMap::new(),
            num_edges: 0,
        }
    }

    // Insert directed edge into the graph.
    // Returns true if inserted edge was new.
    //
    // Note that function add_vertex() doesn't make sense in a given representation
    pub fn add_edge(&mut self, from: &'a T, to: &'a T) -> bool {
        let src_inserted = SparseGraph::insert(&mut self.sources, from, to);
        let dst_inserted = SparseGraph::insert(&mut self.destinations, to, from);
        let new_edge = src_inserted || dst_inserted;
        if new_edge {
            self.num_edges += 1;
        }

        new_edge
    }

    pub fn remove_vertex(&mut self, vertex: &T) -> bool {
        // remove from destination set
        if let Some(incoming_vertices) = self.destinations.get(vertex) {
            for x in incoming_vertices.iter() {
                if let Some(edges) = self.sources.remove(*x) {
                    self.num_edges -= edges.len();
                }
            }
        }

        // remove from source set
        if let Some(outgoing_vertices) = self.sources.get(vertex) {
            for x in outgoing_vertices.iter() {
                if let Some(edges) = self.destinations.remove(*x) {
                    self.num_edges -= edges.len();
                }
            }
        }

        let src_removed = self.sources.remove(vertex).is_some();
        let dst_removed = self.destinations.remove(vertex).is_some();
        src_removed || dst_removed
    }

    pub fn remove_edge(&mut self, from: &T, to: &T) -> bool {
        let src_removed = match self.sources.get_mut(from) {
            Some(dsts) => dsts.remove(to),
            None => false,
        };

        let dst_removed = match self.destinations.get_mut(to) {
            Some(srcs) => srcs.remove(from),
            None => false,
        };

        let removed = src_removed || dst_removed;
        if removed {
            self.num_edges -= 1
        }

        removed
    }

    pub fn is_empty(&self) -> bool {
        self.sources.is_empty() && self.destinations.is_empty()
    }

    pub fn incoming(&self, vertex: &T) -> Option<&HashSet<&T>> {
        self.destinations
            .get(vertex)
            .map(|s| if s.is_empty() { None } else { Some(s) })
            .unwrap_or_default()
    }

    pub fn outgoing(&self, vertex: &T) -> Option<&HashSet<&T>> {
        self.sources
            .get(vertex)
            .map(|s| if s.is_empty() { None } else { Some(s) })
            .unwrap_or_default()
    }

    pub fn contains_cycles(&self, start: &T) -> bool {
        unimplemented!()
    }

    /// Depth-first search from given vertex.
    ///
    ///
    pub fn dfs(&'a self, start: &'a T) -> Option<Vec<&'a T>> {
        // todo change signature to Result<Vec, ErrCycle>
        let mut result = Vec::new();
        let mut queue = VecDeque::new();
        let mut iterations = 0;

        queue.push_front(start);
        while let Some(next) = queue.pop_front() {
            if iterations > self.num_edges {
                return None;
            }

            result.push(next);
            iterations += 1;

            if let Some(children) = self.outgoing(next) {
                for &child in children.iter() {
                    queue.push_front(child);
                }
            }
        }

        Some(result)
    }

    // todo change signature
    // return Result with custom error (failure?)
    // - on cycle detected
    // - for toposort: no order
    pub fn bfs(&'a self, start: &'a T) -> Option<Vec<&'a T>> {
        let mut result = Vec::new();
        let mut queue = VecDeque::new();
        let mut iterations = 0;

        queue.push_back(start);
        while let Some(next) = queue.pop_front() {
            if iterations > self.num_edges {
                return None;
            }

            result.push(next);
            iterations += 1;

            if let Some(children) = self.outgoing(next) {
                for &child in children.iter() {
                    queue.push_back(child);
                }
            }
        }

        Some(result)
    }

    /// Return set of all vertices in graph
    pub fn vertices(&self) -> HashSet<&T> {
        self.sources.iter().chain(&self.destinations).fold(
            HashSet::new(),
            |mut acc, (&src, dsts)| {
                acc.insert(src);
                acc.extend(dsts.iter());
                acc
            },
        )
    }

    // internal helper
    fn insert<'t: 'a>(map: &mut HashMap<&'a T, HashSet<&'a T>>, a: &'t T, b: &'t T) -> bool {
        match map.get_mut(&a) {
            Some(sources) => sources.insert(b),
            None => {
                let mut sources = HashSet::new();
                sources.insert(b);
                map.insert(a, sources).is_none()
            }
        }
    }
}

// todo transformations from various graphs into set of relations?

#[cfg(test)]
mod tests {
    use super::*;

    const a: &'static str = "a";
    const b: &'static str = "b";
    const c: &'static str = "c";
    const d: &'static str = "d";

    #[test]
    fn create_graph() {
        let mut g: SparseGraph<&str> = SparseGraph::new();

        // empty
        assert!(g.is_empty());

        // single edge
        assert!(g.add_edge(&a, &b));
        assert!(!g.is_empty());
    }

    #[test]
    fn add() {
        let g = simple_dag();

        assert!(!g.is_empty());
        assert_eq!(g.incoming(&a), None);

        // edges incoming to the vertex
        let b_in = g.incoming(&b).unwrap();
        assert!(b_in.contains(&a));

        // edges outgoing from the vertex
        let b_out = g.outgoing(&b).unwrap();
        assert!(b_out.contains(&c) && b_out.contains(&d));
    }

    #[test]
    fn rm_edges() {
        let mut g = simple_dag();

        // remove existing edge
        assert!(g.remove_edge(&a, &b));

        // remove non-ex edge
        assert!(!g.remove_edge(&c, &a));

        assert!(g.incoming(&b).is_none());
        assert!(g.outgoing(&b).is_some())
    }

    #[test]
    fn rm_vertex() {
        let mut g = simple_dag();
        g.add_edge(&d, &"e");

        // remove existing vertex
        assert!(g.remove_vertex(&b));
        assert!(g.incoming(&b).is_none());
        assert!(g.outgoing(&b).is_none());

        // edge should be removed automatically
        assert!(!g.remove_edge(&a, &b));

        // remains only single edge: d->e
        assert!(g.outgoing(&d).is_some());
    }

    #[test]
    fn bfs() {
        let mut g = simple_dag();
        g.add_edge(&a, &"e");

        let result = g.bfs(&a).unwrap();
        assert_eq!(result[0], &a);
        assert!((result[1] == &b && result[2] == &"e") || (result[2] == &b && result[1] == &"e"));
        assert!((result[3] == &c && result[4] == &d) || (result[4] == &c && result[3] == &d));
    }

    #[test]
    fn dfs() {
        let mut g = simple_dag();
        g.add_edge(&a, &"e");

        let result = g.dfs(&a).unwrap();

        assert_eq!(result[0], &a);
        assert!(result[1] == &b || result[1] == &"e");
        assert!(result[2] == &c || result[2] == &d || result[2] == &b);
        assert!(result[3] == &c || result[3] == &d);
        assert!(result[4] == &"e" || result[4] == &c || result[4] == &d);
    }

    #[test]
    fn search_cycle() {
        let mut g = simple_dag();
        g.add_edge(&d, &a);

        assert!(g.bfs(&a).is_none());
        assert!(g.dfs(&a).is_none());
    }

    #[test]
    fn cycles() {
        let mut g = simple_dag();
        g.add_edge(&d, &a);

        // todo
        //assert!(g.contains_cycles(&a));
    }

    #[test]
    fn vertices() {
        let g = simple_dag();

        assert_eq!(g.vertices(), vec![a, b, c, d].iter().collect());
    }

    fn simple_dag() -> SparseGraph<'static, &'static str> {
        let mut g = SparseGraph::new();

        g.add_edge(&a, &b);
        g.add_edge(&b, &c);
        g.add_edge(&b, &d);

        g
    }
}
