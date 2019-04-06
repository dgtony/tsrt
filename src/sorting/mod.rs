use std::hash::Hash;
use std::iter::FromIterator;

pub mod dfs;
pub mod graph;
pub mod kahn;

pub use graph::SparseGraph;

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
}
