/*Given an array of string words, return all strings in words that is a substring of another word. You can return the answer in any order.

A substring is a contiguous sequence of characters within a string



Example 1:

Input: words = ["mass","as","hero","superhero"]
Output: ["as","hero"]
Explanation: "as" is substring of "mass" and "hero" is substring of "superhero".
["hero","as"] is also a valid answer.

Example 2:

Input: words = ["leetcode","et","code"]
Output: ["et","code"]
Explanation: "et", "code" are substring of "leetcode".

Example 3:

Input: words = ["blue","green","bu"]
Output: []
Explanation: No string of words is substring of another string.



Constraints:

1 <= words.length <= 100
1 <= words[i].length <= 30
words[i] contains only lowercase English letters.
All the strings of words are unique.

*/

struct TrieNode {
    children: [Option<Box<TrieNode>>; 26],
    frequency: i32,
}

impl TrieNode {
    fn new() -> Self {
        Self {
            children: Default::default(),
            frequency: 0,
        }
    }
}

struct Trie {
    root: Box<TrieNode>,
}

impl Trie {
    fn new() -> Self {
        Self {
            root: Box::new(TrieNode::new())
        }
    }

    fn insert(&mut self, word: &str) {
        let mut node = &mut self.root;
        for &ch in word.as_bytes() {
            let i = (ch - b'a') as usize;
            if node.children[i].is_none() {
                node.children[i] = Some(Box::new(TrieNode::new()));
            }
            node = node.children[i].as_mut().unwrap();
            node.frequency += 1;
        }
    }

    fn exists(&self, word: &str) -> bool {
        let mut node = &self.root;
        for &ch in word.as_bytes() {
            let i = (ch - b'a') as usize;
            if node.children[i].is_none() {
                return false;
            }
            node = node.children[i].as_ref().unwrap();
        }

        node.frequency > 1
    }
}

pub fn string_matching(words: Vec<String>) -> Vec<String> {
    let mut trie = Trie::new();

    for word in &words {
        for i in 0..word.len() {
            trie.insert(&word[i..]);
        }
    }


    let mut result = vec![];

    for word in &words {
        if trie.exists(&word) {
            result.push(word.clone());
        }
    }

    result
}

/*pub fn string_matching(words: Vec<String>) -> Vec<String> {
    let mut result = std::collections::HashSet::new();

    for word1 in &words {
        for word2 in &words {
            if !word1.ne(&word2) && word1.contains(word2) {
                result.insert(word2.clone());
            }
        }
    }

    result.into_iter().collect()
}*/