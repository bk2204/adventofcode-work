use crate::d2;
use permutator::Permutation;

pub struct Chain<'a> {
    prog: &'a str,
    initial: i64,
}

impl<'a> Chain<'a> {
    pub fn new(prog: &'a str, initial: i64) -> Self {
        Chain { prog, initial }
    }

    pub fn run(&self, phases: Vec<i64>) -> Result<Vec<i64>, d2::Error> {
        let code = d2::Parser::parse(self.prog);
        let mut interps = vec![d2::Program::new(code); phases.len()];
        let mut v = vec![self.initial];
        for (interp, &phase) in interps.iter_mut().zip(phases.iter()) {
            v.insert(0, phase);
            let r = interp
                .run(&mut v.iter().cloned())
                .collect::<Result<Vec<_>, d2::Error>>()?;
            v = r;
        }
        Ok(v)
    }
}

pub struct Searcher<'a> {
    chain: Chain<'a>,
    inputs: Vec<i64>,
}

impl<'a> Searcher<'a> {
    pub fn new(chain: Chain<'a>, inputs: Vec<i64>) -> Self {
        Searcher { chain, inputs }
    }

    pub fn outputs(&self) -> Vec<i64> {
        let mut v = self.inputs.clone();
        // The permutation method doesn't include the identity permutation, so chain the iterators
        // together.
        vec![self.inputs.clone()]
            .into_iter()
            .chain(v.permutation())
            .map(|phases| self.chain.run(phases).unwrap()[0])
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::{Chain, Searcher};

    fn find_max(prog: &str) -> i64 {
        let inputs = vec![0, 1, 2, 3, 4];
        let s = Searcher::new(Chain::new(prog, 0), inputs);
        s.outputs().iter().cloned().max().unwrap()
    }

    #[test]
    fn max_val() {
        assert_eq!(
            find_max("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0"),
            43210
        );
        assert_eq!(
            find_max("3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0"),
            54321
        );
        assert_eq!(find_max("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0"), 65210);
    }
}
