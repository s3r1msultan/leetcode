/*Given a binary tree root and an integer target, delete all the leaf nodes with value target.

Note that once you delete a leaf node with value target, if its parent node becomes a leaf node and has the value target, it should also be deleted (you need to continue doing that until you cannot).



Example 1:

Input: root = [1,2,3,2,null,2,4], target = 2
Output: [1,null,3,null,4]
Explanation: Leaf nodes in green with value (target = 2) are removed (Picture in left).
After removing, new nodes become leaf nodes with value (target = 2) (Picture in center).

Example 2:

Input: root = [1,3,3,3,2], target = 3
Output: [1,3,null,null,2]

Example 3:

Input: root = [1,2,null,2,null,2], target = 2
Output: [1]
Explanation: Leaf nodes in green with value (target = 2) are removed at each step.



Constraints:

    The number of nodes in the tree is in the range [1, 3000].
    1 <= Node.val, target <= 1000

*/

use std::cell::RefCell;
use std::rc::Rc;
use crate::data_structures::tree::{build_tree_from_vec, TreeNode};

pub fn remove_leaf_nodes(root: Option<Rc<RefCell<TreeNode>>>, target: i32) -> Option<Rc<RefCell<TreeNode>>> {
	if let Some(root_node) = root {
		{
			let mut borrowed = root_node.borrow_mut();
			borrowed.left = remove_leaf_nodes(borrowed.left.clone(), target);
			borrowed.right = remove_leaf_nodes(borrowed.right.clone(), target);
			if borrowed.left.is_none() && borrowed.right.is_none() && borrowed.val == target {
				return None;
			}
		}
		return Some(root_node);
	}
	return None;
}

#[cfg(test)]
#[test]
fn test_remove_leaf_nodes() {
	let root = build_tree_from_vec(vec![1, 2, 3, 2, -1, 2, 4]);
	let target = 2;
	let result = build_tree_from_vec(vec![1, -1, 3, -1, 4]);
	assert_eq!(remove_leaf_nodes(root, target), result);

	let root = build_tree_from_vec(vec![1, 3, 3, 3, 2]);
	let target = 3;
	let result = build_tree_from_vec(vec![1, 3, -1, -1, 2]);
	assert_eq!(remove_leaf_nodes(root, target), result);

	let root = build_tree_from_vec(vec![1, 2, -1, 2, -1, 2]);
	let target = 2;
	let result = build_tree_from_vec(vec![1]);
	assert_eq!(remove_leaf_nodes(root, target), result);
}