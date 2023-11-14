use salad_calculator;

// const ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

    // TODO - get initial_word as input
fn main() {
      // TODO - ensure casing, trim input?
    let initial_word = "decoy";
    // let alphabet = String::from("abcdefghijklmnopqrstuvwxyz");
    // let mut unused_letters = String::from("");

    // // map over alphabet and create unused_letters string based on input word
    // for letter in alphabet.chars() {
    //   if !initial_word.contains(letter) {
    //     unused_letters.push(letter)
    //   }
    // }
    println!("word_list {:?}", salad_calculator::find_word_salads(initial_word));

}
