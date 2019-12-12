extern crate adventofcode;
use adventofcode::d2::{Error, Parser, Program};
use std::io;
use std::io::BufRead;

fn program_for(inp: &str, v: Vec<i64>) -> Vec<i64> {
    let mut p = Program::new(Parser::parse(&inp));
    p.run(&mut v.into_iter())
        .collect::<Result<Vec<_>, Error>>()
        .unwrap()
}

fn main() -> io::Result<()> {
    let b = io::BufReader::new(io::stdin());
    if let Some(inp) = b.lines().next() {
        let inp = &inp?;
        println!("{:?}", program_for(inp, vec![1]));
        println!("{:?}", program_for(inp, vec![2]));
    }
    Ok(())
}
