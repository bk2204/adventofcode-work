extern crate adventofcode;
use adventofcode::d8::Image;
use std::io;
use std::io::BufRead;

fn min_zero_layer(inp: &[u8]) -> usize {
    let img = Image::new(25, 6, inp);
    img.iter()
        .map(|l| {
            let summary = l.summary();
            (summary[&b'0'], summary[&b'1'] * summary[&b'2'])
        })
        .min()
        .unwrap()
        .1
}

fn image(inp: &[u8]) -> String {
    let img = Image::new(25, 6, inp);
    String::from_utf8(img.image()).unwrap()
}

fn main() -> io::Result<()> {
    let b = io::BufReader::new(io::stdin());
    if let Some(inp) = b.lines().next() {
        let inp = &inp?;
        let bytes: Vec<u8> = inp.bytes().collect();
        println!("{:?}", min_zero_layer(&bytes));
        println!("{}", image(&bytes));
    }
    Ok(())
}
