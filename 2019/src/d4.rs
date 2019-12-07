extern crate adventofcode;
use adventofcode::d4::Filter;
use std::io;

fn main() -> io::Result<()> {
    println!("{}", (240_298..784_956).filter(|&n| Filter::validate_p1(n)).count());
    println!("{}", (240_298..784_956).filter(|&n| Filter::validate_p2(n)).count());
    Ok(())
}
