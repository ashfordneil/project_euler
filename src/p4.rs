//! A palindromic number reads the same both ways. The largest palindrome made from the product of
//! two 2-digit numbers is 9009 = 91 × 99.

//! Find the largest palindrome made from the product of two 3-digit numbers.

fn is_palindrome(input: &u32) -> bool {
    let string = input.to_string();
    let half = ((string.len() as f32) / 2.0).ceil() as usize;
    string[..half].chars().zip(string[half..].chars().rev()).all(|(x, y)| x == y)
}

fn main() {
    let answer = (1..1000000)
        .rev()
        .filter(is_palindrome)
        .filter(|number| {
            (100..999)
                .rev()
                .filter(|divisor| {
                    number % divisor == 0 && number / divisor < 1000 && number / divisor > 100
                })
                .next()
                .is_some()
        })
        .next()
        .unwrap();
    println!("Answer: {}", answer);
}
