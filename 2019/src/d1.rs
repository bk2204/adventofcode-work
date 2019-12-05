extern crate adventofcode;
use adventofcode::d1::*;
use std::io;

fn main() {
    let v = parse_file(Box::new(io::BufReader::new(io::stdin())));
    println!("{}", total_fuel(&v, &fuel_for_module));
    println!("{}", total_fuel(&v, &fuel_for_module_recursive));
}

