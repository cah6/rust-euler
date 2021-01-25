use std::iter::Iterator;

pub fn solve(n : i32) -> i32 {
    (0..n).filter(|x|x % 3 == 0 || x % 5 == 0)
        .fold(0, |acc, x| acc + x)
}

#[cfg(test)]
mod tests {
    use crate::p1::*;

    #[test]
    fn test() {
        assert_eq!(solve(10), 23);
        assert_eq!(solve(1000), 233168);
    }
}
