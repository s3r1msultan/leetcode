/*You are given the root of a binary tree with n nodes. Each node is uniquely assigned a value from 1 to n. You are also given an integer startValue representing the value of the start node s, and a different integer destValue representing the value of the destination node t.

Find the shortest path starting from node s and ending at node t. Generate step-by-step directions of such path as a string consisting of only the uppercase letters 'L', 'R', and 'U'. Each letter indicates a specific direction:

'L' means to go from a node to its left child node.
'R' means to go from a node to its right child node.
'U' means to go from a node to its parent node.

Return the step-by-step directions of the shortest path from node s to node t.



Example 1:

Input: root = [5,1,2,3,null,6,4], startValue = 3, destValue = 6
Output: "UURL"
Explanation: The shortest path is: 3 → 1 → 5 → 2 → 6.

Example 2:

Input: root = [2,1], startValue = 2, destValue = 1
Output: "L"
Explanation: The shortest path is: 2 → 1.



Constraints:

The number of nodes in the tree is n.
2 <= n <= 105
1 <= Node.val <= n
All the values in the tree are unique.
1 <= startValue, destValue <= n
startValue != destValue

*/

use std::cell::RefCell;
use std::rc::Rc;

use crate::data_structures::tree::{build_tree_from_vec, TreeNode};

fn get_directions(root: Option<Rc<RefCell<TreeNode>>>, start_value: i32, dest_value: i32) -> String {
	fn find_path(node: &Option<Rc<RefCell<TreeNode>>>, val: i32, path: &mut Vec<char>) -> bool {
		if let Some(n) = node {
			let borrowed = n.borrow();
			if borrowed.val == val {
				return true;
			}

			path.push('L');
			if find_path(&borrowed.left, val, path) {
				return true;
			}
			path.pop();

			path.push('R');
			if find_path(&borrowed.right, val, path) {
				return true;
			}
			path.pop();
		}

		false
	}

	fn find_lca_path(path1: &Vec<char>, path2: &Vec<char>) -> usize {
		let mut i = 0;
		while i < path1.len() && i < path2.len() && path1[i] == path2[i] {
			i += 1;
		}

		i
	}

	let mut path_to_start: Vec<char> = Vec::new();
	let mut path_to_dest: Vec<char> = Vec::new();

	find_path(&root, start_value, &mut path_to_start);
	find_path(&root, dest_value, &mut path_to_dest);

	let mut i = find_lca_path(&path_to_start, &path_to_dest);

	let mut result = String::new();

	for _ in i..path_to_start.len() {
		result.push('U');
	}

	for &direction in &path_to_dest[i..] {
		result.push(direction);
	}

	result
}


#[cfg(test)]
#[test]
fn test_get_directions() {
	let root = build_tree_from_vec(vec![5, 1, 2, 3, -1, 6, 4]);
	let start_value = 3;
	let dest_value = 6;
	let result = "UURL".to_string();
	assert_eq!(get_directions(root, start_value, dest_value), result);

	let root = build_tree_from_vec(vec![2, 1]);
	let start_value = 2;
	let dest_value = 1;
	let result = "UURL".to_string();
	assert_eq!(get_directions(root, start_value, dest_value), result);
}