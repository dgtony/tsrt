use crate::sorting::{Relation, SparseGraph, TSortErr, TopoSorter};

use std::collections::hash_set::Iter;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::iter::FromIterator;

pub struct DFSSorter;

impl TopoSorter for DFSSorter {
    fn sort<T>(graph: &SparseGraph<T>) -> Result<Vec<&T>, TSortErr>
    where
        T: Hash + Eq + Clone,
    {
        sort(graph)
    }
}

/// Topological sort using simple recursive DFS algorithm
///
/// Time complexity: O(|V|+|E|).
///
/// Caution: not recommended for large graphs!
fn sort<T>(graph: &SparseGraph<T>) -> Result<Vec<&T>, TSortErr>
where
    T: Hash + Eq + Clone,
{
    let mut visited: HashMap<&T, bool> = HashMap::new();
    let mut order: Vec<&T> = Vec::new();

    graph.vertices().iter().for_each(|n| {
        if !*visited.get(n).unwrap_or(&false) {
            recursive_dfs(*n, graph, &mut visited, &mut order);
        }
    });

    order.reverse();
    Ok(order)
}

fn recursive_dfs<'a, T>(
    vertex: &'a T,
    graph: &'a SparseGraph<T>,
    visited: &mut HashMap<&'a T, bool>,
    order: &mut Vec<&'a T>,
) where
    T: Hash + Eq + Clone,
{
    visited.insert(vertex, true);

    if let Some(neighbours) = graph.outgoing(vertex) {
        neighbours.iter().for_each(|n| {
            if !*visited.get(n).unwrap_or(&false) {
                recursive_dfs(n, graph, visited, order);
            }
        });
    }

    order.push(vertex);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn topo_sort() {
        let g = simple_dag();
        let tsorted = sort(&g).unwrap_or_default();

        assert!(
            vec![vec![&"a", &"b", &"c", &"d",], vec![&"a", &"b", &"d", &"c"],].contains(&tsorted)
        )
    }

    /// Construct simple DAG for tests
    ///
    ///    a
    ///    |
    ///    b
    ///   / \
    ///  c   d
    ///
    fn simple_dag() -> SparseGraph<&'static str> {
        let mut g = SparseGraph::new();
        g.add_edge("a", "b");
        g.add_edge("b", "c");
        g.add_edge("b", "d");
        g
    }
}
