use std::collections::HashMap;

#[derive(Debug)]
pub struct SaladLayer {
  word: Option<String>,
  children: HashMap<String, SaladLayer>,
}

impl SaladLayer {
  pub fn create(word: &str) -> SaladLayer {
    SaladLayer {
      word: Some(word.to_string()),
      children: HashMap::new()
    }
  }
  // TODO - do we need an empty root?
  // pub fn create_root() -> SaladLayer {
  //   SaladLayer {
  //     word: None,
  //     children: HashMap::new(),
  //   }
  // }

  pub fn insert_word(&mut self, word: String) {
    self.children.insert(word, SaladLayer::create(&word));
  }

  pub fn toss_salad(&mut self, root_word: &str, word_list: &Vec<String>) {
    let nodes = self.children;
    // let used_letters: Vec<char> = root_word.chars().collect();
    for node in nodes {
      // let used_letters = root_word.to_owned().push(node.0);
      // let 
      // for letter in used_letters {

      // }
      for word in word_list {
        if !root_word.contains(*word) && !node.0.contains(*word) {
          SaladLayer::insert_word(&mut self, word.to_string());
        }
        
      }
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