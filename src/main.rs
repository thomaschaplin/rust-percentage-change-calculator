mod lib;
use lib::get_input;
use lib::match_increase_or_decrease;

fn main() {
    let first = get_input(1);
    let second = get_input(2);
    let (symbol, percentage_change) = match_increase_or_decrease(first, second);
    println!("{}{}%", symbol, percentage_change);
}
