pub fn word_list() -> String {
    let word_list = include_str!("../word_list/words_alpha.txt")
      .split("\r\n")
      .map(|str| str.to_string())
      // filter words that aren't five characters in lenght
      .filter(|ref line| {
        if line.len() != 5 {
          return false
        }
        return true
      }).collect();
    word_list
}