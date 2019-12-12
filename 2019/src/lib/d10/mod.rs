use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Point(pub usize, pub usize);

impl Point {
    fn line(&self, other: &Point) -> Line {
        let m = (other.1 as f64 - self.1 as f64) / (other.0 as f64 - self.0 as f64);
        let b = self.1 as f64 - (self.0 as f64 * m);
        Line {
            m,
            b,
            x: if other.0 == self.0 {
                Some(self.0)
            } else {
                None
            },
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Line {
    m: f64,
    b: f64,
    x: Option<usize>,
}

impl PartialEq for Line {
    fn eq(&self, other: &Self) -> bool {
        match (self.x, other.x) {
            (Some(a), Some(b)) => a == b,
            (Some(_), None) => false,
            (None, Some(_)) => false,
            (None, None) => {
                (self.m - other.m).abs() < 0.000_000_1 && (self.b - other.b).abs() < 0.000_000_1
            }
        }
    }
}

impl Eq for Line {}

impl Hash for Line {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        match self.x {
            Some(_) => (),
            None => {
                self.m.to_bits().hash(state);
                self.b.to_bits().hash(state);
            }
        }
    }
}

pub struct Map {
    points: Vec<Point>,
}

impl Map {
    pub fn new<F: Iterator<Item = String>>(iter: F) -> Self {
        Map {
            points: Self::parse(iter),
        }
    }

    fn parse<F: Iterator<Item = String>>(iter: F) -> Vec<Point> {
        iter.enumerate()
            .flat_map(|(j, s)| {
                s.chars()
                    .enumerate()
                    .flat_map(|(i, c)| if c == '#' { vec![Point(i, j)] } else { vec![] })
                    .collect::<Vec<_>>()
            })
            .collect()
    }

    pub fn visible_from(&self) -> HashMap<Point, usize> {
        // The algorithm is as follows.  We compare each point with each other point.  If the
        // second point is on the same line and in the same direction as another point, then only
        // one of those points is visible.  We therefore use the pair (line, ordering) as a
        // unique key to de-duplicate to find the number of other points visible from this one.
        self.points
            .iter()
            .map(|p| {
                (
                    *p,
                    self.points
                        .iter()
                        .filter(|q| *q != p)
                        .map(|q| (p.line(q), p.cmp(q)))
                        .collect::<HashSet<_>>()
                        .len(),
                )
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::{Line, Map, Point};
    use std::f64::INFINITY;
    use std::io;
    use std::io::BufRead;

    fn map(s: &str) -> Map {
        Map::new(io::Cursor::new(s.trim()).lines().map(|r| r.unwrap()))
    }

    fn best_point(s: &str) -> (Point, usize) {
        map(s)
            .visible_from()
            .iter()
            .fold((Point(0, 0), 0), |(p, pcount), (&q, &qcount)| {
                if qcount > pcount {
                    (q, qcount)
                } else {
                    (p, pcount)
                }
            })
    }

    #[test]
    fn lines() {
        assert_eq!(
            Point(0, 0).line(&Point(1, 1)),
            Line {
                m: 1.0,
                b: 0.0,
                x: None
            }
        );
        assert_eq!(
            Point(0, 1).line(&Point(1, 3)),
            Line {
                m: 2.0,
                b: 1.0,
                x: None
            }
        );
        assert_eq!(
            Point(1, 10).line(&Point(3, 17)),
            Line {
                m: 3.5,
                b: 6.5,
                x: None
            }
        );
        // The specific infinities don't matter here because they won't be compared.
        assert_eq!(
            Point(1, 10).line(&Point(1, 20)),
            Line {
                m: INFINITY,
                b: INFINITY,
                x: Some(1)
            }
        );
    }

    #[test]
    fn examples() {
        let map = "
......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####
";
        assert_eq!(best_point(map), (Point(5, 8), 33));

        let map = "
#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###.
";
        assert_eq!(best_point(map), (Point(1, 2), 35));

        let map = "
.#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#..
";
        assert_eq!(best_point(map), (Point(6, 3), 41));

        let map = "
.#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##
";
        assert_eq!(best_point(map), (Point(11, 13), 210));
    }
}
