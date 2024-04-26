mod bfs; //these are the modules that I'll be using-- the breadth first search, and stat functions
use crate::bfs::bfs_dist;
//mod stats_fc;
//use crate::stats_fc::{};

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
//this is a directed graph 

fn main() {
    let file = "p2p-Gnutella30.txt"; //these three lines read the data from the Amazon file and then print it
    let amazon_file = read_file(file);

    let num_nodes = 20000;
    let small_amazon_file = make_rand_sample(amazon_file, num_nodes); //calling the random function to pick a sample
    let unique_amazon_file = unique_nodes(&small_amazon_file); //finding unique nodes from the truncated dataset unique
    //println!("{:?}", unique_amazon_file);
    let adjacency = adjacency_list(small_amazon_file, unique_amazon_file); //calling the adjacency list on the unique nodes with it's length used
    let bfs_distance = bfs_dist(&adjacency); //picking random node from the sample that has connection/s //see if you end up needing this
    println!("{:?}", bfs_distance); 
}

fn read_file(path: &str) -> Vec<(usize, usize)> { //this function reads the file
    let mut output: Vec<(usize,usize)> = Vec::new();
    let file = File::open(path).expect("Could not open file");
    let buf_reader = std::io::BufReader::new(file).lines();
    for(idx, line) in buf_reader.enumerate(){
        let line_str = line.expect("Error reading");
        let v: Vec<&str> = line_str.trim().split('\t').collect();
        let x = v[0].parse::<usize>().unwrap();
        let y = v[1].parse::<usize>().unwrap();
        output.push((x,y));
    }
    return output;
}

fn make_rand_sample(graph: Vec<(usize,usize)>, n: usize) -> Vec<(usize,usize)> { //because the data is very large, I will pick a random sample of 50000 nodes and edges using this sample
    let mut rng = rand::thread_rng(); //take 50000 elements in the list
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
fn test_connections() {
    let file = "node_test_file.txt"; //these three lines read the data from the Amazon file and then print it
    let node_file = read_file(file);

        //the test is checking for the 10 nodes in my txt file because that is how many are in it
   
    let exp_output = vec![None, Some(0), Some(), Some(1), Some(), Some(), Some(1), Some(), Some(), Some(), Some()]; //the shortest distance from 1 to each node (starts with zero so it's none first since its 1-10)
    let test_adjacency = adjacency_list(node_file, unique_nodes(&node_file));  // Generate the adjacency list using your function
    let distance = bfs_dist(1, &test_adjacency); //picking random node from the sample that has connection/s //see if you end up needing this
    println!("{:?}", distance);
    assert_eq!(distance, exp_output); // check that the generated adjacency list matches the expected adjacency list
}
//1-3-5-8 so 1-8 should be 3
//test 0 or 1 or whatever degrees of seperation
