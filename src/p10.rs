use std::iter::Iterator;

use crate::common::*;

pub fn solve(n : u64) -> u64 {
    primes(n).map(|x|x as u64).sum()
}

#[cfg(test)]
mod tests {
    use crate::p10::*;

    #[test]
    fn test() {
        assert_eq!(solve(10), 17);
        assert_eq!(solve(2_000_000), 142913828922);
    }
}
