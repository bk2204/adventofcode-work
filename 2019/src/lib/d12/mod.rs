use std::cmp::Ordering;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Point(pub isize, pub isize, pub isize);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Velocity(pub isize, pub isize, pub isize);

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Moon {
    point: Point,
    velocity: Velocity,
}

impl Moon {
    pub fn new(point: Point) -> Self {
        Moon {
            point,
            velocity: Velocity(0, 0, 0),
        }
    }

    fn update_velocity(&mut self, tuple: (isize, isize, isize)) {
        self.velocity.0 += tuple.0;
        self.velocity.1 += tuple.1;
        self.velocity.2 += tuple.2;
    }

    fn apply_velocity(&mut self) {
        self.point.0 += self.velocity.0;
        self.point.1 += self.velocity.1;
        self.point.2 += self.velocity.2;
    }

    pub fn potential_energy(&self) -> usize {
        (self.point.0.abs() + self.point.1.abs() + self.point.2.abs()) as usize
    }

    pub fn kinetic_energy(&self) -> usize {
        (self.velocity.0.abs() + self.velocity.1.abs() + self.velocity.2.abs()) as usize
    }

    pub fn total_energy(&self) -> usize {
        self.potential_energy() * self.kinetic_energy()
    }
}

#[derive(Debug, Clone)]
pub struct Universe {
    moons: Vec<Moon>,
}

impl Universe {
    pub fn new(moons: Vec<Moon>) -> Universe {
        Universe { moons }
    }

    pub fn step(&mut self) {
        for i in 0..self.moons.len() {
            for j in i..self.moons.len() {
                let (a, b) = Self::gravity(&self.moons[i], &self.moons[j]);
                self.moons[i].update_velocity(a);
                self.moons[j].update_velocity(b);
            }
        }
        for moon in self.moons.iter_mut() {
            moon.apply_velocity();
        }
    }

    fn gravity(x: &Moon, y: &Moon) -> ((isize, isize, isize), (isize, isize, isize)) {
        let s0 = Self::pull(x.point.0, y.point.0);
        let s1 = Self::pull(x.point.1, y.point.1);
        let s2 = Self::pull(x.point.2, y.point.2);
        ((s0.0, s1.0, s2.0), (s0.1, s1.1, s2.1))
    }

    fn pull(a: isize, b: isize) -> (isize, isize) {
        match a.cmp(&b) {
            Ordering::Equal => (0, 0),
            Ordering::Less => (1, -1),
            Ordering::Greater => (-1, 1),
        }
    }

    pub fn moons(&self) -> Vec<Moon> {
        self.moons.clone()
    }
}

pub struct Parser {}

impl Parser {
    pub fn parse(inp: &str) -> Moon {
        let mut point = Point(0, 0, 0);
        scan!(inp.bytes() => "<x={}, y={}, z={}>", point.0, point.1, point.2);
        Moon::new(point)
    }
}

#[cfg(test)]
mod tests {
    use super::{Moon, Parser, Point, Universe, Velocity};
    use std::io::{BufRead, Cursor};

    fn universe(inp: &str) -> Universe {
        Universe::new(
            Cursor::new(inp)
                .lines()
                .map(|r| Parser::parse(&r.unwrap()))
                .collect(),
        )
    }

    #[test]
    fn ten_steps() {
        let inp = "
<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>
";
        let mut u = universe(inp.trim());
        assert_eq!(
            u.moons(),
            vec![
                Moon {
                    point: Point(-1, 0, 2),
                    velocity: Velocity(0, 0, 0)
                },
                Moon {
                    point: Point(2, -10, -7),
                    velocity: Velocity(0, 0, 0)
                },
                Moon {
                    point: Point(4, -8, 8),
                    velocity: Velocity(0, 0, 0)
                },
                Moon {
                    point: Point(3, 5, -1),
                    velocity: Velocity(0, 0, 0)
                },
            ]
        );
        u.step();
        assert_eq!(
            u.moons(),
            vec![
                Moon {
                    point: Point(2, -1, 1),
                    velocity: Velocity(3, -1, -1)
                },
                Moon {
                    point: Point(3, -7, -4),
                    velocity: Velocity(1, 3, 3)
                },
                Moon {
                    point: Point(1, -7, 5),
                    velocity: Velocity(-3, 1, -3)
                },
                Moon {
                    point: Point(2, 2, 0),
                    velocity: Velocity(-1, -3, 1)
                },
            ]
        );
        for _ in 0..9 {
            u.step();
        }
        assert_eq!(
            u.moons(),
            vec![
                Moon {
                    point: Point(2, 1, -3),
                    velocity: Velocity(-3, -2, 1)
                },
                Moon {
                    point: Point(1, -8, 0),
                    velocity: Velocity(-1, 1, 3)
                },
                Moon {
                    point: Point(3, -6, 1),
                    velocity: Velocity(3, 2, -3)
                },
                Moon {
                    point: Point(2, 0, 4),
                    velocity: Velocity(1, -1, -1)
                },
            ]
        );
        assert_eq!(
            u.moons().iter().map(|m| m.total_energy()).sum::<usize>(),
            179
        );
    }
}
