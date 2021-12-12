use std::env::args;

pub fn get_input(index: usize) -> u32 {
    let args: Vec<String> = args().collect();
    let result: u32 = match args[index].trim().parse() {
        Ok(num) => num,
        Err(_) => panic!(
            "Failed to read value \"{}\", we were expecting a number!",
            args[index].trim()
        ),
    };
    result
}

pub fn calculate_percentage_change(first: u32, second: u32) -> String {
    if first < second {
        return format!("+{}%", (second - first) as f32 / first as f32 * 100.0);
    }
    if first > second {
        return format!("-{}%", (first - second) as f32 / first as f32 * 100.0);
    } else {
        "0%".to_string()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_calculate_percentage_change() {
        assert_eq!(calculate_percentage_change(2, 1), "-50%");
        assert_eq!(calculate_percentage_change(1, 2), "+100%");
        assert_eq!(calculate_percentage_change(1, 1), "0%");
    }
}
