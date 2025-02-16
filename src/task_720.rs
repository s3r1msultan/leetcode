/*Given an array of strings words representing an English Dictionary, return the longest word in words that can be built one character at a time by other words in words.

If there is more than one possible answer, return the longest word with the smallest lexicographical order. If there is no answer, return the empty string.

Note that the word should be built from left to right with each additional character being added to the end of a previous word.



Example 1:

Input: words = ["w","wo","wor","worl","world"]
Output: "world"
Explanation: The word "world" can be built one character at a time by "w", "wo", "wor", and "worl".

Example 2:

Input: words = ["a","banana","app","appl","ap","apply","apple"]
Output: "apple"
Explanation: Both "apply" and "apple" can be built from other words in the dictionary. However, "apple" is lexicographically smaller than "apply".



Constraints:

1 <= words.length <= 1000
1 <= words[i].length <= 30
words[i] consists of lowercase English letters.

*/
use std::collections::HashMap;

struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_end: bool,
}

impl TrieNode {
    fn new() -> Self {
        Self {
            children: HashMap::new(),
            is_end: false,
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
        }
        node.is_end;
    }

    fn get_frequent_word(&self, node: &TrieNode, current_word: &mut String, longest_word: &mut String) {
        for (&ch, node) in node.children.iter() {
            if node.is_end {
                current_word.push(ch);
                if current_word.len() > longest_word.len() || (current_word.len() == longest_word.len() && current_word.as_bytes().lt(&longest_word.as_bytes())) {
                    *longest_word = current_word.clone();
                }
                self.get_frequent_word(node, current_word, longest_word);
                current_word.pop();
            }
        }
    }
}
pub fn longest_word(words: Vec<String>) -> String {
    let mut trie = Trie::new();
    for word in words {
        trie.insert(&word);
    }
    let mut longest_word = String::new();
    trie.get_frequent_word(&trie.root, &mut String::new(), &mut longest_word);
    longest_word
}