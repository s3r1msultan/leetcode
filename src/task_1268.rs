/*You are given an array of strings products and a string searchWord.

Design a system that suggests at most three product names from products after each character of searchWord is typed. Suggested products should have common prefix with searchWord. If there are more than three products with a common prefix return the three lexicographically minimums products.

Return a list of lists of the suggested products after each character of searchWord is typed.



Example 1:

Input: products = ["mobile","mouse","moneypot","monitor","mousepad"], searchWord = "mouse"
Output: [["mobile","moneypot","monitor"],["mobile","moneypot","monitor"],["mouse","mousepad"],["mouse","mousepad"],["mouse","mousepad"]]
Explanation: products sorted lexicographically = ["mobile","moneypot","monitor","mouse","mousepad"].
After typing m and mo all products match and we show user ["mobile","moneypot","monitor"].
After typing mou, mous and mouse the system suggests ["mouse","mousepad"].

Example 2:

Input: products = ["havana"], searchWord = "havana"
Output: [["havana"],["havana"],["havana"],["havana"],["havana"],["havana"]]
Explanation: The only word "havana" will be always suggested while typing the search word.



Constraints:

1 <= products.length <= 1000
1 <= products[i].length <= 3000
1 <= sum(products[i].length) <= 2 * 104
All the strings of products are unique.
products[i] consists of lowercase English letters.
1 <= searchWord.length <= 1000
searchWord consists of lowercase English letters.

*/

fn suggested_products(products: Vec<String>, search_word: String) -> Vec<Vec<String>> {
	use std::collections::BinaryHeap;

	struct Trie {
		root: Box<TrieNode>,
	}

	impl Trie {
		pub fn new() -> Self {
			Trie {
				root: Box::new(TrieNode::new())
			}
		}

		pub fn insert(&mut self, string: &str) {
			let mut current_node = &mut self.root;
			for char in string.chars() {
				let i = (char as u8 - 'a' as u8) as usize;
				if current_node.children[i].is_none() {
					current_node.children[i] = Some(Box::new(TrieNode::new()));
				}
				current_node = current_node.children[i].as_mut().unwrap();
			}
			current_node.is_end = true;
		}

		pub fn search(&self, prefix: &str) -> Vec<String> {
			let mut current_node = &self.root;

			for char in prefix.chars() {
				let i = (char as u8 - 'a' as u8) as usize;
				if current_node.children[i].is_none() {
					return vec![];
				}
				current_node = current_node.children[i].as_ref().unwrap();
			}

			self.collect_words_from_prefix(current_node, prefix)
		}

		fn collect_words_from_prefix(&self, node: &Box<TrieNode>, prefix: &str) -> Vec<String> {
			let mut result = vec![];
			let mut heap = BinaryHeap::new();

			self.dfs(node, &mut heap, prefix.to_string());

			while let Some(string) = heap.pop() {
				result.push(string);
				if result.len() == 3 {
					break;
				}
			}

			result
		}

		fn dfs(&self, node: &Box<TrieNode>, heap: &mut BinaryHeap<String>, current_word: String) {
			if node.is_end {
				heap.push(current_word.clone());
				if heap.len() > 3 {
					heap.pop();
				}
			}

			for i in 0..26 {
				if let Some(ref child) = node.children[i] {
					let next_char = ('a' as u8 + i as u8) as char;
					self.dfs(child, heap, format!("{}{}", current_word, next_char));
				}
			}
		}
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

	let mut trie = Trie::new();

	for product in products {
		trie.insert(product.as_str());
	}
	let mut result = vec![];
	let mut prefix = "".to_string();

	for char in search_word.chars() {
		prefix.push(char);
		result.push(trie.search(&prefix));
	}

	result
}