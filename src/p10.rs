//! The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.

//! Find the sum of all the primes below two million.

// stolen from p7
/// An iteartor that generates sequential prime numbers.
struct PrimeGenerator {
    found: Vec<u64>,
}

impl Iterator for PrimeGenerator {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let prime = match self.found.last() {
            // base case
            None => 2,
            Some(start) => {
                (*start..)
                    .filter(|&number| {
                        self.found
                            .iter()
                            .take_while(|&prime| prime * prime < number)
                            .all(|divisor| number % divisor > 0)
                    })
                    .next()
                    .unwrap()
            }
        };
        self.found.push(prime);
        Some(prime)
    }
}

fn main() {
    let answer: u64 = PrimeGenerator { found: Vec::new() }.take_while(|&x| x < 2_000_000).sum();
    println!("Answer: {}", answer);
}
