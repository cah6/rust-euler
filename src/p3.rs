use crate::common::*;

pub fn solve(n : u64) -> u64 {
    let mut curr = n;
    let max_prime = (n as f64).sqrt().ceil() as u64;
    let mut factors = Vec::new();
    for i in primes(max_prime) {
        if curr == 1 {
            break;
        }

        if curr % i == 0 {
            curr = curr / i;
            factors.push(i);
        }
    }
    factors.last().unwrap_or(&n).clone()
}

#[cfg(test)]
mod tests {
    use crate::p3::*;

    #[test]
    fn p1_test() {
        assert_eq!(solve(13_195), 29);
        assert_eq!(solve(600_851_475_143), 6857);
    }
}
