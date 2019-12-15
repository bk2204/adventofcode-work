extern crate adventofcode;
use adventofcode::d13::Arcade;
use adventofcode::d2::Parser;
use std::io;
use std::io::BufRead;

fn count_for(inp: &str, typ: i64) -> usize {
    let mut a = Arcade::new(Parser::parse(&inp));
    a.run();
    a.square_count()[&typ]
}

fn main() -> io::Result<()> {
    let b = io::BufReader::new(io::stdin());
    if let Some(inp) = b.lines().next() {
        let inp = &inp?;
        println!("{:?}", count_for(inp, 2));
    }
    Ok(())
}
