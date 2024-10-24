/*Given the root of a binary tree, return its maximum depth.

A binary tree's maximum depth is the number of nodes along the longest path from the root node down to the farthest leaf node.



Example 1:

Input: root = [3,9,20,null,null,15,7]
Output: 3

Example 2:

Input: root = [1,null,2]
Output: 2



Constraints:

The number of nodes in the tree is in the range [0, 104].
-100 <= Node.val <= 100

*/
use crate::data_structures::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
	if let Some(node) = root {
		let mut borrowed = node.borrow_mut();
		max_depth(borrowed.left.take()).max(max_depth(borrowed.right.take())) + 1
	} else {
		0
	}
}