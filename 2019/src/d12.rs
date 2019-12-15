extern crate adventofcode;
use adventofcode::d12::{Parser, Universe};
use std::io;
use std::io::BufRead;

fn main() -> io::Result<()> {
    let b = io::BufReader::new(io::stdin());
    let v: Vec<_> = b.lines().map(|r| Parser::parse(&r.unwrap())).collect();
    let mut u = Universe::new(v.clone());
    for _ in 0..1000 {
        u.step();
    }
    println!(
        "{:?}",
        u.moons().iter().map(|m| m.total_energy()).sum::<usize>()
    );
    let mut u = Universe::new(v);
    println!("{:?}", u.period());
    Ok(())
}
