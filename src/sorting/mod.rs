use std::hash::Hash;
use std::iter::FromIterator;

mod dfs;
mod graph;
mod kahn;

pub use graph::SparseGraph;

pub use dfs::DFSSorter;
pub use kahn::KahnSorter;

#[derive(Debug, PartialEq)]
pub enum TSortErr {
    NoOrder,
    Cycle,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct Relation<T>
where
    T: Hash + Eq + Clone,
{
    pub from: T,
    pub to: T,
}

pub trait TopoSorter {
    fn sort<T: Hash + Eq + Clone>(graph: &SparseGraph<T>) -> Result<Vec<&T>, TSortErr>;
}

pub fn mk_sort<'a, T, S: TopoSorter>(
    graph: &'a SparseGraph<T>,
    _sorter: &S,
) -> Result<Vec<&'a T>, TSortErr>
where
    T: Hash + Eq + Clone,
{
    S::sort(graph)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use std::iter::FromIterator;

    #[test]
    fn graph_relations() {
        let rels: HashSet<Relation<String>> = HashSet::from_iter(vec![
            Relation {
                from: "a".to_string(),
                to: "b".to_string(),
            },
            Relation {
                from: "b".to_string(),
                to: "c".to_string(),
            },
            Relation {
                from: "b".to_string(),
                to: "d".to_string(),
            },
        ]);

        let relation_set: HashSet<Relation<String>> = SparseGraph::from_iter(rels.into()).into();

        // expecting to be isomorphic
        assert_eq!(relation_set, rs2);
    }

    #[test]
    fn generic_sort() {
        let g = SparseGraph::from(vec![
            Relation {
                from: &"a",
                to: &"b",
            },
            Relation {
                from: &"b",
                to: &"c",
            },
            Relation {
                from: &"b",
                to: &"d",
            },
        ]);

        let sorted_variants = vec![vec![&"a", &"b", &"c", &"d"], vec![&"a", &"b", &"d", &"c"]];

        let ts_kahn = mk_sort(&g, &KahnSorter).unwrap_or_default();
        let ts_dfs = mk_sort(&g, &DFSSorter).unwrap_or_default();

        assert!(sorted_variants.contains(&ts_kahn));
        assert!(sorted_variants.contains(&ts_dfs));
    }
}
