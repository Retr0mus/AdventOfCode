use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::path::Path;
use petgraph::graph::Graph;
use petgraph::unionfind::UnionFind;
use petgraph::visit::EdgeRef;


fn euclidean_norm(vector: &[f64]) -> f64 {
    let sum_of_squares: f64 = vector.iter().map(|&x| x * x).sum();
    sum_of_squares.sqrt()
}

fn main() -> io::Result<()> {
    // Specify the path to your file
    let path = Path::new("data/day8.txt");
    
    // Open the file
    let file = File::open(path)?;

    // Create a buffered reader
    let reader = BufReader::new(file);

    // Define undirected graph
    let mut graph = Graph::<u32, f64>::new();
    let mut nodes : Vec<Vec<f64>> = Vec::new();
    let mut last_connection : (usize, usize) = (0,0);


    // Iterate over lines in the file
    for line in reader.lines() {
        // Handle each line
        match line {
            Ok(content) => {
                let coordinates : Vec<f64> = content
                    .split(',')
                    .filter_map(|s| s.trim().parse::<f64>().ok())
                    .collect();
                nodes.push(coordinates);
            },
            Err(e) => eprintln!("Error reading line: {}", e),
        }
    }

    // Populate the graph
    for i in 0..nodes.len()-1 {
        graph.add_node(i as u32);
        for j in (i+1)..nodes.len() {
            let distance_vector: Vec<f64> = nodes[i].iter()
                .zip(nodes[j].iter())
                .map(|(&a, &b)| a - b)
                .collect();
            let distance_modulo = euclidean_norm(&distance_vector);
            graph.extend_with_edges(&[(i as u32, j as u32, distance_modulo)]);
        }
    }

    // Implementing Kurlskal's algorithm with stop at fixed edges
    let node_count = graph.node_count();
    let mut edges: Vec<(usize, usize, f64)> = graph
    .edge_references()
    .map(|er| (er.source().index(), er.target().index(), *er.weight()))
    .collect();

    edges.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());
    let mut disjoint_sets = UnionFind::new(node_count);

    // Connecting boxes
    for (src, dest, _) in edges {
        if disjoint_sets.find(src) != disjoint_sets.find(dest) {
            disjoint_sets.union(src, dest);
            last_connection = (src, dest);
        }
    }


    // Computing required metric
    let (src, dest) = last_connection;
    let x_product = nodes[src][0] * nodes[dest][0];

    println!("Result: {}", x_product);      // 1474050600

    Ok(())
}