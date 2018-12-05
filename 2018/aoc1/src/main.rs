use std::fs;
use std::collections::HashSet;

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();
    println!("Final Frequency: {}", final_frequency(&data));
    println!("First_frequency: {}", first_frequency(&data));
}

fn final_frequency(data: &String) -> i32 {
    data
        .lines()
        .fold(0, |acc, x| x.parse().unwrap_or(0) + acc)
}

fn first_frequency(data: &String) -> i32 {
    let mut freq: i32 = 0;
    let mut frequencies_observed = HashSet::new();
    frequencies_observed.insert(0);
    loop {
        for line in data.lines() {
            let change: i32 = line.parse().expect("Error in parsing string to integer");
            freq += change;
            if frequencies_observed.contains(&freq) {
                return freq
            }
            frequencies_observed.insert(freq);
        }   
    }
}
