use regex::Regex;
use std::collections::BTreeMap;
use std::io::{self, BufRead};

pub struct Parser<'a> {
    rdr: Option<io::BufReader<Box<dyn io::Read + 'a>>>,
}

impl<'a> Parser<'a> {
    pub fn new(rdr: Box<dyn io::Read + 'a>) -> Self {
        Parser {
            rdr: Some(io::BufReader::new(rdr)),
        }
    }

    pub fn distance(&mut self) -> u64 {
        let lists = self.parse_lists();
        let d = self.distances(lists);
        d.iter().sum()
    }

    pub fn similarity_score(&mut self) -> u64 {
        let lists = self.parse_lists();
        let d = self.similarity(lists);
        d.iter().sum()
    }

    fn parse_lists(&mut self) -> (Vec<u64>, Vec<u64>) {
        let (mut a, mut b) = (Vec::new(), Vec::new());
        let re = Regex::new(r"(\d+)\s+(\d+)").unwrap();
        for ln in self.rdr.take().unwrap().lines() {
            let ln = ln.unwrap();
            if ln.is_empty() {
                continue;
            }
            let cap = re.captures(&ln).unwrap();
            a.push(cap[1].parse().unwrap());
            b.push(cap[2].parse().unwrap());
        }
        (a, b)
    }

    fn distances(&self, lists: (Vec<u64>, Vec<u64>)) -> Vec<u64> {
        let (mut a, mut b) = lists;
        a.sort();
        b.sort();
        a.iter()
            .zip(b.iter())
            .map(|(a, b)| a.abs_diff(*b))
            .collect()
    }

    fn similarity(&self, lists: (Vec<u64>, Vec<u64>)) -> Vec<u64> {
        let (a, b) = lists;
        let counts = b.iter().fold(BTreeMap::new(), |mut accum, item| {
            *accum.entry(item).or_insert(0) += 1;
            accum
        });
        a.iter()
            .map(|item| item * counts.get(item).unwrap_or(&0))
            .collect()
    }
}

fn main() {
    let data = io::read_to_string(io::stdin()).unwrap();
    let mut p = Parser::new(Box::new(io::Cursor::new(&data)));
    println!("{}", p.distance());

    let mut p = Parser::new(Box::new(io::Cursor::new(&data)));
    println!("{}", p.similarity_score());
}

#[cfg(test)]
mod tests {
    use super::Parser;
    use std::io;

    const DATA: &str = "3   4
4   3
2   5
1   3
3   9
3   3
";

    fn cursor() -> io::Cursor<&'static str> {
        io::Cursor::new(DATA)
    }

    #[test]
    fn expected_values() {
        let mut p = Parser::new(Box::new(cursor()));
        let (a, b) = p.parse_lists();
        assert_eq!(a, &[3, 4, 2, 1, 3, 3]);
        assert_eq!(b, &[4, 3, 5, 3, 9, 3]);

        let p = Parser::new(Box::new(cursor()));
        let d = p.distances((a.clone(), b.clone()));
        assert_eq!(d, &[2, 1, 0, 1, 2, 5]);

        let p = Parser::new(Box::new(cursor()));
        let s = p.similarity((a, b));
        assert_eq!(s, &[9, 4, 0, 0, 9, 9]);

        let mut p = Parser::new(Box::new(cursor()));
        assert_eq!(p.distance(), 11);

        let mut p = Parser::new(Box::new(cursor()));
        assert_eq!(p.similarity_score(), 31);
    }
}
