mod lib;
use lib::get_input;
use lib::calculate_percentage_change;

fn main() {
    let first = get_input(1);
    let second = get_input(2);
    let (symbol, percentage_change) = calculate_percentage_change(first, second);
    println!("{}{}%", symbol, percentage_change);
}
