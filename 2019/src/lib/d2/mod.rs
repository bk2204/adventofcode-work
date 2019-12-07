use std::error;
use std::fmt;
use std::io;
use std::ops::{Index, IndexMut};

#[derive(Copy, Clone)]
enum Parameter {
    Position(usize),
    Immediate(i64),
}

enum Instruction {
    Add(Parameter, Parameter, usize),
    Mul(Parameter, Parameter, usize),
    Input(usize),
    Output(Parameter),
}

pub struct Program {
    data: Vec<i64>,
    off: usize,
}

#[derive(Debug)]
pub enum Error {
    OutOfData,
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            Error::OutOfData => write!(f, "Out of data"),
        }
    }
}

impl Into<io::Error> for Error {
    fn into(self) -> io::Error {
        io::Error::new(io::ErrorKind::InvalidData, self)
    }
}

impl Program {
    pub fn new(data: Vec<i64>) -> Self {
        Program { data, off: 0 }
    }

    pub fn run(&mut self, mut iter: Box<dyn Iterator<Item = i64>>) -> Result<Vec<i64>, Error> {
        let mut v = Vec::new();
        while let Some(op) = self.next() {
            match op {
                Instruction::Add(a, b, s) => self.data[s] = self.load(a) + self.load(b),
                Instruction::Mul(a, b, s) => self.data[s] = self.load(a) * self.load(b),
                Instruction::Input(s) => {
                    self.data[s] = match iter.next() {
                        Some(x) => x,
                        None => return Err(Error::OutOfData),
                    }
                }
                Instruction::Output(a) => v.push(self.load(a)),
            }
        }
        Ok(v)
    }

    fn load(&self, p: Parameter) -> i64 {
        match p {
            Parameter::Position(x) => self.data[x],
            Parameter::Immediate(x) => x,
        }
    }

    fn decode_param(&self, mode: i64, val: i64) -> Parameter {
        if mode % 10 == 0 {
            Parameter::Position(val as usize)
        } else {
            Parameter::Immediate(val)
        }
    }

    fn next(&mut self) -> Option<Instruction> {
        let off = self.off;
        let op = self.data[off];
        match op % 100 {
            1 => {
                self.off += 4;
                Some(Instruction::Add(
                    self.decode_param(op / 100, self.data[off + 1]),
                    self.decode_param(op / 1000, self.data[off + 2]),
                    self.data[off + 3] as usize,
                ))
            }
            2 => {
                self.off += 4;
                Some(Instruction::Mul(
                    self.decode_param(op / 100, self.data[off + 1]),
                    self.decode_param(op / 1000, self.data[off + 2]),
                    self.data[off + 3] as usize,
                ))
            }
            3 => {
                self.off += 2;
                Some(Instruction::Input(self.data[off + 1] as usize))
            }
            4 => {
                self.off += 2;
                Some(Instruction::Output(
                    self.decode_param(op / 100, self.data[off + 1]),
                ))
            }
            99 => None,
            _ => panic!("invalid opcode"),
        }
    }
}

impl Index<usize> for Program {
    type Output = i64;

    fn index(&self, idx: usize) -> &i64 {
        self.data.index(idx)
    }
}

impl IndexMut<usize> for Program {
    fn index_mut(&mut self, idx: usize) -> &mut i64 {
        self.data.index_mut(idx)
    }
}

impl IntoIterator for Program {
    type Item = i64;
    type IntoIter = ::std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

pub struct Parser {}

impl Parser {
    pub fn parse(data: &str) -> Vec<i64> {
        data.split(",")
            .map(|s| i64::from_str_radix(s, 10).unwrap())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::{Parser, Program};

    fn process(inp: &str) -> (Vec<i64>, Vec<i64>) {
        let v: Vec<i64> = Vec::new();
        let mut p = Program::new(Parser::parse(inp));
        let r = p.run(Box::new(v.into_iter())).unwrap();
        (p.into_iter().collect(), r)
    }

    fn process_with_input(inp: &str, data: &str) -> (Vec<i64>, Vec<i64>) {
        let v = Parser::parse(data);
        let mut p = Program::new(Parser::parse(inp));
        let r = p.run(Box::new(v.into_iter())).unwrap();
        (p.into_iter().collect(), r)
    }

    #[test]
    fn integration() {
        assert_eq!(process("1,0,0,0,99"), (Parser::parse("2,0,0,0,99"), vec![]));
        assert_eq!(process("2,3,0,3,99"), (Parser::parse("2,3,0,6,99"), vec![]));
        assert_eq!(
            process("2,4,4,5,99,0"),
            (Parser::parse("2,4,4,5,99,9801"), vec![])
        );
        assert_eq!(
            process("1,1,1,4,99,5,6,0,99"),
            (Parser::parse("30,1,1,4,2,5,6,0,99"), vec![])
        );
        assert_eq!(
            process("1,9,10,3,2,3,11,0,99,30,40,50"),
            (Parser::parse("3500,9,10,70,2,3,11,0,99,30,40,50"), vec![])
        );
        assert_eq!(
            process("1002,4,3,4,33"),
            (Parser::parse("1002,4,3,4,99"), vec![])
        );
        assert_eq!(
            process_with_input("3,0,4,0,99", "10"),
            (Parser::parse("10,0,4,0,99"), vec![10])
        );
        assert_eq!(
            process_with_input("3,0,4,0,99", "-3"),
            (Parser::parse("-3,0,4,0,99"), vec![-3])
        );
        assert_eq!(
            process("1101,100,-1,4,0"),
            (Parser::parse("1101,100,-1,4,99"), vec![])
        );
    }
}
