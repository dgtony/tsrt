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
pub struct Relation<'a, T>
where
    T: Hash + Eq,
{
    pub from: &'a T,
    pub to: &'a T,
}

pub trait TopoSorter {
    fn sort<'a, 'b: 'a, T: Hash + Eq>(
        graph: &'b SparseGraph<'a, T>,
    ) -> Result<Vec<&'a T>, TSortErr>;
}

pub fn mk_sort<'a, T, S: TopoSorter>(
    graph: &'a SparseGraph<'a, T>,
    _sorter: &S,
) -> Result<Vec<&'a T>, TSortErr>
where
    T: Hash + Eq,
{
    S::sort(graph)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn graph_relations() {
        let rels: HashSet<Relation<&str>> = HashSet::from_iter(vec![
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

        let rs = HashSet::from_iter(rels);
        let rs2 = rs.clone();

        // relation set -> graph -> relation set
        let relation_set: HashSet<Relation<&str>> = SparseGraph::from_iter(rs).into();

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
