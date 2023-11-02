use std::collections::HashMap;

#[derive(Debug)]
pub struct TrieNode {
  // TODO
  value: Option<char>,
  is_word: bool,
  children: HashMap<char, TrieNode>,
}

impl TrieNode {
  pub fn create(c: char, is_word: bool) -> TrieNode {
    TrieNode {
      value: Some(c),
      is_word,
      children: HashMap::new(),
    }
  }

  pub fn create_root() -> TrieNode {
    TrieNode {
      value: None,
      is_word: false,
      children: HashMap::new()
    }
  }

  // pub fn check_value(self, c: char) -> bool {
  //   self.value == Some(c)
  // }

  pub fn insert_value(&mut self, c: char, is_word: bool) {
    self.children.insert(c, TrieNode::create(c, is_word));
  }
}

#[derive(Debug)]
pub struct Trie {
  root_node: TrieNode
}

impl Trie {
  pub fn new() -> Trie {
    Trie {
      root_node: TrieNode::create_root()
    }
  }

  pub fn from(word_list: Vec<String>) -> Trie {
    let mut trie = Trie::new();

    print!("Building Trie...");
    for word in word_list.iter() {
      println!("inserting {}...", word);
      trie.insert(word)
    }
    println!("DONE!");

    return trie;
   }

  pub fn insert(&mut self, value: &str) {
    let chars: Vec<char> = value.chars().collect();
    let mut current_node = &mut self.root_node;
    let mut last_match_index = 0;

    for i in 0..chars.len() {
      if current_node.children.contains_key(&chars[i]) {
        // TODO - comments 
        current_node = current_node.children.get_mut(&chars[i]).unwrap();
        last_match_index = i + 1;
      } else {
        last_match_index = i;
        break;
      }
    }

    if last_match_index == chars.len() {
      // TODO - better var name?
      current_node.is_word = true;
    } else {
      for i in last_match_index..chars.len() {
        current_node.insert_value(chars[i], false);
        current_node = current_node.children.get_mut(&chars[i]).unwrap();
      }

      current_node.is_word = true;
    }
  }
}