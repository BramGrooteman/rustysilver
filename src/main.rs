#[macro_use]
extern crate failure;

// For getting the arguments
use std::env;
use failure::Error;

mod simple_graph;

fn main() -> Result<(), Error> {
    let mut args:Vec<String> = env::args().collect();
    let query_filename:String = args.remove(1);
    let graph_filename:String = args.remove(1); 
    println!("Reading {:?} and {:?}", query_filename, graph_filename);

    // Read in graph
    let mut graph = simple_graph::Graph::build_from_file(graph_filename)?;
    Ok(())
}
