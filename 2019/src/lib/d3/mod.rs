use std::cmp;
use std::collections::BTreeMap;
use std::ops::{Index, IndexMut};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct Point(pub i64, pub i64);

#[derive(Clone)]
struct Row {
    v: Vec<u8>,
    off: isize,
}

impl Row {
    fn new(size: usize) -> Self {
        Row {
            v: vec![0u8; size * 2 + 1],
            off: size as isize,
        }
    }
}

impl Index<isize> for Row {
    type Output = u8;

    fn index(&self, idx: isize) -> &u8 {
        self.v.index((idx + self.off) as usize)
    }
}

impl IndexMut<isize> for Row {
    fn index_mut(&mut self, idx: isize) -> &mut u8 {
        self.v.index_mut((idx + self.off) as usize)
    }
}

impl IntoIterator for Row {
    type Item = u8;
    type IntoIter = ::std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.v.into_iter()
    }
}

struct Grid {
    v: Vec<Row>,
    off: isize,
}

impl Grid {
    fn new(size: usize) -> Self {
        Grid {
            v: vec![Row::new(size); size * 2 + 1],
            off: size as isize,
        }
    }
}

impl Index<isize> for Grid {
    type Output = Row;

    fn index(&self, idx: isize) -> &Row {
        self.v.index((idx + self.off) as usize)
    }
}

impl IndexMut<isize> for Grid {
    fn index_mut(&mut self, idx: isize) -> &mut Row {
        self.v.index_mut((idx + self.off) as usize)
    }
}

impl IntoIterator for Grid {
    type Item = Row;
    type IntoIter = ::std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.v.into_iter()
    }
}

pub struct Graph {
    paths: Vec<Vec<Segment>>,
    arr: Grid,
    intersections: BTreeMap<Point, Vec<usize>>,
}

impl Graph {
    pub fn new(data: Vec<Vec<Segment>>) -> Self {
        let max_dimension: i64 = data
            .iter()
            .map(|v| {
                v.iter()
                    .fold((Point(0, 0), 0), |(p, m), s| {
                        let q = s.point_from(p);
                        (q, cmp::max(m, cmp::max(q.0, q.1)))
                    })
                    .1
            })
            .max()
            .unwrap();
        let max_dimension = max_dimension as usize;
        if data.len() > 8 {
            panic!("can't deal with more than 8 paths");
        }
        Graph {
            paths: data,
            arr: Grid::new(max_dimension as usize),
            intersections: BTreeMap::new(),
        }
    }

    pub fn run(&mut self) {
        let arr = &mut self.arr;
        let intersections = &mut self.intersections;
        let paths = &self.paths;
        let len = self.paths.len();

        // This algorithm is two pass because it's much, much faster that way.  Storing the number
        // of steps taken for point uses so much memory that allocation dominates the time spent.
        Self::each_point(paths, |p, i, _| Self::mark(arr, intersections, len, i, p));
        Self::each_point(paths, |p, _, steps| {
            if let Some(v) = intersections.get_mut(&p) {
                v.push(steps)
            }
        });
    }

    fn each_point<F: FnMut(Point, usize, usize)>(paths: &Vec<Vec<Segment>>, mut f: F) {
        for (i, path) in paths.iter().enumerate() {
            let mut cur = Point(0, 0);
            let mut steps = 0;
            for segment in path {
                for point in segment.points_from(cur) {
                    steps += 1;
                    f(point, i, steps);
                    cur = point;
                }
            }
        }
    }

    pub fn intersections<'a>(&'a self) -> impl Iterator<Item = (Point, usize)> + 'a {
        self.intersections.iter().map(|(&p, v)| (p, v.iter().sum()))
    }

    fn mark(
        arr: &mut Grid,
        intersections: &mut BTreeMap<Point, Vec<usize>>,
        npaths: usize,
        i: usize,
        p: Point,
    ) {
        let (x, y) = (p.0 as isize, p.1 as isize);
        let mask = ((1u16 << npaths) - 1) as u8;
        arr[x][y] |= 1u8 << i;
        if arr[x][y] == mask {
            intersections.insert(p, vec![]);
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug)]
pub struct Segment {
    dir: Direction,
    len: i64,
}

impl Segment {
    fn point_from(&self, p: Point) -> Point {
        let (x, y) = self.magnitude();
        Point(p.0 + x, p.1 + y)
    }

    fn points_from(&self, p: Point) -> Box<dyn Iterator<Item = Point>> {
        let len = self.len;
        match self.dir {
            Direction::North => Box::new((p.1 + 1..=(p.1 + len)).map(move |i| Point(p.0, i))),
            Direction::South => Box::new(((p.1 - len)..p.1).rev().map(move |i| Point(p.0, i))),
            Direction::East => Box::new((p.0 + 1..=(p.0 + len)).map(move |i| Point(i, p.1))),
            Direction::West => Box::new(((p.0 - len)..p.0).rev().map(move |i| Point(i, p.1))),
        }
    }

    fn magnitude(&self) -> (i64, i64) {
        match self.dir {
            Direction::North => (0, self.len),
            Direction::South => (0, -self.len),
            Direction::East => (self.len, 0),
            Direction::West => (-self.len, 0),
        }
    }
}

pub struct Parser {}

impl Parser {
    pub fn parse(data: &str) -> Vec<Segment> {
        data.split(",").map(|s| Self::parse_one(s)).collect()
    }

    fn parse_one(seg: &str) -> Segment {
        let dir = match &seg[0..1] {
            "R" => Direction::East,
            "D" => Direction::South,
            "L" => Direction::West,
            "U" => Direction::North,
            _ => panic!("not a direction"),
        };
        Segment {
            dir,
            len: i64::from_str_radix(&seg[1..], 10).unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Graph, Parser};

    fn distance(path1: &str, path2: &str) -> i64 {
        let mut g = Graph::new(vec![Parser::parse(path1), Parser::parse(path2)]);
        g.run();
        g.intersections()
            .map(|(p, _)| p.0.abs() + p.1.abs())
            .filter(|&v| v > 0)
            .min()
            .unwrap()
    }

    fn steps(path1: &str, path2: &str) -> usize {
        let mut g = Graph::new(vec![Parser::parse(path1), Parser::parse(path2)]);
        g.run();
        g.intersections()
            .map(|(_, steps)| steps)
            .filter(|&v| v > 0)
            .min()
            .unwrap()
    }

    #[test]
    fn integration_distance() {
        assert_eq!(distance("R8,U5,L5,D3", "U7,R6,D4,L4"), 6);
        assert_eq!(
            distance(
                "R75,D30,R83,U83,L12,D49,R71,U7,L72",
                "U62,R66,U55,R34,D71,R55,D58,R83"
            ),
            159
        );
        assert_eq!(
            distance(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
                "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            ),
            135
        );
    }

    #[test]
    fn integration_steps() {
        assert_eq!(steps("R8,U5,L5,D3", "U7,R6,D4,L4"), 30);
        assert_eq!(
            steps(
                "R75,D30,R83,U83,L12,D49,R71,U7,L72",
                "U62,R66,U55,R34,D71,R55,D58,R83"
            ),
            610
        );
        assert_eq!(
            steps(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
                "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            ),
            410
        );
    }
}
