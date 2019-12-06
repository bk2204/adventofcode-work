extern crate adventofcode;
use adventofcode::d3::{Graph, Parser};
use std::io;
use std::io::BufRead;

fn distance(paths: Vec<String>) -> i64 {
    let mut g = Graph::new(paths.iter().map(|p| Parser::parse(&p)).collect());
    g.run();
    g.intersections()
        .map(|p| p.0.abs() + p.1.abs())
        .filter(|&v| v > 0)
        .min()
        .unwrap()
}

fn main() -> io::Result<()> {
    let b = io::BufReader::new(io::stdin());
    println!(
        "{}",
        distance(b.lines().collect::<Result<Vec<String>, io::Error>>()?)
    );
    Ok(())
}
