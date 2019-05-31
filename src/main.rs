use std::io::{self, Read};
use std::collections::HashMap;
mod graph;

fn fetch_node<'a>(nodes: &HashMap<usize, graph::Graph>, id: usize) -> graph::Graph {
    if nodes.contains_key(id) {
        return nodes.get(id).unwrap()
    }
    let graph = graph::Graph {
        id: id,
        neighbors: Vec::new(),
    };
    nodes.insert(id, graph);
    graph
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let mut nodes: HashMap<usize, graph::Graph> = HashMap::new();
    let lines = buffer.lines();
    for line in lines {
        let split = line.split(",");
        let ids: Vec<usize> = split.map(|e| e.parse::<usize>().unwrap()).collect();
        println!("{}:{}", ids[0], ids[1]);
    }
    Ok(())
}
