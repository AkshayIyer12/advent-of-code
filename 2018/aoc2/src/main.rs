use std::io::{self, BufRead, BufReader};
use std::fs::File;
use std::collections::HashMap;

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let boxes: Vec<String> = BufReader::new(file).lines().filter_map(|s| s.ok()).collect();
    let mut two_letters: u32 = 0;
    let mut three_letters: u32 = 0;
    for box_id in boxes {
        let mut letter_counter = HashMap::new();

        for letter in box_id.chars() {
            *letter_counter.entry(letter).or_insert(0) += 1
        }

        if letter_counter.values().any(|n| *n == 2) {
            two_letters += 1;
        }

        if letter_counter.values().any(|n| *n == 3) {
            three_letters += 1;
        }
    }
    println!("Checksum is {}", two_letters * three_letters);
    Ok(())
}
