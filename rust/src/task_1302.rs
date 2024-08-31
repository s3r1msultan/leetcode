// Given the root of a binary tree, return the sum of values of its deepest leaves.
//
//
//
// Example 1:
//
// Input: root = [1,2,3,4,5,null,6,7,null,null,null,null,8]
// Output: 15
//
// Example 2:
//
// Input: root = [6,7,8,2,7,1,3,9,null,1,4,null,null,null,5]
// Output: 19
//
//
//
// Constraints:
//
// The number of nodes in the tree is in the range [1, 104].
// 1 <= Node.val <= 100



use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
use crate::data_structures::tree::{build_tree_from_vec, TreeNode};

pub fn deepest_leaves_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
	if root.is_none() {
		return 0;
	}
	let mut sum = 0;
	let mut max_depth = 0;
	let mut stack = VecDeque::new();
	stack.push_back((root, 1));
	while let Some((node_option, curr_depth)) = stack.pop_front() {
		if let Some(node) = node_option {
			let mut node_ref = node.borrow_mut();
			let left_leave = node_ref.left.take();
			let right_leave = node_ref.right.take();

			if left_leave.is_none() && right_leave.is_none() {
				if curr_depth == max_depth {
					sum += node_ref.val;
				} else if curr_depth > max_depth {
					sum = node_ref.val;
					max_depth = curr_depth;
				}
			}
			if left_leave.is_some() {
				stack.push_back((left_leave, curr_depth + 1));
			}
			if right_leave.is_some() {
				stack.push_back((right_leave, curr_depth + 1));
			}
		}
	}

	sum
}

#[cfg(test)]
#[test]
fn test_deepest_leaves_sum() {
	let root = build_tree_from_vec(vec![1, 2, 3, 4, 5, -1, 6, 7, -1, -1, -1, -1, 8]);
	assert_eq!(deepest_leaves_sum(root), 15);

	let root = build_tree_from_vec(vec![6, 7, 8, 2, 7, 1, 3, 9, -1, 1, 4, -1, -1, -1, 5]);
	assert_eq!(deepest_leaves_sum(root), 19);
}