use std::fs;

fn parse_input() -> Vec<u32> {
    fs::read_to_string("input.txt").unwrap().trim().split(",").map(|c| c.parse().unwrap()).collect()
}

fn fetch_values_from_vector(mem: &Vec<u32>, c: usize) -> (u32, u32, u32) {
    let (p1, p2, p3) = (mem[c+1], mem[c+2], mem[c+3]);
    (mem[p1 as usize], mem[p2 as usize], mem[p3 as usize])
}

fn compute_part1(mut arr2: Vec<u32>) -> Vec<u32> {
    let mut counter: usize = 0;
    loop {
        let opcode = arr2[counter];
        match opcode {
            1 => {
                let (p1, p2, p3) = fetch_values_from_vector(&arr2, counter);
                arr2[p3 as usize] = p1 + p2;
            },
            2 => {
                let (p1, p2, p3) = fetch_values_from_vector(&arr2, counter);
                arr2[p3 as usize] = p1 * p2;
            },
            99 => {
                break;
            },
             _ => {},
        }
        counter += 4;
    }
    arr2
}

fn main() {
    let arr: Vec<u32> = parse_input();

    println!("Value at 0th position is: {}", compute_part1(arr.clone())[0]);
    
}
