mod first_part;
mod second_part;

fn main() {
    let numbers = first_part::parse_and_extract();
    // let distances = first_part::find_distances(numbers);
    //
    // let result = first_part::add_distances(distances);
    //
    // print!("{result}");
    //
    //
    let sim = second_part::find_similarity(numbers);

    let sum = second_part::get_similaarity_value(sim);

    print!("{sum}");
}
