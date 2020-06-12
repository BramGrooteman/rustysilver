#[macro_use]
extern crate failure;

#[macro_use]
extern crate lazy_static;

// For getting the arguments
use failure::Error;
use std::env;

mod rpq_tree;
mod simple_graph;

use std::fs::File;
use std::io::BufReader;
use std::io::Read;

fn main() -> Result<(), Error> {
    let mut args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: \n\nrustysilver [query_file].csv [graph_file].nt \n\n");
        return Err(failure::err_msg("Incorrect invocation"));
    }
    let query_filename: String = args.remove(1);
    let graph_filename: String = args.remove(1);
    println!("Reading {:?} and {:?}", query_filename, graph_filename);

    // Read in graph
    let mut graph = simple_graph::Graph::build_from_file(graph_filename)?;

    // Read in queries
    let query_file = match File::open(query_filename) {
        Ok(file) => file,
        Err(e) => {
            println!("Something went wrong: {}", e);
            return Err(format_err!("Could not read in the file"));
        }
    };
    let mut queries_raw = String::new();
    let mut filebuf = BufReader::new(&query_file);
    filebuf.read_to_string(&mut queries_raw).unwrap();

    let queries = rpq_tree::Query::parse_queries(&queries_raw);
    for q in queries.unwrap() {
        q.print();
        println!("");
    }

    Ok(())
}
