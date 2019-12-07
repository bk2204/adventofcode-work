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
    JumpIfTrue(Parameter, Parameter),
    JumpIfFalse(Parameter, Parameter),
    LessThan(Parameter, Parameter, usize),
    Equals(Parameter, Parameter, usize),
}

#[derive(Clone)]
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

pub struct Iter<'a> {
    prog: &'a mut Program,
    iter: &'a mut dyn Iterator<Item = i64>,
}

impl<'a> Iter<'a> {
    fn new(prog: &'a mut Program, iter: &'a mut dyn Iterator<Item = i64>) -> Self {
        Iter { prog, iter }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = Result<i64, Error>;

    fn next(&mut self) -> Option<Result<i64, Error>> {
        while let Some(op) = self.prog.next() {
            match op {
                Instruction::Add(a, b, s) => {
                    self.prog.data[s] = self.prog.load(a) + self.prog.load(b)
                }
                Instruction::Mul(a, b, s) => {
                    self.prog.data[s] = self.prog.load(a) * self.prog.load(b)
                }
                Instruction::Input(s) => {
                    self.prog.data[s] = match self.iter.next() {
                        Some(x) => x,
                        None => return Some(Err(Error::OutOfData)),
                    }
                }
                Instruction::Output(a) => return Some(Ok(self.prog.load(a))),
                Instruction::JumpIfTrue(a, b) => {
                    if self.prog.load(a) != 0 {
                        self.prog.off = self.prog.load(b) as usize;
                    }
                }
                Instruction::JumpIfFalse(a, b) => {
                    if self.prog.load(a) == 0 {
                        self.prog.off = self.prog.load(b) as usize;
                    }
                }
                Instruction::LessThan(a, b, s) => {
                    self.prog.data[s] = if self.prog.load(a) < self.prog.load(b) {
                        1
                    } else {
                        0
                    }
                }
                Instruction::Equals(a, b, s) => {
                    self.prog.data[s] = if self.prog.load(a) == self.prog.load(b) {
                        1
                    } else {
                        0
                    }
                }
            }
        }
        None
    }
}

impl Program {
    pub fn new(data: Vec<i64>) -> Self {
        Program { data, off: 0 }
    }

    pub fn run<'a>(&'a mut self, iter: &'a mut dyn Iterator<Item = i64>) -> Iter<'a> {
        Iter::new(self, iter)
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
            5 => {
                self.off += 3;
                Some(Instruction::JumpIfTrue(
                    self.decode_param(op / 100, self.data[off + 1]),
                    self.decode_param(op / 1000, self.data[off + 2]),
                ))
            }
            6 => {
                self.off += 3;
                Some(Instruction::JumpIfFalse(
                    self.decode_param(op / 100, self.data[off + 1]),
                    self.decode_param(op / 1000, self.data[off + 2]),
                ))
            }
            7 => {
                self.off += 4;
                Some(Instruction::LessThan(
                    self.decode_param(op / 100, self.data[off + 1]),
                    self.decode_param(op / 1000, self.data[off + 2]),
                    self.data[off + 3] as usize,
                ))
            }
            8 => {
                self.off += 4;
                Some(Instruction::Equals(
                    self.decode_param(op / 100, self.data[off + 1]),
                    self.decode_param(op / 1000, self.data[off + 2]),
                    self.data[off + 3] as usize,
                ))
            }
            99 => None,
            _ => panic!("invalid opcode {}", op),
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
        data.split(',')
            .map(|s| i64::from_str_radix(s, 10).unwrap())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::{Error, Parser, Program};

    fn process(inp: &str) -> (Vec<i64>, Vec<i64>) {
        let v: Vec<i64> = Vec::new();
        let mut p = Program::new(Parser::parse(inp));
        let r = p
            .run(&mut v.into_iter())
            .collect::<Result<Vec<_>, Error>>()
            .unwrap();
        (p.into_iter().collect(), r)
    }

    fn process_with_input(inp: &str, data: &str) -> (Vec<i64>, Vec<i64>) {
        let v = Parser::parse(data);
        let mut p = Program::new(Parser::parse(inp));
        let r = p
            .run(&mut v.into_iter())
            .collect::<Result<Vec<_>, Error>>()
            .unwrap();
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

        // Compare for equality to 8.
        assert_eq!(
            process_with_input("3,9,8,9,10,9,4,9,99,-1,8", "8").1,
            vec![1]
        );
        assert_eq!(
            process_with_input("3,9,8,9,10,9,4,9,99,-1,8", "7").1,
            vec![0]
        );
        assert_eq!(
            process_with_input("3,9,8,9,10,9,4,9,99,-1,8", "9").1,
            vec![0]
        );
        assert_eq!(process_with_input("3,3,1108,-1,8,3,4,3,99", "8").1, vec![1]);
        assert_eq!(process_with_input("3,3,1108,-1,8,3,4,3,99", "7").1, vec![0]);
        assert_eq!(process_with_input("3,3,1108,-1,8,3,4,3,99", "9").1, vec![0]);

        // Compare for less than 8.
        assert_eq!(
            process_with_input("3,9,7,9,10,9,4,9,99,-1,8", "8").1,
            vec![0]
        );
        assert_eq!(
            process_with_input("3,9,7,9,10,9,4,9,99,-1,8", "7").1,
            vec![1]
        );
        assert_eq!(
            process_with_input("3,9,7,9,10,9,4,9,99,-1,8", "9").1,
            vec![0]
        );
        assert_eq!(process_with_input("3,3,1107,-1,8,3,4,3,99", "8").1, vec![0]);
        assert_eq!(process_with_input("3,3,1107,-1,8,3,4,3,99", "7").1, vec![1]);
        assert_eq!(process_with_input("3,3,1107,-1,8,3,4,3,99", "9").1, vec![0]);

        // Compare for inequality to 0.
        assert_eq!(
            process_with_input("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", "-1").1,
            vec![1]
        );
        assert_eq!(
            process_with_input("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", "0").1,
            vec![0]
        );
        assert_eq!(
            process_with_input("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", "1").1,
            vec![1]
        );
        assert_eq!(
            process_with_input("3,3,1105,-1,9,1101,0,0,12,4,12,99,1", "-1").1,
            vec![1]
        );
        assert_eq!(
            process_with_input("3,3,1105,-1,9,1101,0,0,12,4,12,99,1", "0").1,
            vec![0]
        );
        assert_eq!(
            process_with_input("3,3,1105,-1,9,1101,0,0,12,4,12,99,1", "1").1,
            vec![1]
        );

        // Return 999 if < 8, 1000 if == 8, and 1001 otherwise.
        let prog = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
        assert_eq!(process_with_input(prog, "8").1, vec![1000]);
        assert_eq!(process_with_input(prog, "7").1, vec![999]);
        assert_eq!(process_with_input(prog, "9").1, vec![1001]);
    }
}
