/*You are given a 2D integer array descriptions where descriptions[i] = [parenti, childi, isLefti] indicates that parenti is the parent of childi in a binary tree of unique values. Furthermore,

If isLefti == 1, then childi is the left child of parenti.
If isLefti == 0, then childi is the right child of parenti.

Construct the binary tree described by descriptions and return its root.

The test cases will be generated such that the binary tree is valid.



Example 1:

Input: descriptions = [[20,15,1],[20,17,0],[50,20,1],[50,80,0],[80,19,1]]
Output: [50,20,80,15,17,19]
Explanation: The root node is the node with value 50 since it has no parent.
The resulting binary tree is shown in the diagram.

Example 2:

Input: descriptions = [[1,2,1],[2,3,0],[3,4,1]]
Output: [1,2,null,null,3,4]
Explanation: The root node is the node with value 1 since it has no parent.
The resulting binary tree is shown in the diagram.



Constraints:

1 <= descriptions.length <= 104
descriptions[i].length == 3
1 <= parenti, childi <= 105
0 <= isLefti <= 1
The binary tree described by descriptions is valid.

*/


use std::cell::RefCell;
use std::rc::Rc;

use crate::tree::{build_tree_from_vec, TreeNode};

fn create_binary_tree(descriptions: Vec<Vec<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
	use std::collections::{HashMap, HashSet};
	let mut parents = HashSet::new();
	let mut children = HashSet::new();
	let mut nodes = HashMap::new();
	for description in descriptions.iter() {
		let parent = description[0];
		let child = description[1];
		let is_left = description[2] == 1;
		parents.insert(parent);
		children.insert(child);
		let parent_node = nodes.entry(parent).or_insert_with(|| Rc::new(RefCell::new(TreeNode::new(parent)))).clone();
		let child_node = nodes.entry(child).or_insert_with(|| Rc::new(RefCell::new(TreeNode::new(child)))).clone();
		if is_left {
			parent_node.borrow_mut().left = Some(Rc::clone(&child_node));
		} else {
			parent_node.borrow_mut().right = Some(Rc::clone(&child_node));
		}
	}

	let root_num = parents.difference(&children).next().unwrap();

	nodes.get(root_num).cloned()
}


#[cfg(test)]
#[test]
fn test_create_binary_tree() {
	let descriptions = vec![vec![20, 15, 1], vec![20, 17, 0], vec![50, 20, 1], vec![50, 80, 0], vec![80, 19, 1]];
	let result = build_tree_from_vec(vec![50, 20, 80, 15, 17, 19]);
	assert_eq!(create_binary_tree(descriptions), result);

	let descriptions = vec![vec![1, 2, 1], vec![2, 3, 0], vec![3, 4, 1]];
	let result = build_tree_from_vec(vec![1, 2, -1, -1, 3, 4]);
	assert_eq!(create_binary_tree(descriptions), result);
}