extern crate adventofcode;
use adventofcode::d3::{Graph, Parser};
use std::io;
use std::io::BufRead;

fn distance(paths: Vec<String>) -> i64 {
    let mut g = Graph::new(paths.iter().map(|p| Parser::parse(&p)).collect());
    g.run();
    g.intersections()
        .map(|(p, _)| p.0.abs() + p.1.abs())
        .filter(|&v| v > 0)
        .min()
        .unwrap()
}

fn steps(paths: Vec<String>) -> usize {
    let mut g = Graph::new(paths.iter().map(|p| Parser::parse(&p)).collect());
    g.run();
    g.intersections()
        .map(|(_, steps)| steps)
        .filter(|&v| v > 0)
        .min()
        .unwrap()
}

fn main() -> io::Result<()> {
    let b = io::BufReader::new(io::stdin());
    let v = b.lines().collect::<Result<Vec<String>, io::Error>>()?;
    println!("{}", distance(v.clone()));
    println!("{}", steps(v));
    Ok(())
}
