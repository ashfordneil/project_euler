//! The prime factors of 13195 are 5, 7, 13 and 29.

//! What is the largest prime factor of the number 600851475143 ?

fn main() {
    let mut number: u64 = 600851475143;
    for divisor in 2.. {
        while number % divisor == 0 && number != divisor {
            number /= divisor;
        }
        if divisor * divisor >= number {
            break;
        }
    }
    println!("Answer: {}", number);
}
