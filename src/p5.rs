//! 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any
//! remainder.

//! What is the smallest positive number that is evenly divisible by all of the numbers from 1 to
//! 20?

fn main() {
    let mut numbers: Vec<_> = (1..20).collect();
    let mut output = 1;
    for divisor in 2.. {
        while numbers.iter().any(|number| number % divisor == 0) {
            output *= divisor;
            for number in &mut numbers {
                if *number % divisor == 0 {
                    *number /= divisor;
                }
            }
        }
        numbers.retain(|&x| x != 1);
        if numbers.len() == 0 {
            break;
        }
    }
    println!("Answer: {}", output);
}
