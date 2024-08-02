fn main() {
    println!("Generating a wordsalad...");
    let solution_set = salad_calculator::find_word_salads();
    println!("word_list {:?}, LENGTH {}", solution_set, solution_set.len());
}
