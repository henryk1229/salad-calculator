use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct WordSalad {
  word: Option<String>,
  children: HashMap<String, WordSalad>,
}

impl WordSalad {
  pub fn create(word: &String) -> WordSalad {
    WordSalad {
      word: Some(word.to_string()),
      children: HashMap::new()
    }
  }
  // TODO - do we need an empty root?
  // pub fn create_root() -> WordSalad {
  //   WordSalad {
  //     word: None,
  //     children: HashMap::new(),
  //   }
  // }

  pub fn insert_word(&mut self, word: &String) {
    // let word = word.clone().to_string();
    self.children.insert(word.to_string(), WordSalad::create(word));
  }

  pub fn toss_salad(&mut self, root_word: &str, word_list: &Vec<String>, layer_number: i32) {
    let root_letters: Vec<char> = root_word.chars().collect();
    println!("ROOT LETTEERS {:?}", root_letters);
    let current_word = self.word.as_ref().unwrap();
    let child_nodes = self.children.clone();
    // handle root level - TODO - reduce branching?
    if layer_number == 0 {
      for word in word_list.iter() {
        // TODO - is there a better way to do this? substring checking?
        let mut should_insert = true;
        for letter in root_letters[0..3].iter() {
          if word.contains(*letter) {
            should_insert = false;
          }
        }
        if should_insert && word.starts_with(root_letters[4]) {
          WordSalad::insert_word(self, word)
        }
      }
    } else {
      // TODO - debug infinite loop
      // handle node layers
      for mut node in child_nodes {
        // let used_letters = root_word.to_owned().push(node.0);
        for word in word_list.iter() {
          println!("In here in here word");
          let root_letters: Vec<char> = root_word.chars().collect();
          let mut current_letters: Vec<char> = node.0.to_owned().chars().collect();
          current_letters.pop();
          let used_letters: Vec<char> = current_letters.splice(0..current_letters.len() - 1, root_letters).collect();
          let mut should_insert = true;
          for letter in used_letters.iter() {
            if word.contains(*letter) {
              should_insert = false
            }
          }
          if should_insert {
            WordSalad::insert_word(&mut node.1, word);
          }
        }
      }

      // println!("ROOT NODE LENGTH {:?}", child_nodes.len());
    }
  }
}

// #[derive(Debug)]
// pub struct WordSalad {
//   root_leaf: WordSaladLeaf
// }

// impl WordSalad {
//   pub fn new() -> WordSalad {
//     WordSalad{
//       root_leaf: WordSaladLeaf::create_root()
//     }
//   }

//   pub fn from(word_list: Vec<String>, input_string: String, parent_salad: WordSalad) -> WordSalad {
//     let mut word_salad = WordSalad::new();

//     for node in parent_salad.iter() {
//       println!("Building leaf...");
//       for word in word_list.iter() {
//         word_salad.insert(word)
//       }
//       println!("Done!");
//     }

//     // println!("Building salad...");
//     // for word in word_list.iter() {
//     //   word_salad.insert(word)
//     // }
//     // println!("Done!");

//     return word_salad
//   }
// }

// is a wordSalad a struct of tries? or is it four leaves of a trie?