mod lib;
use lib::calculate_percentage_change;
use lib::get_input;

fn main() {
    let first = get_input(1);
    let second = get_input(2);
    let percentage_change = calculate_percentage_change(first, second);
    println!("{}", percentage_change);
}
