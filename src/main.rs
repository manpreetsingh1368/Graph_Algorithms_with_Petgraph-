use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::Dfs;
use petgraph::algo::dijkstra;

fn main() {
    // Create a directed graph
    let mut graph = DiGraph::new();

    // Add nodes to the graph
    let node_a = graph.add_node("A");
    let node_b = graph.add_node("B");
    let node_c = graph.add_node("C");
    let node_d = graph.add_node("D");
    let node_e = graph.add_node("E");

    // Add directed edges to the graph
    graph.add_edge(node_a, node_b, 1);
    graph.add_edge(node_a, node_c, 1);
    graph.add_edge(node_b, node_d, 1);
    graph.add_edge(node_c, node_d, 1);
    graph.add_edge(node_c, node_e, 1);
    graph.add_edge(node_d, node_e, 1);

    // Perform DFS traversal starting from node A
    println!("DFS Traversal:");
    let start_node = node_a;
    dfs_traversal(&graph, start_node);

    // Find the shortest paths from node A to all other nodes using Dijkstra's algorithm
    println!("Shortest Paths from Node A:");
    let start_node_index = node_a;
    let shortest_paths = dijkstra(&graph, start_node_index, None, |e| *e.weight());

    for (node_index, distance) in shortest_paths {
        let node = graph[node_index];
        println!("Shortest path to {:?} is {:?}", node, distance);
    }
}

fn dfs_traversal(graph: &DiGraph<&str, i32>, start_node: NodeIndex) {
    let mut dfs = Dfs::new(graph, start_node);
    while let Some(node) = dfs.next(graph) {
        println!("Visited node: {:?}", graph[node]);
    }
}
