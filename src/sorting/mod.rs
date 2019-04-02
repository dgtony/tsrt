//use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::sync::atomic::Ordering::Relaxed;

pub mod graph;
//pub mod dfs;
pub mod kahn;

pub enum TSortErr {
    NoOrder,
    Cycle,
}

pub struct Relation<'a, T>
where
    T: Hash + Eq,
{
    pub from: &'a T,
    pub to: &'a T,
}

/* Relation graph representation */

//todo move to separate module

/*
pub struct RelationGraph<'a, T> {
    adj_list_out: HashMap<&'a T, HashSet<&'a T>>, // src_v -> [dst_v]
    adj_list_in: HashMap<&'a T, HashSet<&'a T>>, // dst_v -> [src_v]
}

impl<'a, T> RelationGraph<'a, T>
where
    T: Hash + Eq,
{
    pub fn new() -> Self {
        RelationGraph {
            adj_list_in: HashMap::new(),
            adj_list_out: HashMap::new(),
        }
    }

    // Add relation to the graph
    // Return true is new unique relation was added to the graph
    pub fn add(&mut self, relation: Relation<'a, T>) -> bool {
//        match self.adj_list_out.get_mut(&relation.from) {
//            Some(destinations) => {
//                let dst_ins_res = destinations.insert(relation.to);
//                dst_ins_res
//            },
//            None => {
//                let mut destinations = HashSet::new();
//                destinations.insert(relation.to);
//                self.adj_list_out.insert(relation.from, destinations).is_some() //always true
//            }
//        }

        let rel2 = Relation {
           from: relation.from,
            to: relation.to,
        };

        self.insert_in(relation) || self.insert_out(rel2)
    }

    pub fn del(&mut self, relation: Relation<'a, T>) -> bool {
        match self.adj_list_out.get_mut(relation.from) {
            Some(destinations) => destinations.remove(relation.to),
            None => false,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.adj_list_out.is_empty()
    }

    pub fn incoming(&self, vertex: &T) -> Option<Vec<&T>> {
        unimplemented!()
    }

    pub fn outgoing(&self, vertex: &T) -> Option<Vec<&T>> {
        self.adj_list_out
            .get(&vertex)
            .and_then(|dsts| Some(dsts.iter().collect()))
    }


    // internal
    fn insert_in(&mut self, relation: Relation<'a, T>) -> bool {
        RelationGraph::insert(&mut self.adj_list_in, relation.to, relation.from)
    }

    fn insert_out(&mut self, relation: Relation<'a, T>) -> bool {
        RelationGraph::insert(&mut self.adj_list_out, relation.from, relation.to)
    }

    fn insert(map: &mut HashMap<&T, HashSet<&T>>, a: &T, b: &T) -> bool {
        match map.get_mut(&a) {
            Some(sources) => sources.insert(b),
            None => {
                let mut sources = HashSet::new();
                sources.insert(b);
                map.insert(a, sources).is_some()
            }
        }
    }
}

// todo transformations from various graphs into set of relations?
*/
