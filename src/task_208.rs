/*A trie (pronounced as "try") or prefix tree is a tree data structure used to efficiently store and retrieve keys in a dataset of strings. There are various applications of this data structure, such as autocomplete and spellchecker.

Implement the Trie class:

Trie() Initializes the trie object.
void insert(String word) Inserts the string word into the trie.
boolean search(String word) Returns true if the string word is in the trie (i.e., was inserted before), and false otherwise.
boolean startsWith(String prefix) Returns true if there is a previously inserted string word that has the prefix prefix, and false otherwise.



Example 1:

Input
["Trie", "insert", "search", "search", "startsWith", "insert", "search"]
[[], ["apple"], ["apple"], ["app"], ["app"], ["app"], ["app"]]
Output
[null, null, true, false, true, null, true]

Explanation
Trie trie = new Trie();
trie.insert("apple");
trie.search("apple");   // return True
trie.search("app");     // return False
trie.startsWith("app"); // return True
trie.insert("app");
trie.search("app");     // return True



Constraints:

1 <= word.length, prefix.length <= 2000
word and prefix consist only of lowercase English letters.
At most 3 * 104 calls in total will be made to insert, search, and startsWith.

*/

struct Trie {
	root: Box<TrieNode>,
}
struct TrieNode {
	children: [Option<Box<TrieNode>>; 26],
	is_end: bool,
}

impl TrieNode {
	pub fn new() -> Self {
		TrieNode {
			children: Default::default(),
			is_end: false,
		}
	}
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {
	fn new() -> Self {
		Trie {
			root: Box::new(TrieNode::new())
		}
	}

	fn insert(&mut self, word: String) {
		let mut current_node = &mut self.root;
		for char in word.chars() {
			let index = (char as u8 - 'a' as u8) as usize;
			if current_node.children[index].is_none() {
				current_node.children[index] = Some(Box::new(TrieNode::new()));
			}
			current_node = current_node.children[index].as_mut().unwrap();
		}
		current_node.is_end = true;
	}

	fn search(&self, word: String) -> bool {
		let mut current_node = &self.root;
		for char in word.chars() {
			let index = (char as u8 - 'a' as u8) as usize;
			if current_node.children[index].is_none() {
				return false;
			}
			current_node = current_node.children[index].as_ref().unwrap();
		}
		current_node.is_end
	}

	fn starts_with(&self, prefix: String) -> bool {
		let mut current_node = &self.root;
		for char in prefix.chars() {
			let index = (char as u8 - 'a' as u8) as usize;
			if current_node.children[index].is_none() {
				return false;
			}
			current_node = current_node.children[index].as_ref().unwrap();
		}
		true
	}
}

/*
 * Your Trie object will be instantiated and called as such:
 * let obj = Trie::new();
 * obj.insert(word);
 * let ret_2: bool = obj.search(word);
 * let ret_3: bool = obj.starts_with(prefix);
 */

