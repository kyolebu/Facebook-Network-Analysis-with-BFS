
#[cfg(test)]
mod test {

    use crate::graph::Graph;
    use crate::count_degree;    

    #[test]
    fn test_graph() {
        let edges: Vec<(usize,usize)> = vec![(0,2),(1,2),(2,4),(2,6)];
        let graph = Graph::make_graph(3,edges);
        let mut count_edge = vec![];
        
        for (_i, j) in graph.adjacency_list.iter().enumerate(){
            count_edge.push(j.len());
        }
        assert_eq!(count_edge,[1,1,2]);
    }

    #[test]
    fn test_count_degree(){
        let count_sample = vec![1,1,2,2,2,4,8];
        let count_test = count_degree(count_sample);
        assert_eq!(count_test, vec![[1,2],[2,3],[3,0],[4,1],[5,0],[6,0],[7,1]]);
    }
}