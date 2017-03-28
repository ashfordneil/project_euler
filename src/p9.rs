//! A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,

//! a^2 + b^2 = c^2
//! For example, 3^2 + 4^2 = 9 + 16 = 25 = 5^2.

//! There exists exactly one Pythagorean triplet for which a + b + c = 1000.
//! Find the product abc.
extern crate itertools;
use itertools::Itertools;

fn main() {
    let (x, y) = (1..500 as u32)
        .cartesian_product(1..500)
        .filter(|&(x, y)| x > y)
        .filter(|&(x, y)| x.pow(2) + y.pow(2) == (1000 - x - y).pow(2))
        .next()
        .unwrap();
    let answer = x * y * (1000 - x - y);
    println!("{:?}", answer);
}
