/*Given the root of a binary tree, return the postorder traversal of its nodes' values.



Example 1:

Input: root = [1,null,2,3]
Output: [3,2,1]

Example 2:

Input: root = []
Output: []

Example 3:

Input: root = [1]
Output: [1]



Constraints:

The number of the nodes in the tree is in the range [0, 100].
-100 <= Node.val <= 100


Follow up: Recursive solution is trivial, could you do it iteratively?*/
use std::cell::RefCell;
use std::rc::Rc;

use crate::data_structures::tree::TreeNode;

pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
	let mut result = vec![];
	let mut stack = vec![];
	let mut current = root;
	while current.is_some() || !stack.is_empty() {
		while let Some(node) = current {
			result.push(node.borrow().val);
			stack.push(node.clone());
			current = node.borrow().right.clone();
		}
		if let Some(node) = stack.pop() {
			current = node.borrow().left.clone();
		}
	}
	result.reverse();
	result
}