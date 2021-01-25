use std::iter::Iterator;

use crate::common::*;

pub fn solve(n: u64) -> u64 {
    triangular().find(|&x|num_divisor(x) > n).unwrap()
}

fn num_divisor(n : u64) -> u64 {
    let mut ret = 1; // div by itself
    for i in 1..=(n / 2) {
        if n % i == 0 {
            ret += 1;
        }
    }
    ret
}

#[cfg(test)]
mod tests {
    use crate::p12::*;

    #[test]
    fn test() {
        assert_eq!(solve(5), 28);
        assert_eq!(solve(500), 28);
    }
}
