use std::collections::HashMap;

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
    use std::collections::HashMap;
    use std::io;
    use std::io::BufRead;

    fn traverse() -> HashMap<String, usize> {
        let data = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L";
        let v = io::Cursor::new(data).lines().map(|r| r.unwrap()).collect::<Vec<_>>();
        let orbits = Parser::parse(&v);
        Graph::new(orbits).traverse()
    }

    #[test]
    fn validate_p1() {
        let g = traverse();
        assert_eq!(g["D"], 3);
        assert_eq!(g["L"], 7);
        assert_eq!(g["COM"], 0);
        assert_eq!(g.iter().map(|(_, v)| *v).sum::<usize>(), 42);
    }
}
