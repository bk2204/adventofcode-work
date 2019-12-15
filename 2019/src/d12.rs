extern crate adventofcode;
use adventofcode::d12::{Universe, Parser};
use std::io;
use std::io::BufRead;

fn main() -> io::Result<()> {
    let b = io::BufReader::new(io::stdin());
    let mut u = Universe::new(b.lines().map(|r| Parser::parse(&r.unwrap())).collect());
    for _ in 0..1000 {
        u.step();
    }
    println!("{:?}", u.moons().iter().map(|m| m.total_energy()).sum::<usize>());
    Ok(())
}
