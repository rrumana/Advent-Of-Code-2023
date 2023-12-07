extern crate lazy_static;

use lazy_static::lazy_static;
use std::{fs, collections::HashMap};

lazy_static! {
    static ref WORDS: HashMap<String, char> = HashMap::from([
        ("one".to_string(), '1'),
        ("two".to_string(), '2'),
        ("three".to_string(), '3'),
        ("four".to_string(), '4'),
        ("five".to_string(), '5'),
        ("six".to_string(), '6'),
        ("seven".to_string(), '7'),
        ("eight".to_string(), '8'),
        ("nine".to_string(), '9'),
    ]);
}

fn check_for_word(input: &str, i: usize) -> Option<char> {
    for word in WORDS.keys() {
        let substr = match input.get(i..i+word.len()).ok_or("Out of bounds") {
            Ok(str)=> str,
            _ => {continue;}
        };

        if substr == word {
            return WORDS.get(word).copied();
        }
    }

    None
}

fn first_number(input: &str) -> Option<char> {
    let mut i = 0;

    for c in input.chars() {
        if c.is_digit(10) {
            return Some(c);
        }

        match check_for_word(input, i) {
            Some(digit) => { return Some(digit); },
            None        => { i += 1; }
        }
    }

    None
}

fn last_number(input: &str) -> Option<char> {
    let mut i = 0;

    for c in input.chars().rev() {
        if c.is_digit(10) {
            return Some(c);
        }

        match check_for_word(input, input.len() - i - 1) {
            Some(digit) => { return Some(digit); },
            None        => { i += 1; }
        }
    }

    None
}

fn calculate_sum(filepath: &str) -> i32 {
    let contents = fs::read_to_string(filepath).expect("Something went wrong reading the file");
    let lines = contents.split("\n");

    let mut sum: i32 = 0;
    for line in lines {
        let first = first_number(line).expect("No first digit found.");
        let last = last_number(line).expect("No last digit found.");
        let str_num = format!("{}{}", first, last);
        sum += str_num.parse::<i32>().expect("Selected chars do not parse to i32");
    }

    return sum;
}

fn main() {
    let filepath = "real_input.txt";
    let sum = calculate_sum(filepath);
        println!("The sum is: {}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() {
        assert_eq!(first_number(&"1abc2").unwrap(), '1');
        assert_eq!(first_number(&"pqr3stu8vwx").unwrap(), '3');
        assert_eq!(first_number(&"ab12cd34ef").unwrap(), '1');
        assert_eq!(first_number(&"treb7uchet").unwrap(), '7');
    }

    #[test]
    fn test_last() {
        assert_eq!(last_number(&"1abc2").unwrap(), '2');
        assert_eq!(last_number(&"pqr3stu8vwx").unwrap(), '8');
        assert_eq!(last_number(&"ab12cd34ef").unwrap(), '4');
        assert_eq!(last_number(&"treb7uchet").unwrap(), '7');
    }

    #[test]
    fn test_calc_one() {
        let filepath = "part_one_test_input.txt";
        assert_eq!(calculate_sum(filepath), 142)
    }

    #[test]
    fn test_calc_two() {
        let filepath = "part_two_test_input.txt";
        assert_eq!(calculate_sum(filepath), 281)
    }
}