extern crate adventofcode;
use adventofcode::d2::{Parser, Program};
use std::io;
use std::io::BufRead;

fn main() -> io::Result<()> {
    let b = io::BufReader::new(io::stdin());
    if let Some(inp) = b.lines().next() {
        let mut p = Program::new(Parser::parse(&inp?));
        p[1] = 12;
        p[2] = 2;
        p.run();
        println!("{}", p[0]);
    }
    Ok(())
}
