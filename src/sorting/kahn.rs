use crate::sorting::graph::SparseGraph;
use crate::sorting::TSortErr;

use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;

use std::fmt::Debug;
use std::iter::FromIterator;

// todo use graph trait instead!!!
// most likely it is a sparse graph structure, hence we
// can use adjacency list representation

/// Topological sort using Kahn's algorithm
///
///
pub fn sort<'a, T>(graph: &'a SparseGraph<'a, T>) -> Vec<&'a T>
where
    T: Hash + Eq,
{
    let mut tsorted: Vec<&'a T> = Vec::new();
    let mut iteration: usize = 0;
    let vertices = graph.vertices();
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
            neighbours.iter().for_each(|&v| {
                if let Some(d) = in_degrees.get_mut(v) {
                    *d -= 1;
                    if *d == 0 {
                        queue.push_back(v);
                    }
                }
            })
        });
    }

    if iteration > vertices.len() {
        // todo return Result
        println!("shit happened => cycle detected");
    }

    return tsorted;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn simple_dag() -> SparseGraph<'static, &'static str> {
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
        let tsorted = sort(&g);


        assert!(vec![
            vec![&"a", &"b", &"c", &"d", &"f", &"e"],
            vec![&"a", &"b", &"d", &"c", &"f", &"e"],
            vec![&"a", &"b", &"c", &"d", &"e", &"f"],
            vec![&"a", &"b", &"d", &"c", &"e", &"f"],
        ].contains(&tsorted))
    }
}
