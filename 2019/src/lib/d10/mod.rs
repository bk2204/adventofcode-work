use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::hash::{Hash, Hasher};

#[derive(Debug)]
struct ComparableFloat(f64);

impl PartialEq for ComparableFloat {
    fn eq(&self, other: &Self) -> bool {
        (self.0 - other.0).abs() < 0.000_000_1
    }
}

impl Eq for ComparableFloat {}

impl Ord for ComparableFloat {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.eq(other) {
            Ordering::Equal
        } else if self.0 < other.0 {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}

impl PartialOrd for ComparableFloat {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

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

    fn polar_to(&self, other: &Point) -> PolarCoordinates {
        let pi = 4.0 * 1.0f64.atan();
        let dx = other.0 as isize - self.0 as isize;
        let dy = self.1 as isize - other.1 as isize;
        let dsq = (dx * dx + dy * dy) as usize;
        let val = (dy as f64 / dx as f64).atan();
        let theta = match (dy.signum(), dx.signum()) {
            (0, 1) => 0.0,
            (0, 0) => unreachable!(),
            (0, -1) => pi,
            (1, 1) => val,
            (1, 0) => pi / 2.0,
            (1, -1) => val + pi,
            (-1, 1) => val + 2.0 * pi,
            (-1, 0) => 3.0 * pi / 2.0,
            (-1, -1) => val + pi,
            _ => unreachable!(),
        };
        PolarCoordinates { dsq, theta }
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
                ComparableFloat(self.m) == ComparableFloat(other.m)
                    && ComparableFloat(self.b) == ComparableFloat(other.b)
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

// This is a slightly modified version of polar coordinates in that we use the
// distance squared instead of the distance.  This simplifies things since floats
// are not comparable but integers are.
#[derive(Copy, Clone, Debug)]
struct PolarCoordinates {
    dsq: usize,
    theta: f64,
}

impl PolarCoordinates {
    // We adjust theta for sorting such that the value will end up always positive and increasing
    // in the clockwise direction, starting at pi/2 (pointing directly up).
    fn adjusted_theta(&self) -> f64 {
        let pi = 4.0 * 1.0f64.atan();
        let circ = 2.0 * pi;
        (circ - (self.theta - (pi / 2.0))) % circ
    }
}

impl PartialEq for PolarCoordinates {
    fn eq(&self, other: &Self) -> bool {
        if self.dsq != other.dsq {
            return false;
        }
        ComparableFloat(self.theta) == ComparableFloat(other.theta)
    }
}

impl Eq for PolarCoordinates {}

impl Ord for PolarCoordinates {
    fn cmp(&self, other: &Self) -> Ordering {
        match ComparableFloat(self.adjusted_theta()).cmp(&ComparableFloat(other.adjusted_theta())) {
            Ordering::Equal => self.dsq.cmp(&other.dsq),
            x => x,
        }
    }
}

impl PartialOrd for PolarCoordinates {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
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

    pub fn destroyed_from(&self, point: &Point) -> Vec<Point> {
        // We partition the items into groups based on their direction (line, comparison) from the
        // point and then iterate over them in polar coordinate order one at a time.
        let v: Vec<_> = self
            .points
            .iter()
            .filter(|q| *q != point)
            .map(|q| ((point.line(q), point.cmp(q)), point.polar_to(q), *q))
            .collect();

        let mut thetagrouped: BTreeMap<_, Vec<(PolarCoordinates, Point)>> = BTreeMap::new();
        for (_, polar, point) in &v {
            let atheta = ComparableFloat(polar.adjusted_theta());
            if let Some(v) = thetagrouped.get_mut(&atheta) {
                v.push((*polar, *point));
            } else {
                thetagrouped.insert(atheta, vec![(*polar, *point)]);
            }
        }
        for v in thetagrouped.values_mut() {
            v.sort();
        }
        let mut seen = HashSet::new();
        let mut r = Vec::new();

        // This isn't especially efficient, but n is small.
        while r.len() < v.len() {
            for seq in thetagrouped.values() {
                if let Some(point) = seq
                    .iter()
                    .sorted()
                    .map(|(_, p)| p)
                    .filter(|p| !seen.contains(p))
                    .nth(0)
                {
                    seen.insert(point);
                    r.push(*point);
                }
            }
        }
        r
    }
}

#[cfg(test)]
mod tests {
    use super::{ComparableFloat, Line, Map, Point, PolarCoordinates};
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

    fn destroyed_from(s: &str, p: Point) -> Vec<Point> {
        map(s).destroyed_from(&p)
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
    fn polar() {
        let pi = 4.0 * 1.0f64.atan();

        assert_eq!(
            ComparableFloat(
                PolarCoordinates {
                    dsq: 1,
                    theta: 3.0 * pi / 2.0
                }
                .adjusted_theta()
            ),
            ComparableFloat(pi)
        );

        assert_eq!(
            ComparableFloat(Point(0, 0).polar_to(&Point(0, 1)).theta),
            ComparableFloat(3.0 * pi / 2.0)
        );

        assert_eq!(
            ComparableFloat(Point(0, 1).polar_to(&Point(0, 0)).theta),
            ComparableFloat(pi / 2.0)
        );

        assert_eq!(
            ComparableFloat(Point(0, 0).polar_to(&Point(1, 0)).theta),
            ComparableFloat(0.0)
        );

        assert_eq!(
            ComparableFloat(Point(1, 0).polar_to(&Point(0, 0)).theta),
            ComparableFloat(pi)
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
        let destroyed = destroyed_from(map, Point(11, 13));
        assert_eq!(destroyed.len(), 299);
        assert_eq!(destroyed[0], Point(11, 12));
        assert_eq!(destroyed[1], Point(12, 1));
        assert_eq!(destroyed[2], Point(12, 2));
        assert_eq!(destroyed[9], Point(12, 8));
        assert_eq!(destroyed[19], Point(16, 0));
        assert_eq!(destroyed[49], Point(16, 9));
        assert_eq!(destroyed[99], Point(10, 16));
        assert_eq!(destroyed[198], Point(9, 6));
        assert_eq!(destroyed[199], Point(8, 2));
        assert_eq!(destroyed[200], Point(10, 9));
        assert_eq!(destroyed[298], Point(11, 1));
    }
}
