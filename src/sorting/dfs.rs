use crate::sorting::{Relation, TSortErr};
use std::collections::hash_set::Iter;
use std::collections::HashSet;
use std::hash::Hash;
use std::iter::FromIterator;

/// Topological sort using simple DFS algorithm
///
/// Time complexity: O(|V|+|E|).
//pub fn sort<T, R>(relations: R)
//    where T: Hash + Eq,
//          R: Sized + Iterator<Item=T>
//{
//
//    //HashSet::new()
//
//}

//pub fn sort<T, R, S>(relations: R) -> S
//pub fn sort<'a, T: 'a, R>(relations: R) -> impl Iterator<Item=&'a T>
//pub fn sort<'a, T, R>(relations: R) -> impl IntoIterator<Item=&'a T>
//pub fn sort<T, R>(relations: R) -> impl IntoIterator<Item=T>
//pub fn sort<T, R>(relations: R) -> Vec<T>
//pub fn sort<'a, T>(relations: impl Iterator<Item=&'a Relation<T>>) -> Vec<&'a T>
//pub fn sort<'a, T>(relations: impl Iterator<Item=Relation<T>>) -> Vec<T>
//where T: Hash + Eq + 'a,
//R: Iterator<Item=Relation<T>>,
//S: Iterator<Item=T>

pub fn sort<'a, T, R>(relations: R) -> Vec<&'a T>
where
    T: Hash + Eq,
    R: Iterator<Item = &'a Relation<T>>,
{
    let mut visited = HashSet::new();

    relations.for_each(|r| {
        visited.insert(&r.from);
        visited.insert(&r.to);
    });

    // todo

    Vec::from_iter(visited)
}
