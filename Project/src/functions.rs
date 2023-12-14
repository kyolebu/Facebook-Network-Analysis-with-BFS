use std::collections::VecDeque;
use crate::graph::Graph;

// breadth first search algorithm
pub fn bfs(n_start: usize, graph: &Graph) -> i32 {

    // intialize variables to store values
    let mut distance_cleaned: Vec<u32> = vec![];
    let mut count: u32 = 0;
    let mut avg_distance: i32 = 0;

    // vector to store distances from starting node
    let mut distance: Vec<Option<u32>> = vec![None;graph.node];
    distance[n_start] = Some(0); 

    // queue for BFS
    let mut queue: VecDeque<usize> = VecDeque::new();
    queue.push_back(n_start);
    
    // loop to perform BFS
    while let Some(v) = queue.pop_front() {
        let neighbors = graph.adjacency_list[v].iter().any(|&u| distance[u].is_none());
    
        if neighbors {
            for &u in &graph.adjacency_list[v] {
                if distance[u].is_none() {
                    // increment the distance of the unvisited neighbor
                    let new_distance = distance[v].map(|d| d + 1);
                    distance[u] = new_distance;
    
                    // add the unvisited neighbor to the queue
                    queue.push_back(u);
                }
            }
        }
    }
    
    // calculate average distance from BFS algorithm
    let mut distance_cleaned: Vec<u32> = Vec::new();
    let mut count: u32 = 0;

    for &d in distance.iter() {
        if let Some(val) = d {
            distance_cleaned.push(val);
        }
    }

    for &val in distance_cleaned.iter() {
        count += val;
    }
    
    let avg_distance = (count / distance_cleaned.len() as u32) as i32;

    avg_distance
}

// count degrees in the format of [average distance to others, # of nodes with that distance]
pub fn count_degree(result: Vec<i32>) -> Vec<[usize; 2]> {

    // initialize counters for degrees of 1,2,3,4,5,6,7
    let mut count = vec![
        [1,0],
        [2,0],
        [3,0],
        [4,0],
        [5,0],
        [6,0],
        [7,0]]; // 7 degrees of separation or more

    // loop through vector of average degrees 
    for &degree in &result {
        if degree == 0 {
            continue;
        } else {
            let index = {
                if degree <= 6 { 
                    degree as usize - 1
                } else { // 7 degrees of separation or more
                    6
                }
            };
            // add to specified counter based on average degree
            count[index][1] += 1;
        }
    }
    count
}