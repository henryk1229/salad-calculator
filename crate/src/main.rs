use std::io;

// TODO - handle unexpected input
fn main() {
    println!("Please enter a 5 letter word...");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let initial_word = input.trim();
    let solution_set = salad_calculator::find_word_salads(initial_word);
    println!("word_list {:?}, LENGTH {}", solution_set, solution_set.len());
}
