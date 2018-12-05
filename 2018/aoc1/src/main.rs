use std::fs;

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();
    let frequency = data
        .lines()
        .fold(0, |acc, x| x.parse().unwrap_or(0) + acc);
    println!("Frequency: {}", frequency);
}
