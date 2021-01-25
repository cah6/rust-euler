use std::ops::Range;

// Fib iterator
pub struct Fibonacci {
    prev: u32,
    curr: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let new_curr = self.prev + self.curr;

        self.prev = self.curr;
        self.curr = new_curr;

        Some(self.curr)
    }
}

pub fn fibonacci() -> Fibonacci {
    Fibonacci { prev: 0, curr: 1 }
}

// Primes iterator
pub struct Primes {
    current: u64,
    possible: Vec<u64>
}

pub fn primes(up_to: u64) -> Primes {
    Primes {
        current: 1,
        possible: (1..up_to).collect()
    }
}

impl Iterator for Primes {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        self.current += 1;

        let curr = self.current.clone();
        self.possible.retain(|&x|x % curr != 0);

        let index = self.current as usize - 1;
        self.possible.get(index).cloned()
    }
}

//
pub struct Triangular {
    curr: u64,
    n: u64
}

pub fn triangular() -> Triangular {
    Triangular {
        curr: 0,
        n: 0
    }
}

impl Iterator for Triangular {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        self.n += 1;
        self.curr += self.n;
        Some(self.curr)
    }
}

