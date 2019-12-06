extern crate adventofcode;
use adventofcode::d2::{Parser, Program};
use std::io;
use std::io::BufRead;

fn program_for(inp: &str, noun: u64, verb: u64) -> u64 {
    let mut p = Program::new(Parser::parse(&inp));
    p[1] = noun;
    p[2] = verb;
    p.run();
    p[0]
}

fn search_for(inp: &str, val: u64) -> Option<(u64, u64)> {
    let v = Parser::parse(inp);
    let len = v.len() as u64;
    // We know that the only valid values are those which can be indices into the array, and
    // therefore they must also be smaller than the array size.
    for i in 0..len {
        for j in 0..len {
            if program_for(inp, i, j) == val {
                return Some((i, j));
            }
        }
    }
    None
}

fn main() -> io::Result<()> {
    let b = io::BufReader::new(io::stdin());
    if let Some(inp) = b.lines().next() {
        let inp = &inp?;
        println!("{}", program_for(inp, 12, 2));
        match search_for(inp, 19690720) {
            Some((noun, verb)) => println!("{}", 100 * noun + verb),
            None => eprintln!("No solution found"),
        }
    }
    Ok(())
}