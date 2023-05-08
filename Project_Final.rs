use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{VecDeque, HashMap};

// Define vertex type
type Vertex = u64;

// Define component type
type Component = usize;

// Define graph structure
struct Graph {
    n: usize,
    edges: Vec<(Vertex, Vertex)>,
    outedges: Vec<Vec<Vertex>>,
}

impl Graph {
    // Create a new undirected graph
    fn create_undirected(n: usize, edges: &Vec<(Vertex, Vertex)>) -> Graph {
        let mut outedges = vec![Vec::new(); n];

        for &(u, v) in edges {
            outedges[u as usize].push(v);
            outedges[v as usize].push(u);
        }

        Graph { n, edges: edges.clone(), outedges }
    }
}

// Compute connected components using BFS
fn compute_connected_components(graph: &Graph) -> Vec<Option<Component>> {
    let mut component = vec![None; graph.n];
    let mut component_no = 0;

    for v in 0..graph.n {
        if component[v].is_none() {
            mark_component_bfs(v as Vertex, graph, &mut component, component_no);
            component_no += 1;
        }
    }

    component
}

// Mark connected component using BFS
fn mark_component_bfs(vertex: Vertex, graph: &Graph, component: &mut Vec<Option<Component>>, component_no: Component) {
    component[vertex as usize] = Some(component_no);

    let mut queue = VecDeque::new();
    queue.push_back(vertex);

    while let Some(v) = queue.pop_front() {
        for &w in &graph.outedges[v as usize] {
            if component[w as usize].is_none() {
                component[w as usize] = Some(component_no);
                queue.push_back(w);
            }
        }
    }
}

// Compute average distance between vertices in the graph
fn compute_average_distance(graph: &Graph, component: &Vec<Option<Component>>) -> f64 {
    let mut total_distance = 0;
    let mut total_pairs = 0;
}

    // For each connected component
    let mut component_vertices = HashMap::new();
    for (v, &c) in component.iter().enumerate() {
        if let Some(c) = c {
            component_vertices.entry(c).or_insert_with(Vec::new).push(v as Vertex);
        }
    }

    for vertices in component_vertices.values() {
        // Compute distance between all pairs of vertices in the component
        let mut distance = vec![None; vertices.len()];
        for i in 0..vertices.len() {
            let start = vertices[i];
            let mut queue = VecDeque::new();
            queue.push_back(start);
            distance[i] = Some(0);
            while let Some(v) = queue.pop_front() {
                for &w in &graph.outedges[v as usize] {
                    if let Some(j) = vertices.iter().position(|&x| x == w) {
                        if distance[j].is_none() {
                            distance[j] = Some(distance[i].unwrap() + 1);
                            queue.push_back(w);
                        }
                    }
                }
            }
        }

        // Compute average distance for the component
        for d in distance {
            if let Some(d) = d {
                total_distance += d;
                total_pairs += 1;
            }
        }
   }

    total_distance as f64 / total_pairs as f64
}

    // Define the function to compute the average distance between pairs of vertices
fn compute_average_distance(graph: &Graph) -> Option<f64> {
    // Create an empty vector to hold the distances
    let mut distances: Vec<f64> = Vec::new();

    // Loop through all pairs of vertices and compute the distance between them
    for i in 0..graph.n {
        for j in i+1..graph.n {
            let distance = shortest_path_length_bfs(i, j, graph)?;
            distances.push(distance as f64);
        }
    }

    // Compute the average distance and return it
    let sum: f64 = distances.iter().sum();
    let n = distances.len() as f64;
    Some(sum / n)
    }

fn main() {
    // Load the Twitter graph from the CSV file
    let graph = load_graph_from_csv("tdata.csv").unwrap();

    // Compute the connected components using BFS
    let components = compute_connected_components(&graph);

    // Compute the average distance between pairs of vertices in the graph
    let avg_distance_pairs = compute_average_distance_pairs(&graph).unwrap();

    // Compute the average distance between vertices in each connected component
    let avg_distances: Vec<f64> = components.iter().filter_map(|c| c.as_ref()).unique().map(|c| compute_average_distance(&graph, &components, *c)).collect();

    println!("Number of vertices: {}", graph.n);
    println!("Number of edges: {}", graph.edges.len());
    println!("Number of connected components: {}", components.iter().filter(|c| c.is_some()).count());
    println!("Average distance between pairs of vertices: {:.2}", avg_distance_pairs);
    println!("Average distance between vertices in each connected component:");
    for (i, avg_distance) in avg_distances.iter().enumerate() {
        println!("  Component {}: {:.2}", i+1, avg_distance);
    }
}




