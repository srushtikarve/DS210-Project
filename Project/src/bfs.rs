use std::collections::VecDeque;
use std::collections::HashSet;
use std::collections::HashMap;

pub fn bfs_dist(graph: &Vec<Vec<usize>>) -> HashMap<usize, Vec<usize>> { //looks at which nodes have connections to which ones
    let mut distance_map: HashMap<usize, Vec<usize>> = HashMap::new(); //store the distances for each node degree in this
    for i in 0..graph.len() { //calling breadth first search
        let mut distance: Vec<usize> = vec![0;graph.len()];
        let mut queue: VecDeque<usize> = VecDeque::new();
        distance[i] = 0;
        queue.push_back(i); //adding initial vector to queue
        while let Some(v) = queue.pop_front() { // new unprocessed vertex
            for &u in &graph[v] {
                if distance[u] == 0 { // consider all unprocessed neighbors of v
                    distance[u] = distance[v] + 1;
                    queue.push_back(u);
                }
            }
        }
        let mut distance1 = Vec::new();
        for dist in distance.iter() {
            if *dist != 0 {
                distance1.push(*dist)
                }
            }
        distance_map.insert(i, distance1);
    }
    return distance_map;
}