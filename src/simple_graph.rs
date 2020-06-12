// For reading file
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::process;
use failure::Error;


pub struct Graph {
    adj:    Vec<Vec<(usize, usize)>>,
    rev_adj:Vec<Vec<(usize, usize)>>,
    no_vertice: usize,
    no_edge: usize,
    no_label: usize,
}

impl Graph  {
    fn read_header(line:String) -> Result<(usize, usize, usize), Error> {
        let linelist: Vec<&str>  = line.split(",").collect();
        let no_nodes = linelist[0].parse::<usize>().unwrap();
        let no_edges  = linelist[1].parse::<usize>().unwrap();
        let no_labels = linelist[2].parse::<usize>().unwrap();
        Ok((no_nodes, no_edges, no_labels))
    }

    fn read_line(line:String) -> Result<(usize, usize, usize), Error> {
        let linelist: Vec<&str> = line.split(" ").collect();
        let subject = linelist[0].parse::<usize>()?;
        let predicate = linelist[1].parse::<usize>()?;
        let object = linelist[2].parse::<usize>()?;
        Ok((subject, predicate, object))
    }

    pub fn build_from_file(filename :String) -> Result<Graph, Error> {
        let graph_file = match File::open(filename) {
            Ok(file) => file,
            Err(e) => {
                println!("Something went wrong: {}", e);
                process::exit(0);
            }
        };

        let filebuf = BufReader::new(&graph_file);
        
        let header = filebuf.lines().next().unwrap()?;
        let (no_nodes, no_edges, no_labels) = Graph::read_header(header)?;
        
        let mut graph = Graph {
            adj: Vec::with_capacity(no_nodes as usize),
            rev_adj: Vec::with_capacity(no_nodes as usize),
            no_vertice: no_nodes, 
            no_edge: no_edges,
            no_label: no_labels,
        };
        graph.adj.resize(no_nodes, Vec::new());
        graph.rev_adj.resize(no_nodes, Vec::new());

        let filebuf2 = BufReader::new(&graph_file);
        for line in filebuf2.lines().skip(1) {
            let l = line?; 
            let (from, edge_label, to) = Graph::read_line(l)?;
            graph.add_edge(from, edge_label, to).unwrap();
        }
        Ok(graph)
    }

    pub fn get_no_vertices(&self) -> usize {
        self.no_vertice
    }

    pub fn get_no_edges(&self) -> usize {
        self.no_edge
    }

    pub fn get_no_labels(&self) -> usize {
        self.no_label      
    }

    pub fn add_edge(&mut self, from:usize , edge_label:usize, to: usize) -> Result<(), Error> {
        if from >= self.no_vertice || to >= self.no_vertice || edge_label >= self.no_label {
            bail!("Edge data out of bounds: ({}, {}, {});", from, edge_label, to);
        }
        self.adj[from].push((edge_label, to));
        self.rev_adj[to].push((edge_label, from));

        Ok(())
    }

    pub fn set_no_vertices(&mut self, no_vertices: usize) -> () {
        self.no_vertice = no_vertices;
    }

    pub fn set_no_labels(&mut self, no_labels: usize) -> () {
        self.no_label = no_labels;
    }
}
