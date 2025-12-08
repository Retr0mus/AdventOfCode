use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::path::Path;
use petgraph::graph::{Graph, NodeIndex};
use petgraph::unionfind::UnionFind;
use petgraph::visit::EdgeRef;


fn euclidean_norm(vector: &[f64]) -> f64 {
    let sum_of_squares: f64 = vector.iter().map(|&x| x * x).sum();
    sum_of_squares.sqrt()
}

fn disjoint_sets_from_unionfind(n: usize, uf: &UnionFind<usize>) -> Vec<Vec<NodeIndex>> {
    let mut groups: std::collections::HashMap<usize, Vec<NodeIndex>> = std::collections::HashMap::new();
    for i in 0..n {
        let rep = uf.find(i);
        groups.entry(rep).or_default().push(NodeIndex::new(i));
    }
    groups.into_values().collect()
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
    let max_junctions : usize = 1000;


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

    // Implementing partial Kurlskal's algorithm with stop at fixed edges
    let node_count = graph.node_count();
    let mut edges: Vec<(usize, usize, f64)> = graph
    .edge_references()
    .map(|er| (er.source().index(), er.target().index(), *er.weight()))
    .collect();

    edges.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

    let mut disjoint_sets = UnionFind::new(node_count);

    // Connect the boxes
    for i in 0..max_junctions {
        let (src, dest, _w) = edges[i];
        //println!("{} {} {}", src, dest, w);
        disjoint_sets.union(src, dest);
    }


    // Computing required metric
    let mut circuits: Vec<Vec<NodeIndex>> = disjoint_sets_from_unionfind(node_count, &disjoint_sets);
    circuits.sort_unstable_by_key(|c| c.len());
    let highest_three_product = circuits
        .into_iter()
        .rev()
        .take(3)
        .map(|set| set.len())
        .fold(1, |x, acc| x * acc);

    println!("Result: {}", highest_three_product);      // 52668

    Ok(())
}