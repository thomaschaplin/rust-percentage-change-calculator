use std::env::args;

pub fn get_input(index: usize) -> f32 {
    let args: Vec<String> = args().collect();
    let result: f32 = match args[index].trim().parse() {
        Ok(num) => num,
        Err(_) => panic!(
            "Failed to read value \"{}\", we were expecting a u32 or f32!",
            args[index].trim()
        ),
    };
    result
}

pub fn calculate_percentage_change(first: f32, second: f32) -> String {
    if first < second {
        return format!("+{}%", (second - first) / first * 100.0);
    }
    if first > second {
        return format!("-{}%", (first - second) / first * 100.0);
    } else {
        "0%".to_string()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_calculate_percentage_change() {
        assert_eq!(calculate_percentage_change(1 as f32, 2 as f32), "+100%");
        assert_eq!(calculate_percentage_change(2 as f32, 1 as f32), "-50%");
        assert_eq!(calculate_percentage_change(1 as f32, 1 as f32), "0%");
        assert_eq!(calculate_percentage_change(1.2, 2.4), "+100%");
        assert_eq!(calculate_percentage_change(2.4, 1.2), "-50%");
        assert_eq!(calculate_percentage_change(1.1, 1.1), "0%");
    }
}
