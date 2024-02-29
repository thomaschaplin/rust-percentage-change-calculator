use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <first_value> <second_value>", args[0]);
        std::process::exit(1);
    }

    let first = parse_input(&args[1]);
    let second = parse_input(&args[2]);
    match (first, second) {
        (Ok(first), Ok(second)) => {
            match calculate_percentage_change(first, second) {
                Ok(percentage_change) => println!("{}", percentage_change),
                Err(err) => {
                    eprintln!("{}", err);
                    std::process::exit(1);
                }
            }
        }
        (Err(err), _) | (_, Err(err)) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    }
}

fn parse_input(input: &str) -> Result<f32, String> {
    input.trim().parse().map_err(|_| format!("Failed to parse value: {}", input))
}

fn calculate_percentage_change(first: f32, second: f32) -> Result<String, String> {
    let percentage_change = match (first, second) {
        (f, s) if f < s => format!("+{}%", ((s - f) / f * 100.0)),
        (f, s) if f > s => format!("-{}%", ((f - s) / f * 100.0)),
        (_, _) => "0%".to_string(),
    };
    Ok(percentage_change)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_percentage_change() {
        assert_eq!(calculate_percentage_change(1.0, 2.0), Ok("+100%".to_string()));
        assert_eq!(calculate_percentage_change(2.0, 1.0), Ok("-50%".to_string()));
        assert_eq!(calculate_percentage_change(1.0, 1.0), Ok("0%".to_string()));
        assert_eq!(calculate_percentage_change(1.2, 2.4), Ok("+100%".to_string()));
        assert_eq!(calculate_percentage_change(2.4, 1.2), Ok("-50%".to_string()));
        assert_eq!(calculate_percentage_change(1.1, 1.1), Ok("0%".to_string()));
    }
}
