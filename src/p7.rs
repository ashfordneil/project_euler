//! By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime
//! is 13.

//! What is the 10 001st prime number?

use std::iter::Iterator;

/// An iteartor that generates sequential prime numbers.
struct PrimeGenerator {
    found: Vec<u32>,
}

impl Iterator for PrimeGenerator {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        let prime = match self.found.last() {
            // base case
            None => 2,
            Some(start) => {
                (*start..)
                    .filter(|&number| self.found.iter().all(|divisor| number % divisor > 0))
                    .next()
                    .unwrap()
            }
        };
        self.found.push(prime);
        Some(prime)
    }
}

fn main() {
    let answer = PrimeGenerator { found: Vec::new() }
        .nth(10001 - 1)
        .unwrap();
    println!("Answer: {}", answer);
}
