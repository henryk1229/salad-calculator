use salad_calculator;

// TODO - get initial_word as input, return solution set
fn main() {
      // TODO - ensure casing, trim input?
    let initial_word = "decoy";
    let solution_set = salad_calculator::find_word_salads(initial_word);
    println!("word_list {:?}, LENGTH {}", solution_set, solution_set.len());
}
