use std::collections::HashMap;

pub struct Trie {
    end_of_word: bool,
    children: HashMap<char, Trie>,
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            end_of_word: false,
            children: HashMap::new(),
        }
    }

    pub fn insert(&mut self, word: &str) {
        let mut node = self;
        for ch in word.chars() {
            node = node.children.entry(ch).or_insert(Trie::new());
        }
        node.end_of_word = true;
    }

    pub fn search(&self, word: &str) -> bool {
        let mut node = self;
        for ch in word.chars() {
            match node.children.get(&ch) {
                Some(n) => node = n,
                None => return false,
            }
        }
        node.end_of_word
    }

    pub fn starts_with(&self, prefix: &str) -> bool {
        let mut node = self;
        for ch in prefix.chars() {
            match node.children.get(&ch) {
                Some(n) => node = n,
                None => return false,
            }
        }
        true
    }

    pub fn print_children_starting_with(&self, prefix: &str) {
        let mut node = self;
        let mut prefix = prefix.to_string();
        for ch in prefix.chars() {
            match node.children.get(&ch) {
                Some(n) => node = n,
                None => return,
            }
        }
        self.print_paths(&prefix, node);
    }

    fn print_paths(&self, prefix: &String, node: &Trie) {
        if node.end_of_word {
            println!("{}", prefix);
        }
        for (ch, child) in node.children.iter() {
            let mut new_prefix = prefix.clone();
            new_prefix.push(*ch);
            self.print_paths(&new_prefix, child);
        }
    }
}
