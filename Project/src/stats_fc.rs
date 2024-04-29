use std::collections::HashMap;

pub fn mean(data: &[f64]) -> Option<f64> { //finds the average number of node connections to find the typical number
    let len = data.len();
    if len == 0 {
        return None; //this returns None if the input data is empty
    }

    let sum: f64 = data.iter().map(|&x| x as f64).sum(); //Calculates the sum of all elements in the slice
    Some(sum / len as f64) //Calculate the mean and return it
}

pub fn max(data: &[f64]) -> Option<f64> { //finds the maximum number of node connections so we can know who's most popular
    if data.is_empty() {
        return None; //Return None if the input data is empty
    }

    let mut max_value = data[0]; //Initialize max_value with the first element
    for &value in data.iter().skip(1) {
        if value > max_value {
            max_value = value; //Update max_value if a greater value is found
        }
    }
    Some(max_value) //Return the maximum value
}

pub fn mode(data: &[f64]) -> Option<f64> {
    if data.is_empty() {
        return None; // Return None if the input data is empty
    }

    let mut frequency_map: HashMap<usize, usize> = HashMap::new();

    for &value in data { // Counts the frequency of each value in the data
        *frequency_map.entry(value as usize).or_insert(0) += 1;
    }

    let mut max_frequency = 0;
    let mut mode_value = data[0];
    for (&value, &frequency) in &frequency_map {
        if frequency > max_frequency {
            max_frequency = frequency;
            mode_value = value as f64;
        }
    }

    Some(mode_value) // Return the mode value
}