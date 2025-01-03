use first_part::algo;
use first_part::parse_input;
mod first_part;

fn main() {
    let list = parse_input();
    let result = algo(list);
    println!("The final count is: {:?}", result);
}
