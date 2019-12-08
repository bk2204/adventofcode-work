extern crate adventofcode;
use adventofcode::d7::{Chain, Searcher};
use std::io;
use std::io::BufRead;

fn program_for(inp: &str) -> i64 {
    let inputs = vec![0, 1, 2, 3, 4];
    let s = Searcher::new(Chain::new(inp, 0), inputs);
    s.outputs().iter().cloned().max().unwrap()
}

fn feedback_program_for(inp: &str) -> i64 {
    let inputs = vec![5, 6, 7, 8, 9];
    let s = Searcher::new(Chain::new(inp, 0), inputs);
    s.outputs_with_feedback().iter().cloned().max().unwrap()
}

fn main() -> io::Result<()> {
    let b = io::BufReader::new(io::stdin());
    if let Some(inp) = b.lines().next() {
        let inp = &inp?;
        println!("{:?}", program_for(inp));
        println!("{:?}", feedback_program_for(inp));
    }
    Ok(())
}
