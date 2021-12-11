use std::cmp::Ordering;
use std::env;

const INCREASE: char = '+';
const DECREASE: char = '-';
const EQUAL: char = '\0';

pub fn get_input(index: usize) -> u32 {
    let args: Vec<String> = env::args().collect();
    let result: u32 = match args[index].trim().parse() {
        Ok(num) => num,
        Err(_) => panic!(
            "Failed to read value \"{}\", we were expecting a number!",
            args[index].trim()
        ),
    };
    result
}

pub fn calculate_percentage_increase(first: u32, second: u32) -> f32 {
    (second - first) as f32 / first as f32 * 100.0
}

pub fn calculate_percentage_decrease(first: u32, second: u32) -> f32 {
    (first - second) as f32 / first as f32 * 100.0
}

pub fn match_increase_or_decrease(first: u32, second: u32) -> (String, f32) {
    match first.cmp(&second) {
        Ordering::Less => (
            INCREASE.to_string(),
            calculate_percentage_increase(first, second),
        ),
        Ordering::Greater => (
            DECREASE.to_string(),
            calculate_percentage_decrease(first, second),
        ),
        Ordering::Equal => (EQUAL.to_string(), 0.0),
    }
}

#[cfg(test)]
mod tests {
    use super::{
        calculate_percentage_decrease, calculate_percentage_increase, match_increase_or_decrease,
    };
    #[test]
    fn test_calculate_percentage_increase() {
        assert_eq!(calculate_percentage_increase(1, 2), 100.0);
        assert_eq!(calculate_percentage_increase(1, 1), 0.0);
    }

    fn test_calculate_percentage_decrease() {
        assert_eq!(calculate_percentage_increase(2, 1), 100.0);
    }

    fn test_match_increase_or_decrease() {
        let symbol_plus = "x";
        let symbol_minus = "-";
        assert_eq!(
            match_increase_or_decrease(2, 1),
            (symbol_plus.to_string(), 100.0)
        );
        assert_eq!(
            match_increase_or_decrease(1, 2),
            (symbol_minus.to_string(), 100.0)
        );
    }
}
