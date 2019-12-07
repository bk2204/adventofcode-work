use std::collections::BTreeMap;

pub struct Filter {}

impl Filter {
    pub fn validate_p1(n: usize) -> bool {
        let s = format!("{}", n);
        // Is this a six-digit number?
        if s.len() != 6 {
            return false;
        }
        // Is there at least one consecutive pair of digits?
        if !s
            .chars()
            .fold(('0', false), |(last, pair), c| (c, c == last || pair))
            .1
        {
            return false;
        }
        // Do the digits never decrease?
        if !s
            .chars()
            .fold(('0', true), |(last, acc), c| (c, c >= last && acc))
            .1
        {
            return false;
        }
        true
    }

    pub fn validate_p2(n: usize) -> bool {
        let s = format!("{}", n);
        // Is this a six-digit number?
        if s.len() != 6 {
            return false;
        }
        // Do the digits never decrease?
        if !s
            .chars()
            .fold((' ', true), |(last, acc), c| (c, c >= last && acc))
            .1
        {
            return false;
        }
        // Is there at least one consecutive pair of digits that isn't part of a triplet?
        let is_pair = !s
            .chars()
            .fold(BTreeMap::<char, usize>::new(), |mut map, c| {
                if let Some(v) = map.get_mut(&c) {
                    *v += 1;
                } else {
                    map.insert(c, 1);
                };
                map
            })
            .iter()
            .any(|(_, &v)| v == 2);
        if is_pair {
            return false;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::Filter;

    #[test]
    fn validate_p1() {
        assert_eq!(Filter::validate_p1(111111), true);
        assert_eq!(Filter::validate_p1(223450), false);
        assert_eq!(Filter::validate_p1(123789), false);
        assert_eq!(Filter::validate_p1(123788), true);
    }

    #[test]
    fn validate_p2() {
        assert_eq!(Filter::validate_p2(111111), false);
        assert_eq!(Filter::validate_p2(112233), true);
        assert_eq!(Filter::validate_p2(123444), false);
        assert_eq!(Filter::validate_p2(111122), true);
        assert_eq!(Filter::validate_p2(111223), true);
    }
}
