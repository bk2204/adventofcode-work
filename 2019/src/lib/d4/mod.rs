pub struct Filter {}

impl Filter {
    pub fn validate(n: usize) -> bool {
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
}

#[cfg(test)]
mod tests {
    use super::Filter;

    #[test]
    fn validate() {
        assert_eq!(Filter::validate(111111), true);
        assert_eq!(Filter::validate(223450), false);
        assert_eq!(Filter::validate(123789), false);
        assert_eq!(Filter::validate(123788), true);
    }
}
