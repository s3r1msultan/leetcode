/*You have two types of tiles: a 2 x 1 domino shape and a tromino shape. You may rotate these shapes.

Given an integer n, return the number of ways to tile an 2 x n board. Since the answer may be very large, return it modulo 109 + 7.

In a tiling, every square must be covered by a tile. Two tilings are different if and only if there are two 4-directionally adjacent cells on the board such that exactly one of the tilings has both squares occupied by a tile.



Example 1:

Input: n = 3
Output: 5
Explanation: The five different ways are show above.

Example 2:

Input: n = 1
Output: 1



Constraints:

1 <= n <= 1000

*/

fn sum_prefix_scores(words: Vec<String>) -> Vec<i32> {
	#[derive(Debug)]
	struct Trie {
		root: Box<TrieNode>,
	}

	impl Trie {
		fn new() -> Self {
			Trie {
				root: Box::new(TrieNode::new())
			}
		}

		fn insert(&mut self, string: &str) {
			let mut current_node = &mut self.root;
			for char in string.chars() {
				let index = (char as u8 - 'a' as u8) as usize;
				if current_node.children[index].is_none() {
					current_node.children[index] = Some(Box::new(TrieNode::new()));
				}
				current_node.count += 1;
				current_node = current_node.children[index].as_mut().unwrap();
			}
		}

		fn count(&mut self, string: &str) -> i32 {
			let mut current_node = &self.root;
			let mut count = 0;
			for char in string.chars() {
				let index = (char as u8 - 'a' as u8) as usize;
				count += current_node.children[index].as_ref().unwrap().count;
				current_node = current_node.children[index].as_ref().unwrap();
			}
			count
		}
	}
	#[derive(Debug, Clone)]
	struct TrieNode {
		children: Vec<Option<Box<TrieNode>>>,
		count: i32,
	}
	impl TrieNode {
		fn new() -> Self {
			TrieNode {
				children: vec![None; 26],
				count: 0,
			}
		}
	}

	let mut trie = Trie::new();

	let mut result = vec![];
	for word in words.iter() {
		trie.insert(word.as_str());
	}

	for word in words.iter() {
		result.push(trie.count(word.as_str()));
	}

	result
}