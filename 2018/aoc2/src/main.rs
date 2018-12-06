use std::io::{self, BufRead, BufReader};
use std::fs::File;

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let data: Vec<String> = BufReader::new(file).lines().filter_map(|s| s.ok()).collect();
    Ok(())
}
