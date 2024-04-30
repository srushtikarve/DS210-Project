mod bfs; //these are the modules that I'll be using-- the breadth first search, and stat functions
use crate::bfs::bfs_dist;
mod stats_fc;
use crate::stats_fc::{mean, max, mode};

use rand::Rng;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::prelude::*;
use std::ptr::read;
use std::{usize, vec};
use rand::seq::SliceRandom;
use std::collections::HashSet;
use std::collections::HashMap;

//I went to OH to work on this project
//this is a directed graph of Gnutella's data (a peer to peer sharing network) from August 2002

fn main() {
    let file = "p2p-Gnutella30.txt"; //these lines read the data from the main gnutella file
    let gnutella_file = read_file(file);
    let num_nodes = 20000;

    let small_gnutella_file = make_rand_sample(gnutella_file, num_nodes); //calling the random function below to pick a sample of 20000 (smaller than the original)

    let unique_gnutella_file = unique_nodes(&small_gnutella_file); //calling the unique function to find unique nodes from the truncated dataset "small"

    let adjacency = adjacency_list(small_gnutella_file, unique_gnutella_file); //calling the adjacency list on the unique nodes with it's length or edges used
    let bfs_distance = bfs_dist(&adjacency); //calling the breadth-first search function
    println!("{:?}", bfs_distance);[]
    let bfs_distance_values: Vec<f64> = bfs_distance.values() //extracting dist values and converting them into a vector of f64 values (Vec<f64>)
        .flatten() //flattening the values of the HashMap, filtering out any None values (if any), and then mapping the values to f64 types
        .filter_map(|&x| Some(x))
        .map(|x| x as f64)
        .collect();

    let average = mean(&bfs_distance_values).unwrap_or(0.0); //calling the stat functions from the other module and then printing them below
    let maximum = max(&bfs_distance).unwrap_or(0);
    let mode_val = mode(&bfs_distance_values).unwrap_or(0.0);

    println!("The average value of node connections is: {:?}", average);
    println!("The maximum number of node connections (showing which ones are most popular) are: {:?}", maximum);
    println!("The most common node connections distance value is: {:?}", mode_val);
}

fn read_file(path: &str) -> Vec<(usize, usize)> { //this function reads the file given above
    let mut output: Vec<(usize,usize)> = Vec::new();
    let file = File::open(path).expect("Could not open file");
    let buf_reader = std::io::BufReader::new(file).lines();
    for(idx, line) in buf_reader.enumerate(){
        let line_str = line.expect("Error reading");
        let v: Vec<&str> = line_str.trim().split(|c: char| c.is_whitespace() || c == '\t').collect();
        let x = v[0].parse::<usize>().unwrap();
        let y = v[1].parse::<usize>().unwrap();
        output.push((x,y));
    }
    return output;
}

fn make_rand_sample(graph: Vec<(usize,usize)>, n: usize) -> Vec<(usize,usize)> { //because the data is very large, I'm picking a random sample of 20000 nodes and edges using this sample function
    let mut rng = rand::thread_rng(); //take 20000 elements in the list
    let n_sample: Vec<(usize,usize)> = graph.choose_multiple(&mut rng, n).cloned().collect();
    return n_sample; //this is storing the random collection of elements
}

fn unique_nodes(edges: &Vec<(usize,usize)>) -> HashSet<usize> { //there might be duplicate nodes, so this function will find the unique ones using a hash set
    let mut unique_nodes_set: HashSet<usize> = HashSet::new();
    for (i,j) in edges.iter() {
        unique_nodes_set.insert(*i);
        unique_nodes_set.insert(*j);
    }
    return unique_nodes_set; //list of the nodes that show up
}

fn adjacency_list(edges: Vec<(usize,usize)>, unique_nodes_set:HashSet<usize>) -> Vec<Vec<usize>> { //making the adjacency list, connecting the nodes as needed with edges
    let num_unique = unique_nodes_set.len(); //getting it's length
    let vec_unique_nodes: Vec<&usize> = unique_nodes_set.iter().collect(); //taking a hash set and making it into a vector with index remembered to iterate with
    let mut graph_list: Vec<Vec<usize>> = vec![vec![]; num_unique]; 
    let mut node_map: HashMap<usize, usize> = HashMap::new(); //pairing a node to it's index here, so adjacency list has no gaps
    for (idx, val) in vec_unique_nodes.iter().enumerate(){
        node_map.insert(**val, idx); //pairing continued
    }
    for (source, destination) in edges {
        graph_list[node_map[&source]].push(node_map[&destination]);
    }
    for (vertex, neighbors) in graph_list.iter().enumerate() {
        println!("Vertex {}: {:?}", vertex, neighbors);
    }
    return graph_list;
}

#[test]
fn test_unique_nodes(){ //tests the test nodes file for the unique nodes which are 0-10 so 11 numbers
    let file = "node_test_file.txt"; //these lines read the data from the node file
    let node_file = read_file(file);
    let num_unique = unique_nodes(&node_file).len();
    assert_eq!(num_unique, 11); //again, my data file has 11 nodes so we should get the number 11 outputted here
}

#[test]
fn test_max() { //this tests whether the max function can actually calculate which node has the most connections
    let bfs_distance: HashMap<usize, Vec<usize>> = [ //connections of nodes to others from the node file hard-coded here, because in the actual data the answer will be based on the randomized sample each time
        (0, vec![2]),
        (1, vec![3, 6, 5]),
        (2, vec![4, 10]),
        (3, vec![9, 5, 1]),
        (4, vec![7, 5]),
        (5, vec![8]),
        (6, vec![9, 1]),
        (7, vec![2, 5, 3, 1]), 
        (8, vec![7, 10]),
        (9, vec![6, 7, 1]),
        (10, vec![3, 6]),
    ]
    .iter()
    .cloned()
    .collect();

    let maximum = max(&bfs_distance).unwrap_or(0); //calculating which node has highest connections with the highest length vector
    assert_eq!(maximum, 7); //here the answer should be seven, since this vector has the most connections
}
