extern crate adventofcode;
use adventofcode::d10::{Map, Point};
use std::io;
use std::io::BufRead;

fn best_point(map: &Map) -> (Point, usize) {
    map.visible_from()
        .iter()
        .fold((Point(0, 0), 0), |(p, pcount), (&q, &qcount)| {
            if qcount > pcount {
                (q, qcount)
            } else {
                (p, pcount)
            }
        })
}

fn destroyed_from(map: &Map, point: &Point) -> Vec<Point> {
    map.destroyed_from(point)
}

fn main() -> io::Result<()> {
    let b = io::BufReader::new(io::stdin());
    let map = Map::new(b.lines().map(|r| r.unwrap()));
    let (point, sz) = best_point(&map);
    println!("{:?}", (point, sz));
    println!("{:?}", destroyed_from(&map, &point)[199]);
    Ok(())
}
