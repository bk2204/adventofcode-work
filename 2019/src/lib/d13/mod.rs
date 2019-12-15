use super::d2::Program;
use itertools::Itertools;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Point(pub usize, pub usize);

pub struct Arcade {
    prog: Vec<i64>,
    map: BTreeMap<Point, i64>,
}

impl Arcade {
    pub fn new(prog: Vec<i64>) -> Self {
        Arcade {
            prog,
            map: BTreeMap::new(),
        }
    }

    pub fn run(&mut self) {
        let mut p = Program::new(self.prog.clone());
        let iter = p
            .run(Rc::new(RefCell::new(vec![].into_iter())))
            .map(|r| r.unwrap());
        self.process_sequence(iter);
    }

    fn process_sequence<I: Iterator<Item = i64>>(&mut self, iter: I) {
        self.map = iter
            .chunks(3)
            .into_iter()
            .map(|s| {
                let v: Vec<_> = s.collect();
                (Point(v[0] as usize, v[1] as usize), v[2])
            })
            .collect();
    }

    pub fn square_count(&self) -> BTreeMap<i64, usize> {
        let mut res = BTreeMap::new();
        self.map.iter().fold(&mut res, |res, (_, val)| {
            if let Some(count) = res.get_mut(val) {
                *count += 1;
            } else {
                res.insert(*val, 1);
            }
            res
        });
        res
    }
}

#[cfg(test)]
mod tests {
    use super::{Arcade, Point};

    #[test]
    fn simple() {
        let mut a = Arcade::new(vec![]);
        a.process_sequence(vec![1, 2, 3, 6, 5, 4].into_iter());
        assert_eq!(
            a.map.iter().map(|(&p, &i)| (p, i)).collect::<Vec<(Point, i64)>>(),
            vec![(Point(1, 2), 3), (Point(6, 5), 4)]
        );
        assert_eq!(a.square_count().get(&3), Some(&1));
        assert_eq!(a.square_count().get(&4), Some(&1));
        assert_eq!(a.square_count().get(&0), None);
    }
}
