
pub fn solve(n : i32) -> i32 {
    let max_len = n / 2;
    for a in 1..max_len {
        for b in 1..max_len {
            let c = n - a - b;
            if (a + b + c == n) && (a.pow(2) + b.pow(2) == c.pow(2)) {
                println!("Triple was: a={}, b={}, c={}", a, b, c);
                return a * b * c;
            }
        }
    }
    1
}

#[cfg(test)]
mod tests {
    use crate::p9::*;

    #[test]
    fn p1_test() {
        assert_eq!(solve(3 + 4 + 5), 3 * 4 * 5);
        assert_eq!(solve(1000), 31875000);
    }
}
