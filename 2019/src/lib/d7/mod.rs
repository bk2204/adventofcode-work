use crate::d2;
use permutator::Permutation;
use std::cell::RefCell;
use std::rc::Rc;

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
        let mut v = Box::new(vec![self.initial]);
        for (interp, &phase) in interps.iter_mut().zip(phases.iter()) {
            v.insert(0, phase);
            let r = interp
                .run(Rc::new(RefCell::new(v.into_iter())))
                .collect::<Result<Vec<_>, d2::Error>>()?;
            v = Box::new(r);
        }
        Ok(*v)
    }

    pub fn run_with_feedback(&self, phases: Vec<i64>) -> Result<Vec<i64>, d2::Error> {
        let mut inp = vec![self.initial];
        loop {
            let code = d2::Parser::parse(self.prog);
            let mut interps = vec![d2::Program::new(code); phases.len()];
            let mut v = Box::new(inp.clone());
            for (interp, &phase) in interps.iter_mut().zip(phases.iter()) {
                v.insert(0, phase);
                let r = interp
                    .run(Rc::new(RefCell::new(v.into_iter())))
                    .filter(|r| r.is_ok())
                    .collect::<Result<Vec<_>, d2::Error>>()?;
                v = Box::new(r);
            }
            if v.len() + 1 == inp.len() {
                return Ok(*v);
            }
            inp.push(v[inp.len() - 1]);
        }
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

    pub fn outputs_with_feedback(&self) -> Vec<i64> {
        let mut v = self.inputs.clone();
        // The permutation method doesn't include the identity permutation, so chain the iterators
        // together.
        vec![self.inputs.clone()]
            .into_iter()
            .chain(v.permutation())
            .map(|phases| {
                *self
                    .chain
                    .run_with_feedback(phases)
                    .unwrap()
                    .last()
                    .unwrap()
            })
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

    fn feedback(prog: &str) -> i64 {
        let inputs = vec![5, 6, 7, 8, 9];
        let s = Searcher::new(Chain::new(prog, 0), inputs);
        s.outputs_with_feedback().iter().cloned().max().unwrap()
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

    #[test]
    fn feedback_max_val() {
        assert_eq!(
            feedback("3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5"),
            139629729
        );

        assert_eq!(
            feedback("3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10"),
            18216
        );
    }
}
