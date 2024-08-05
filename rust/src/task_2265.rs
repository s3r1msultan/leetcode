// Given the root of a binary tree, return the number of nodes where the value of the node is equal to the average of the values in its subtree.
//
// Note:
//
// The average of n elements is the sum of the n elements divided by n and rounded down to the nearest integer.
// A subtree of root is a tree consisting of root and all of its descendants.
//
//
//
// Example 1:
//
// Input: root = [4,8,5,0,1,null,6]
// Output: 5
// Explanation:
// For the node with value 4: The average of its subtree is (4 + 8 + 5 + 0 + 1 + 6) / 6 = 24 / 6 = 4.
// For the node with value 5: The average of its subtree is (5 + 6) / 2 = 11 / 2 = 5.
// For the node with value 0: The average of its subtree is 0 / 1 = 0.
// For the node with value 1: The average of its subtree is 1 / 1 = 1.
// For the node with value 6: The average of its subtree is 6 / 1 = 6.
//
// Example 2:
//
// Input: root = [1]
// Output: 1
// Explanation: For the node with value 1: The average of its subtree is 1 / 1 = 1.

use std::cell::RefCell;
use std::rc::Rc;
use crate::data_structures::tree::{build_tree_from_vec, TreeNode};

pub fn average_of_subtree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
	let mut k = 0;
	sum_of_subtree(root, &mut k);
	fn sum_of_subtree(root: Option<Rc<RefCell<TreeNode>>>, k: &mut i32) -> (i32, i32) {
		return if let Some(node) = root {
			let mut borrowed = node.borrow_mut();
			let mut n = 1;
			let mut sum = borrowed.val;
			let (n1, sum1) = sum_of_subtree(borrowed.left.take(), k);
			let (n2, sum2) = sum_of_subtree(borrowed.right.take(), k);
			n += n1 + n2;
			sum += sum1 + sum2;
			if sum / n == borrowed.val {
				*k += 1
			}
			(n, sum)
		} else {
			(0, 0)
		};
	}
	k
}


#[cfg(test)]
#[test]
fn test_average_of_subtree() {
	let tree = build_tree_from_vec(vec![4, 8, 5, 0, 1, -1, 6]);
	let result = 5;
	assert_eq!(average_of_subtree(tree), result);
}