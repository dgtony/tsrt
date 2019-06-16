use crate::sorting::{SparseGraph, TSortErr, TopoSorter};

use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;
use std::iter::FromIterator;

pub struct KahnSorter;

impl TopoSorter for KahnSorter {
    fn sort<T>(graph: &SparseGraph<T>) -> Result<Vec<&T>, TSortErr>
    where
        T: Hash + Eq + Clone,
    {
        sort(graph)
    }
}

/// Topological sorting using Kahn's algorithm
fn sort<T>(graph: &SparseGraph<T>) -> Result<Vec<&T>, TSortErr>
where
    T: Hash + Eq + Clone,
{
    let mut tsorted: Vec<&T> = Vec::new();
    let mut iteration: usize = 0;
    let vertices = graph.vertices();
    // incoming degrees of all vertices in the graph
    let mut in_degrees: HashMap<&T, usize> = vertices
        .iter()
        .map(|&v| {
            let d = graph.incoming(v).map(|n| n.len()).unwrap_or(0);
            (v, d)
        })
        .collect();

    // push vertices w/o incoming edges into the queue
    let mut queue: VecDeque<&T> =
        VecDeque::from_iter(in_degrees.iter().filter(|(_, d)| **d == 0).map(|(&v, _)| v));

    while let Some(vertex) = queue.pop_front() {
        tsorted.push(vertex);
        iteration += 1;

        // remove edges from vertex to its descendants and
        // add nodes without incoming edges to the queue
        graph.outgoing(vertex).map(|neighbours| {
            neighbours.iter().for_each(|v| {
                if let Some(d) = in_degrees.get_mut(&v) {
                    *d -= 1;
                    if *d == 0 {
                        queue.push_back(&v);
                    }
                }
            })
        });
    }

    if iteration == vertices.len() {
        Ok(tsorted)
    } else {
        Err(TSortErr::Cycle)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Construct simple DAG for tests
    ///
    ///    a
    ///    |
    ///    b
    ///   / \
    ///  c   d
    ///     / \
    ///    e   f
    ///
    fn simple_dag() -> SparseGraph<&'static str> {
        let mut g = SparseGraph::new();
        g.add_edge(&"a", &"b");
        g.add_edge(&"b", &"c");
        g.add_edge(&"b", &"d");
        g.add_edge(&"d", &"e");
        g.add_edge(&"d", &"f");
        g
    }

    #[test]
    fn topo_sort() {
        let g = simple_dag();
        let tsorted = sort(&g).unwrap_or_default();

        assert!(vec![
            vec![&"a", &"b", &"c", &"d", &"f", &"e"],
            vec![&"a", &"b", &"d", &"c", &"f", &"e"],
            vec![&"a", &"b", &"c", &"d", &"e", &"f"],
            vec![&"a", &"b", &"d", &"c", &"e", &"f"],
        ]
        .contains(&tsorted))
    }

    #[test]
    fn sort_cycle() {
        let mut g = simple_dag();
        g.add_edge(&"f", &"b");
        let tsorted = sort(&g);

        assert_eq!(tsorted, Err(TSortErr::Cycle))
    }

    #[test]
    fn sort_cycle_full() {
        let mut g = simple_dag();
        g.add_edge(&"f", &"a");
        let tsorted = sort(&g);

        assert_eq!(tsorted, Err(TSortErr::Cycle))
    }
}
