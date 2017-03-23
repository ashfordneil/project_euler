//! The sum of the squares of the first ten natural numbers is,
//! 1^2 + 2^2 + ... + 10^2 = 385

//! The square of the sum of the first ten natural numbers is,
//! (1 + 2 + ... + 10)^2 = 552 = 3025

//! Hence the difference between the sum of the squares of the first ten natural numbers and the
//! square of the sum is 3025 − 385 = 2640.

//! Find the difference between the sum of the squares of the first one hundred natural numbers and
//! the square of the sum.

#![feature(inclusive_range_syntax)]
use std::ops::Add;

fn main() {
    let sum_of_squares = (1...100).map(|x| x * x).fold(0, Add::add);
    let square_of_sums = (1...100).fold(0 as u32, Add::add).pow(2);
    let answer = square_of_sums - sum_of_squares;
    println!("Answer: {}", answer);
}
