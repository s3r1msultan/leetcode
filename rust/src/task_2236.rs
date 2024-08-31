/*You are given the root of a binary tree that consists of exactly 3 nodes: the root, its left child, and its right child.

Return true if the value of the root is equal to the sum of the values of its two children, or false otherwise.



Example 1:

Input: root = [10,4,6]
Output: true
Explanation: The values of the root, its left child, and its right child are 10, 4, and 6, respectively.
10 is equal to 4 + 6, so we return true.

Example 2:

Input: root = [5,3,1]
Output: false
Explanation: The values of the root, its left child, and its right child are 5, 3, and 1, respectively.
5 is not equal to 3 + 1, so we return false.



Constraints:

The tree consists only of the root, its left child, and its right child.
-100 <= Node.val <= 100

*/

use std::cell::RefCell;
use std::rc::Rc;
use crate::data_structures::tree::{build_tree_from_vec, TreeNode};

// pub fn check_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
// 	let left_val = root.as_ref().unwrap().borrow().left.as_ref().unwrap().borrow().val;
// 	let right_val = root.as_ref().unwrap().borrow().right.as_ref().unwrap().borrow().val;
// 	return root.as_ref().unwrap().borrow().val == left_val  + right_val;
// }
pub fn check_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
	if let Some(root_node) = root {
		let borrowed = root_node.borrow();
		let left_val = borrowed.left.as_ref().unwrap().borrow().val;
		let right_val = borrowed.right.as_ref().unwrap().borrow().val;
		return borrowed.val == left_val + right_val;
	}
	return false;
}

#[cfg(test)]
#[test]
fn test_check_tree() {
	let root = build_tree_from_vec(vec![10, 4, 6]);
	let result = true;
	assert_eq!(check_tree(root), result);

	let root = build_tree_from_vec(vec![5, 3, 1]);
	let result = false;
	assert_eq!(check_tree(root), result);
}