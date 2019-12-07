extern crate adventofcode;
use adventofcode::d4::Filter;
use std::io;

fn main() -> io::Result<()> {
    println!("{}", (240_298..784_956).filter(|&n| Filter::validate(n)).count());
    Ok(())
}
