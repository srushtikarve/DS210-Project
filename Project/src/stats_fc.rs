use std::collections::HashMap;

pub fn mean(data: &[f64]) -> Option<f64> { //finds the average number of node connections to find the typical number
    let len = data.len(); //adding up all the values in a dataset and dividing by the total number of values
    if len == 0 {
        return None; //this returns None if the input data is empty
    }

    let sum: f64 = data.iter().map(|&x| x as f64).sum(); //Calculates the sum of all elements in the slice
    Some(sum / len as f64) //Calculates the mean and return it
}

pub fn max(data: &HashMap<usize, Vec<usize>>) -> Option<usize> { //finds the maximum number of node connections so we can know who's most popular
    if data.is_empty() {
        return None; //Return None if the input data is empty
    }

    let mut max_length = 0;
    let mut max_node = None;

    for (&node, distances) in data {
        let length = distances.len();
        if length > max_length { //Updates max length node value if a greater one is found
            max_length = length;
            max_node = Some(node);
        }
    }

    max_node
}

pub fn mode(data: &[f64]) -> Option<f64> { //the mode function finds which degree of distance is the most common in the data provided
    if data.is_empty() { //value that appears most frequently in the dataset
        return None; //returns None if the input data is empty
    }

    let mut frequency_map: HashMap<usize, usize> = HashMap::new();

    for &value in data { //counts the frequency of each value in the data
        *frequency_map.entry(value as usize).or_insert(0) += 1;
    }

    let mut max_frequency = 0;
    let mut mode_value = data[0];
    for (&value, &frequency) in &frequency_map {
        if frequency > max_frequency { //updates the frequency with each extra count of distance found
            max_frequency = frequency;
            mode_value = value as f64;
        }
    }

    Some(mode_value) //returns the mode value
}