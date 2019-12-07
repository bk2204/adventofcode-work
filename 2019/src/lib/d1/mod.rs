use std::io;
use std::io::BufRead;

pub fn fuel_for_module(mass: u64) -> u64 {
    (mass / 3).saturating_sub(2)
}

pub fn fuel_for_module_recursive(mass: u64) -> u64 {
    let mut total = 0;
    let mut cur = mass;
    while cur > 0 {
        cur = fuel_for_module(cur);
        total += cur;
    }
    total
}

pub fn parse_file(f: Box<dyn io::BufRead>) -> Vec<u64> {
    f.lines()
        .map(|l| u64::from_str_radix(&l.unwrap(), 10).unwrap())
        .collect()
}

pub fn total_fuel(v: &[u64], op: &dyn Fn(u64) -> u64) -> u64 {
    v.iter().map(|mass| op(*mass)).sum()
}

#[cfg(test)]
mod tests {
    use super::{fuel_for_module, fuel_for_module_recursive, parse_file, total_fuel};
    use std::io::Cursor;

    #[test]
    fn known_values() {
        assert_eq!(fuel_for_module(12), 2);
        assert_eq!(fuel_for_module(14), 2);
        assert_eq!(fuel_for_module(1969), 654);
        assert_eq!(fuel_for_module(100756), 33583);
    }

    #[test]
    fn known_values_recursive() {
        assert_eq!(fuel_for_module_recursive(12), 2);
        assert_eq!(fuel_for_module_recursive(14), 2);
        assert_eq!(fuel_for_module_recursive(1969), 966);
        assert_eq!(fuel_for_module_recursive(100756), 50346);
    }

    #[test]
    fn total_value() {
        let input = b"12\n14\n1969\n100756\n";

        assert_eq!(
            total_fuel(&parse_file(Box::new(Cursor::new(input))), &fuel_for_module),
            2 + 2 + 654 + 33583
        );
    }
}
