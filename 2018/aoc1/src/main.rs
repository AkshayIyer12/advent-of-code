use std::fs;

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();
    let mut frequency = 0;
    for num in data.lines() {
        let change: i32  = num.parse().unwrap_or(0);
        frequency += change;
    }

    println!("Frequency: {}", frequency);
}
