use std::ops::{Index, IndexMut};

enum Instruction {
    Add(usize, usize, usize),
    Mul(usize, usize, usize),
}

pub struct Program {
    data: Vec<u64>,
    off: usize,
}

impl Program {
    pub fn new(data: Vec<u64>) -> Self {
        Program { data, off: 0 }
    }

    pub fn run(&mut self) {
        while let Some(op) = self.next() {
            match op {
                Instruction::Add(a, b, s) => self.data[s] = self.data[a] + self.data[b],
                Instruction::Mul(a, b, s) => self.data[s] = self.data[a] * self.data[b],
            }
        }
    }

    fn next(&mut self) -> Option<Instruction> {
        let off = self.off;
        match self.data[off] {
            1 => {
                self.off += 4;
                Some(Instruction::Add(
                    self.data[off + 1] as usize,
                    self.data[off + 2] as usize,
                    self.data[off + 3] as usize,
                ))
            }
            2 => {
                self.off += 4;
                Some(Instruction::Mul(
                    self.data[off + 1] as usize,
                    self.data[off + 2] as usize,
                    self.data[off + 3] as usize,
                ))
            }
            99 => None,
            _ => panic!("invalid opcode"),
        }
    }
}

impl Index<usize> for Program {
    type Output = u64;

    fn index(&self, idx: usize) -> &u64 {
        self.data.index(idx)
    }
}

impl IndexMut<usize> for Program {
    fn index_mut(&mut self, idx: usize) -> &mut u64 {
        self.data.index_mut(idx)
    }
}

impl IntoIterator for Program {
    type Item = u64;
    type IntoIter = ::std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

pub struct Parser {}

impl Parser {
    pub fn parse(data: &str) -> Vec<u64> {
        data.split(",")
            .map(|s| u64::from_str_radix(s, 10).unwrap())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::{Parser, Program};

    fn process(inp: &str) -> Vec<u64> {
        let mut p = Program::new(Parser::parse(inp));
        p.run();
        p.into_iter().collect()
    }

    #[test]
    fn integration() {
        assert_eq!(process("1,0,0,0,99"), Parser::parse("2,0,0,0,99"));
        assert_eq!(process("2,3,0,3,99"), Parser::parse("2,3,0,6,99"));
        assert_eq!(process("2,4,4,5,99,0"), Parser::parse("2,4,4,5,99,9801"));
        assert_eq!(process("1,1,1,4,99,5,6,0,99"), Parser::parse("30,1,1,4,2,5,6,0,99"));
        assert_eq!(
            process("1,9,10,3,2,3,11,0,99,30,40,50"),
            Parser::parse("3500,9,10,70,2,3,11,0,99,30,40,50")
        );
    }
}
