use regex::Regex;
use std::{cmp, str};

pub fn rec_mul(left: &str, right: &str) -> String {
    // strictly start and end with a digit, i.e. one or more digits would match
    let regex = Regex::new(r"^\d\d*$").unwrap();
    assert!(
        regex.is_match(left),
        "First (left) number either negative or contains non-digit characters: {}",
        left
    );
    assert!(
        regex.is_match(right),
        "Second (right) number either negative or contains non-digit characters: {}",
        right
    );

    // base case
    if left.len() == 1 && right.len() == 1 {
        // we can use u8 as only 1 digit 0-9 is supported by this arm
        let left = left.parse::<u8>().unwrap();
        let right = right.parse::<u8>().unwrap();
        return (left * right).to_string();
    }

    let left_pow = (left.len() as f64).log2().ceil() as usize;
    let right_pow = (right.len() as f64).log2().ceil() as usize;
    let max_pow = cmp::max(left_pow, right_pow);
    let digits = 2usize.pow(max_pow as u32);

    let mut padded_left = left.to_string();
    if left.len() < digits {
        padded_left = prefix_with_zeroes(left, digits - left.len());
    }
    let mut padded_right = right.to_string();
    if right.len() < digits {
        padded_right = prefix_with_zeroes(right, digits - right.len());
    }

    let n = padded_left.len();
    let half = n / 2;

    let (a, b) = padded_left.split_at(half);
    let (c, d) = padded_right.split_at(half);

    let ac = rec_mul(a, c);
    let ad = rec_mul(a, d);
    let bc = rec_mul(b, c);
    let bd = rec_mul(b, d);

    // 10^n * ac
    let x = to_the_power_of_10(&ac, n);
    // 10^(n/2) * (ad + bc)
    let y = to_the_power_of_10(&addition(ad, bc), n / 2);
    // 10^n * ac + 10^(n/2) * (ad + bc) + bd
    let result = addition(addition(x, y), bd);
    // unpad zeroes if needed for cases like "000000" or "000001"
    let final_result = result.trim_start_matches('0').to_string();
    if final_result.is_empty() {
        // if the result was all zeroes, we return a single zero
        return String::from("0");
    }
    final_result
}

fn prefix_with_zeroes(a: &str, zeroes: usize) -> String {
    format!("{}{}", init_zeroes(zeroes), a)
}

fn to_the_power_of_10(a: &str, power: usize) -> String {
    format!("{}{}", a, init_zeroes(power))
}

fn init_zeroes(num: usize) -> String {
    let zeroes = vec![b'0'; num];
    str::from_utf8(&zeroes).unwrap().to_string()
}

// got from https://medium.com/@jeanjacques.strydom34/summing-really-big-numbers-in-rust-d649e3f92a2e
// as this wasn't main task in the course
fn addition(num1: String, num2: String) -> String {
    let mut additive_string = String::new();
    let mut overflow = 0;
    let mut number1: Vec<char> = num1.chars().collect();
    let mut number2: Vec<char> = num2.chars().collect();

    // Get the length of the longest number.
    let longest_len = number1.len().max(number2.len());

    // Do simple primary school addition with each value in the number.
    for _ in 0..longest_len {
        // Convert the first number to u8.
        let first: u8 = match number1.pop() {
            Some(character) => character.to_string().parse().unwrap(),
            None => 0,
        };

        // Convert the second number to u8.
        let second: u8 = match number2.pop() {
            Some(character) => character.to_string().parse().unwrap(),
            None => 0,
        };

        // Add the two numbers together and turn it into a string.
        let mut sum: String = (first + second + overflow)
            .to_string()
            .chars()
            .rev()
            .collect();

        // If the string has more than 1 number in it, it means we have to pass
        // the overflow value onto the next round of addition.
        overflow = if sum.len() > 1 {
            sum.pop().unwrap().to_string().parse().unwrap()
        } else {
            // Set the overflow back to 0 just in case.
            0
        };

        // Add the sum of the two values to the string.
        additive_string.push(*sum.chars().collect::<Vec<char>>().first().unwrap());
    }

    // Finally if there was a leftover overflow value, add it to the string.
    if overflow > 0 {
        additive_string.push(
            *overflow
                .to_string()
                .chars()
                .collect::<Vec<char>>()
                .first()
                .unwrap(),
        );
    }

    // Reverse the order of the string and return it.
    additive_string.chars().rev().collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_rec_mul_panic_nan() {
        let result = rec_mul("10", "ab");
    }

    #[test]
    #[should_panic]
    fn test_rec_mul_panic_neg() {
        let result = rec_mul("-1000", "1000");
    }

    #[test]
    fn test_rec_mul() {
        let result = rec_mul("1", "7");
        assert_eq!(result, "7");

        let result = rec_mul("1", "0");
        assert_eq!(result, "0");

        let result = rec_mul("1234", "0");
        assert_eq!(result, "0");

        let result = rec_mul("1234", "78910");
        assert_eq!(result, "97374940");

        let result = rec_mul("10", "78");
        assert_eq!(result, "780");

        let result = rec_mul("9988", "1000");
        assert_eq!(result, "9988000");

        let result = rec_mul("33778822", "18293826");
        assert_eq!(result, "617943892152972");

        // let result = rec_mul(
        //     "3141592653589793238462643383279502884197169399375105820974944592",
        //     "2718281828459045235360287471352662497757247093699959574966967627",
        // );
        // assert_eq!(result, "8539734222673567065463550869546574495034888535765114961879601127067743044893204848617875072216249073013374895871952806582723184");
    }
}
