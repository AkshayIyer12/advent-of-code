use std::fs;

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();
    println!("Sum of fuel requirement is: {}",  calculate_module_fuel(&data));
    println!("Sum of fuel requirement, taking into account the mass of fuel is: {}", calculate_fuel_for_fuel(&data));
}

fn calculate_module_fuel(data: &String) -> i32 {
    data
        .lines()
        .fold(0, |mut acc, x| {
            let div: f64 = (x.parse::<i32>().unwrap()/3).into();
            let sub = (div.round() as i32) - 2;
            println!("{} -> fuel is: {}", x, sub);
            acc += sub;
            acc
        })
}

fn calculate_fuel_for_fuel(data: &String) -> f64 {
    data
        .lines()
        .fold(0.0, |mut acc, x| {
            acc += calculate_mod_fuel_recr(x.parse::<f64>().unwrap(), 0.0);
            acc
        })
}


fn calculate_mod_fuel_recr(mut fuel: f64, mut accum: f64) -> f64 {
    while fuel > 0.0 {
        let data = fuel/3.0;
        fuel = data.floor() - 2.0;
        if fuel > 0.0 {
            accum += fuel;
        }
    }
    accum
}
