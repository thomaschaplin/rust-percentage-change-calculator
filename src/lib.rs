use std::cmp::Ordering;
use std::env;

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

pub fn calculate_percentage_change(first: u32, second: u32) -> (String, f32) {
    match first.cmp(&second) {
        Ordering::Less => (
            "+".to_string(),
            (second - first) as f32 / first as f32 * 100.0,
        ),
        Ordering::Greater => (
            "-".to_string(),
            (first - second) as f32 / first as f32 * 100.0,
        ),
        Ordering::Equal => ("".to_string(), 0.0),
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_match_increase_or_decrease() {
        assert_eq!(calculate_percentage_change(2, 1), ("-".to_string(), 50.0));
        assert_eq!(calculate_percentage_change(1, 2), ("+".to_string(), 100.0));
        assert_eq!(calculate_percentage_change(1, 1), ("".to_string(), 0.0));
    }
}
