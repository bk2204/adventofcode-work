extern crate adventofcode;
use adventofcode::d6::{Graph, Parser};
use std::io;
use std::io::BufRead;

fn total_orbits(orbits: &[String]) -> usize {
    let orbits = Parser::parse(orbits);
    let counts = Graph::new(orbits).traverse();
    counts.iter().map(|(_, v)| *v).sum::<usize>()
}

fn transfers_between(orbits: &[String]) -> usize {
    let orbits = Parser::parse(orbits);
    Graph::new(orbits).transfers_between("YOU", "SAN") - 2
}

fn main() -> io::Result<()> {
    let b = io::BufReader::new(io::stdin());
    let v = b.lines().collect::<Result<Vec<String>, io::Error>>()?;
    println!("{}", total_orbits(&v));
    println!("{}", transfers_between(&v));
    Ok(())
}
