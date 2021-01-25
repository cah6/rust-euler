use std::iter::Iterator;

use crate::common::*;

pub fn solve(n: u32) -> u32 {
    fibonacci()
        .take_while(|&x| x < n)
        .filter(|x| x % 2 == 0)
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::p2::*;

    #[test]
    fn test() {
        assert_eq!(solve(10), 10);
        assert_eq!(solve(4_000_000), 4613732);
    }
}
