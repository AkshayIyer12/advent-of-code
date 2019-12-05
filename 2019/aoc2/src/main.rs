use std::fs;

fn parse_input() -> Vec<u32> {
    fs::read_to_string("input.txt").unwrap().trim().split(",").map(|c| c.parse().unwrap()).collect()
}

fn fetch_values_from_vector(mem: &Vec<u32>, c: usize) -> (u32, u32) {
    let (p1, p2) = (mem[c+1], mem[c+2]);
    let first: usize = p1 as usize;
    let second: usize = p2 as usize;
    (mem[first], mem[second])
}

fn compute_part1(mut arr2: Vec<u32>) -> Vec<u32> {
    let mut counter: usize = 0;
    loop {
        let opcode = arr2[counter];
        match opcode {
            1 => {
                let (p1, p2) = fetch_values_from_vector(&arr2, counter);
                let target: u32 = arr2[counter+3];
                arr2[target as usize] = p1 + p2;
            },
            2 => {
                let (p1, p2) = fetch_values_from_vector(&arr2, counter);
                let target: u32 = arr2[counter+3];
                arr2[target as usize] = p1 * p2;
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

fn compute_part2(arr: Vec<u32>) -> u32 {
    for noun in 0..99 {
        for verb in 0..99 {
            let mut arr2: Vec<u32> = arr.clone();
            arr2[1] = noun;
            arr2[2] = verb;
            let result = compute_part1(arr2);
            if result[0] == 19690720 as u32 {
                return 100 * noun + verb;
            }
        }
    }
    0
}


fn main() {
    let arr: Vec<u32> = parse_input();

    println!("Value at 0th position is: {}", compute_part1(arr.clone())[0]);
    println!("Part2 answer is: {}", compute_part2(arr));
    
}
