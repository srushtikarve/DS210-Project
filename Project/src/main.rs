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
//this is a directed graph 

fn main() {
    let file = "p2p-Gnutella30.txt"; //these three lines read the data from the gnutella file and then print it
    let gnutella_file = read_file(file);

    let num_nodes = 20000;
    let small_gnutella_file = make_rand_sample(gnutella_file, num_nodes); //calling the random function to pick a sample
    let unique_gnutella_file = unique_nodes(&small_gnutella_file); //finding unique nodes from the truncated dataset unique
    //println!("{:?}", unique_gnutella_file);
    let adjacency = adjacency_list(small_gnutella_file, unique_gnutella_file); //calling the adjacency list on the unique nodes with it's length used
    let bfs_distance = bfs_dist(&adjacency); //picking random node from the sample that has connection/s //see if you end up needing this
    println!("{:?}", bfs_distance); 

    //let bfs_distance = bfs_dist(&adjacency);
    //println!("{:?}", bfs_distance);

    let bfs_distance_values: Vec<f64> = bfs_distance.values()
        .flatten()
        .filter_map(|&x| Some(x))
        .map(|x| x as f64)
        .collect();

    let average = mean(&bfs_distance_values).unwrap_or(0.0);
    let maximum = max(&bfs_distance_values).unwrap_or(0.0);
    let mode_val = mode(&bfs_distance_values).unwrap_or(0.0);

    println!("The average value of node connections is: {:?}", average);
    println!("The maximum number of node connections (showing which ones are most popular) are: {:?}", maximum);
    println!("The most common node connections distance value is: {:?}", mode_val);
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
fn test_mean(){
    let file = "node_test_file.txt"; //these lines read the data from the node file and then print it
    let node_file = read_file(file);
    let test_mean_val = 5.913; //mean is sum of all numbers/count of numbers in test data

    let mean = mean(node_file);
    assert_eq!(mean, test_mean_val); //checking that the function's output matches known value
}

#[test]
fn test_max(){
    let file = "node_test_file.txt"; //these lines read the data from the node file and then print it
    let node_file = read_file(file);
    let test_max_val = 10; //highest value in dataset

    let maximum = max(node_file); //checking that the function's output matches known value
    assert_eq!(maximum, test_max_val);
}

#[test]
fn test_mode(){
    let file = "node_test_file.txt"; //these lines read the data from the node file and then print it
    let node_file = read_file(file);
    let test_mode_val = 1; //is the mode because that has a frequency of 5 (most out of the rest)

    let mode = mode(node_file); //checking that the function's output matches known value
    assert_eq!(mode, test_mode_val);
}

#[test]
fn test_connections(){
    let file = "node_test_file.txt"; //these lines read the data from the node file and then print it
    let node_file = read_file(file);

    //the test is checking for the 10 nodes in my txt file because that is how many are in it
    //let exp_output_for_1 = vec![None, Some(0), Some(4), Some(1), Some(5), Some(2), Some(1), Some(3), Some(3), Some(2), Some(4)]; //the shortest distance from 1 to each node (starts with zero so it's none first since its 1-10)
    let test_adjacency = adjacency_list(node_file, unique_nodes(&node_file));  //Generate the adjacency list using your function
    //println!("{:?}", distance.get(1));
    //assert_eq!(distance, exp_output_for_1); //check that the generated adjacency list matches the expected adjacency list

    let distance_map = bfs_dist(&test_adjacency); //picking random node from the sample that has connection/s //see if you end up needing this

    assert_eq!(distance_map[&1][0], None); // Distance from 1 to 0 (doesn't exist) //checking distances from 1 to each
    assert_eq!(distance_map[&1][1], Some(0)); //dist from 1 to 1 (self) is 0
    assert_eq!(distance_map[&1][8], Some(3)); //dist from 1 to 8, example 1-3-5-8 degrees of seperation
}
//1-3-5-8 so 1-8 should be 3
//test 0 or 1 or whatever degrees of seperation