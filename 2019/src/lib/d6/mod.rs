use std::collections::{HashMap, VecDeque};

pub struct Graph {
    orbits: HashMap<String, String>,
}

impl Graph {
    pub fn new(orbits: HashMap<String, String>) -> Self {
        Graph { orbits }
    }

    pub fn traverse(&self) -> HashMap<String, usize> {
        let mut map = HashMap::new();
        // The center of mass orbits nothing.
        map.insert("COM".to_string(), 0);

        for k in self.orbits.keys() {
            self.count(&mut map, &k);
        }
        map
    }

    fn count(&self, m: &mut HashMap<String, usize>, s: &str) -> usize {
        if let Some(x) = m.get(s) {
            return *x;
        }
        let n = self.count(m, &self.orbits[s]);
        m.insert(s.to_string(), n + 1);
        m[s]
    }

    pub fn common_ancestor(&self, a: &str, b: &str) -> String {
        let a_path = self.path(a);
        let b_path = self.path(b);
        let mut last = "COM";
        for (ai, bi) in a_path.iter().zip(b_path.iter()) {
            if ai != bi {
                return last.to_string();
            }
            last = ai;
        }
        last.to_string()
    }

    fn path(&self, obj: &str) -> VecDeque<String> {
        let mut v = VecDeque::new();
        let mut this = obj;
        while let Some(cur) = self.orbits.get(this) {
            v.push_front(cur.to_string());
            this = cur;
        }
        v
    }

    pub fn transfers_between(&self, a: &str, b: &str) -> usize {
        let map = self.traverse();
        let ancestor = self.common_ancestor(a, b);
        map[a] + map[b] - 2 * map[&ancestor]
    }
}

pub struct Parser {}

impl Parser {
    pub fn parse(inp: &[String]) -> HashMap<String, String> {
        inp.iter()
            .map(|s| {
                let v: Vec<_> = s.split(')').collect();
                (v[1].to_string(), v[0].to_string())
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::{Graph, Parser};
    use std::io;
    use std::io::BufRead;

    fn data_p1() -> &'static str {
        "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L"
    }

    fn data_p2() -> &'static str {
        "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN"
    }

    fn graph(data: &str) -> Graph {
        let v = io::Cursor::new(data)
            .lines()
            .map(|r| r.unwrap())
            .collect::<Vec<_>>();
        let orbits = Parser::parse(&v);
        Graph::new(orbits)
    }

    #[test]
    fn validate_p1() {
        let g = graph(data_p1()).traverse();
        assert_eq!(g["D"], 3);
        assert_eq!(g["L"], 7);
        assert_eq!(g["COM"], 0);
        assert_eq!(g.iter().map(|(_, v)| *v).sum::<usize>(), 42);
    }

    #[test]
    fn common_ancestor() {
        let g = graph(data_p2());
        assert_eq!(g.common_ancestor("YOU", "SAN"), "D");
        assert_eq!(g.common_ancestor("K", "I"), "D");
        assert_eq!(g.common_ancestor("YOU", "L"), "K");
        assert_eq!(g.common_ancestor("G", "D"), "B");
    }

    #[test]
    fn transfers_between() {
        let g = graph(data_p2());
        assert_eq!(g.transfers_between("YOU", "SAN"), 6);
        assert_eq!(g.transfers_between("K", "I"), 4);
        assert_eq!(g.transfers_between("YOU", "L"), 2);
        assert_eq!(g.transfers_between("G", "D"), 3);
    }
}
