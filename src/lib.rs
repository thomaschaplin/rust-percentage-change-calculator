use std::env;
use std::cmp::Ordering;

const INCREASE: &str = "+";
const DECREASE: &str = "-";
const EQUAL: &str = "";

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
    return (second - first) as f32 / first as f32 * 100.0;
}

pub fn calculate_percentage_decrease(first: u32, second: u32) -> f32 {
    return (first - second) as f32 / first as f32 * 100.0;
}

pub fn match_increase_or_decrease(first: u32, second: u32) -> (String, f32) {
    match first.cmp(&second) {
        Ordering::Less => return (INCREASE.to_string(), calculate_percentage_increase(first, second)),
        Ordering::Greater => return (DECREASE.to_string(), calculate_percentage_decrease(first, second)),
        Ordering::Equal => {
            return (EQUAL.to_string(), 0.0);
        }
    }
}
