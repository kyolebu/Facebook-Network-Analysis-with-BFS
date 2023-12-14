#[derive(Debug)]
// defining structure of Graph
pub struct Graph {
    pub node: usize,
    pub adjacency_list: Vec<Vec<usize>>,
}

impl Graph {
    // create a new graph with the specified number of nodes and edges
    pub fn make_graph(node: usize, edges: Vec<(usize, usize)>) -> Graph {
        // Create a new graph with the specified number of nodes and empty adjacency lists.
        let mut graph = Graph {
            node,
            adjacency_list: vec![vec![]; node],
        };
    
        // Iterate over the provided edges and add them to the graph.
        for (n, e) in edges {
            // Ensure the node and edge indices are within the valid range.
            if n >= node {
                panic!("Node index {} exceeds the number of nodes {}", n, node);
            }
    
            // Push the edge index into the adjacency list of the corresponding node.
            graph.adjacency_list[n].push(e);
        }
        graph
    }
}