use std::collections::HashMap;

struct TrieNode {
    children: HashMap<char, TrieNode>,
    frequency: i32,
}

impl TrieNode {
    fn new() -> Self {
        Self {
            children: HashMap::new(),
            frequency: 0,
        }
    }
}


struct Trie {
    root: TrieNode,
}

impl Trie {
    fn new() -> Self {
        Self {
            root: TrieNode::new()
        }
    }

    fn insert(&mut self, word: &str) {
        let mut node = &mut self.root;
        for ch in word.chars() {
            node = node.children.entry(ch).or_insert_with(TrieNode::new);
            node.frequency += 1;
        }
    }

    fn starts_with(&self, word: &str) -> bool {
        let mut node = &self.root;
        for ch in word.chars() {
            if let Some(child) = node.children.get(&ch) {
                node = child;
            } else {
                return false;
            }
        }
        true
    }
}

pub fn is_prefix_of_word(sentence: String, search_word: String) -> i32 {
    let mut trie = Trie::new();
    for (i, word) in sentence.split_whitespace().enumerate() {
        trie.insert(word);
        if trie.starts_with(&search_word) {
            return i as i32 + 1;
        }
    }
    -1
}
