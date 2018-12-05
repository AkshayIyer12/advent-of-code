use std::fs;

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();
    println!("Frequency: {}", final_frequency(data));
}

fn final_frequency(data: String) -> i32 {
    data
        .lines()
        .fold(0, |acc, x| x.parse().unwrap_or(0) + acc)
}
