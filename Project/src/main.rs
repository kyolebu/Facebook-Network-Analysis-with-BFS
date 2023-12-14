mod read;
mod graph;
mod functions;
mod test;

fn main() {

    // initializing variables
    let mut result: Vec<i32> = vec![];
    let n = 4039; // number of nodes in the data set

    // read graph from txt
    let mut read_edges = read::read_file("facebook.txt");
    let mut graph = graph::Graph::make_graph(n, read_edges.clone());

    // degrees of separation for each
    for i in 0..n {
        result.push(functions::bfs(i, &graph));   
    }

    // average distance between a pair of verticies
    let avg: f32 = result.iter().sum::<i32>() as f32 / result.len() as f32;

    // printing result
    println!("List of average distances from each node to its connected nodes:");
    println!("{:?}",result);
    println!("[average distance to others, # of nodes]: {:?}", functions::count_degree(result));
    println!("Average: {}", avg);
}