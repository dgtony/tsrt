extern crate tsrt;

use tsrt::{Relation,SparseGraph, TSortErr};
use tsrt::{mk_sort, TopoSorter, KahnSorter};

use std::env;
use std::process::exit;


fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() < 1 {
        print_usage();
        exit(0)
    }

//    dbg!(&args);

    let arg_ref: Vec<&str> = args.iter().map(|r| r.as_str()).collect();
//    println!("parsed: {:#?}", parse_relations(arg_ref));

    // todo uncomment
//    let rel = match parse_relations(arg_ref) {
//        Ok(relations) => relations,
//        Err(e) => {
//            eprintln!("parsing relations: {:?}", e);
//            return;
//        }
//    };
//
//    let rel_graph = SparseGraph::from(rel);
//    let sorted = mk_sort(&rel_graph, &KahnSorter);
//    println!("relation graph: {:?}\ntopologically sorted: {:?}", rel_graph, sorted)


}

fn print_usage() {
    println!("Utility tsrt makes topological sort of relation graph.");
    println!("Provide it with a set of space-delimited relations, represented");
    println!("as comma-separated pairs 'X,Y', where vertex X precedes Y in the DAG.\n");
    println!("Output will be a topological ordering of all the vertices.");
}

#[derive(Debug)]
enum ParseErr<'a> {
    BadRelation(&'a str)
}

//fn parse_relations(rels: Vec<&str>) -> Result<Vec<Relation<str>>, ParseErr> {
//    let mut result = Vec::with_capacity(rels.len());
//    for &rel in rels.iter() {
//        let relation: Vec<_> = rel.split( ",").collect();
//        if relation.len() != 2 {
//            return Err(ParseErr::BadRelation(rel))
//        }
//
//        result.push(Relation{from: relation[0], to:relation[1]})
//    }
//
//    Ok(result)
//}

