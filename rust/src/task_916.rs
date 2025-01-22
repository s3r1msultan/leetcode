/*You are given two string arrays words1 and words2.

A string b is a subset of string a if every letter in b occurs in a including multiplicity.

For example, "wrr" is a subset of "warrior" but is not a subset of "world".

A string a from words1 is universal if for every string b in words2, b is a subset of a.

Return an array of all the universal strings in words1. You may return the answer in any order.



Example 1:

Input: words1 = ["amazon","apple","facebook","google","leetcode"], words2 = ["e","o"]
Output: ["facebook","google","leetcode"]

Example 2:

Input: words1 = ["amazon","apple","facebook","google","leetcode"], words2 = ["l","e"]
Output: ["apple","google","leetcode"]



Constraints:

1 <= words1.length, words2.length <= 104
1 <= words1[i].length, words2[i].length <= 10
words1[i] and words2[i] consist only of lowercase English letters.
All the strings of words1 are unique.

*/
/*use std::collections::HashMap;

struct TrieNode {
    children: HashMap<char, TrieNode>,
}

impl TrieNode {
    fn new() -> Self {
        Self {
            children: HashMap::new()
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
        for char in word.chars() {
            node = node.children.entry(char).or_insert_with(TrieNode::new);
        }
    }

    fn get_subsets(&self, node: &TrieNode, chars: &mut [i32; 26], word: &mut String, result: &mut Vec<String>) {
        if chars.iter().filter(|&&x| x > 0).count() == 0 && node.children.is_empty() {
            result.push(word.clone());
            return;
        }

        for (&char, node) in node.children.iter() {
            word.push(char);
            let i = (char as u8 - 'a' as u8) as usize;
            chars[i] -= 1;
            self.get_subsets(node, chars, word, result);
            chars[i] += 1;
            word.pop();
        }
    }
}

pub fn word_subsets(words1: Vec<String>, words2: Vec<String>) -> Vec<String> {
    let mut trie = Trie::new();
    for word in &words1 {
        trie.insert(word);
    }
    let mut max_freq = [0; 26];
    for word in words2 {
        let mut freq = [0; 26];
        for &byte in word.as_bytes() {
            let i = (byte - b'a') as usize;
            freq[i] += 1;
        }
        for i in 0..26 {
            max_freq[i] = max_freq[i].max(freq[i]);
        }
    }
    let mut result = vec![];
    trie.get_subsets(&trie.root, &mut max_freq, &mut String::new(), &mut result);
    result
}*/
pub fn word_subsets(words1: Vec<String>, words2: Vec<String>) -> Vec<String> {
    let mut max_freq = [0; 26];

    for word in &words2 {
        let mut freq = [0; 26];
        for &byte in word.as_bytes() {
            let i = (byte - b'a') as usize;
            freq[i] += 1;
        }

        for i in 0..26 {
            max_freq[i] = max_freq[i].max(freq[i]);
        }
    }

    let mut result = vec![];
    for word in &words1 {
        let mut freq = [0; 26];
        for &byte in word.as_bytes() {
            let i = (byte - b'a') as usize;
            freq[i] += 1;
        }
        let mut is_subset = true;
        for i in 0..26 {
            if freq[i] < max_freq[i] {
                is_subset = false;
                break;
            }
        }
        if is_subset {
            result.push(word.clone());
        }
    }
    result
}