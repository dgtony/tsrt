extern crate tsrt;

use tsrt::{mk_sort, KahnSorter};
use tsrt::{Relation, SparseGraph, TSortErr};

use std::env;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() < 1 {
        print_usage();
        exit(0)
    }

    let rel = match parse_relations(args) {
        Ok(relations) => relations,
        Err(e) => {
            eprintln!("parsing relations: {:?}", e);
            return;
        }
    };

    let rel_graph = SparseGraph::from(rel);
    match mk_sort(&rel_graph, &KahnSorter) {
        Ok(order) => {
            let formatted: Vec<String> = order.into_iter().map(|s| s.to_owned()).collect();
            println!("Topological order found:\n{}", formatted.join(" -> "))
        }
        Err(TSortErr::NoOrder) => {
            eprintln!("no topological sorting order could be found for given graph")
        }
        Err(TSortErr::Cycle) => eprintln!("no sorting possible: cycles detected in graph"),
    }
}

fn print_usage() {
    println!("Utility tsrt makes topological sort of relation graph.");
    println!("Provide it with a set of space-delimited relations, represented as a");
    println!("comma-separated pairs 'X,Y[,Z...]', where vertex X precedes Y in the DAG.\n");
    println!("Output will be a topological ordering of all the vertices.");
}

#[derive(Debug)]
enum ParseErr {
    BadRelation(String),
}

fn parse_relations(rels: Vec<String>) -> Result<Vec<Relation<String>>, ParseErr> {
    let mut result = Vec::with_capacity(rels.len());
    for rel in rels.into_iter() {
        let relation: Vec<_> = rel.split(",").collect();
        if relation.len() < 2 {
            return Err(ParseErr::BadRelation(relation.as_slice().join(",")));
        }

        let from = relation[0];
        relation.iter().skip(1).for_each(|to| {
            result.push(Relation {
                from: from.to_string(),
                to: to.to_string(),
            })
        });
    }

    Ok(result)
}
