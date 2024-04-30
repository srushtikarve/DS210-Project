use std::collections::VecDeque;
use std::collections::HashSet;
use std::collections::HashMap;

pub fn bfs_dist(start: usize, graph: &Vec<Vec<usize>>) -> Vec<isize> { //looks at which nodes have connections to which ones
    let mut distance: Vec<isize> = vec![-1;graph.len()];
    let mut visited: Vec<usize> = vec![];
    let mut queue: VecDeque<usize> = VecDeque::new();
    distance[start] = 0;
    queue.push_back(start); //adding initial vector to queue
    while let Some(v) = queue.pop_front() { //new unprocessed vertex
        visited.push(v);
        for &u in &graph[v] {
            if distance[v] + 1 < distance[u] || distance[u] == -1 {
                distance[u] = distance[v] + 1;
            }
            if !visited.contains(&u) {
                queue.push_back(u);
            }
        }
    }
    return distance; //BFS determines the distances between nodes
}