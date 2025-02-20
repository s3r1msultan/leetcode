/*Given the root of a binary tree, return the average value of the nodes on each level in the form of an array. Answers within 10-5 of the actual answer will be accepted.



Example 1:

Input: root = [3,9,20,null,null,15,7]
Output: [3.00000,14.50000,11.00000]
Explanation: The average value of nodes on level 0 is 3, on level 1 is 14.5, and on level 2 is 11.
Hence, return [3, 14.5, 11].

Example 2:

Input: root = [3,9,20,15,7]
Output: [3.00000,14.50000,11.00000]



Constraints:

The number of nodes in the tree is in the range [1, 104].
-231 <= Node.val <= 231 - 1
*/

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
use crate::data_structures::tree::{build_tree_from_vec, TreeNode};

fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
	let mut queue = VecDeque::new();
	queue.push_back((root, 0));
	let mut counts: Vec<i64> = vec![];
	let mut sums: Vec<i64> = vec![];

	while let Some((node, pos)) = queue.pop_front() {
		if let Some(subtree) = node {
			let borrowed = subtree.borrow();

			if sums.len() <= pos {
				sums.push(0);
				counts.push(0);
			}

			sums[pos] += borrowed.val as i64;
			counts[pos] += 1;

			queue.push_back((borrowed.left.clone(), pos + 1));
			queue.push_back((borrowed.right.clone(), pos + 1));
		}
	}
	let mut result: Vec<f64> = vec![];
	for (&sum, &count) in sums.iter().zip(counts.iter()) {
		result.push(sum as f64 / count as f64)
	}
	result
}

#[cfg(test)]
#[test]
fn test_average_of_levels() {
	let root = build_tree_from_vec(vec![3, 9, 20, -1, -1, 15, 7]);
	let result = vec![3.00000, 14.50000, 11.00000];
	assert_eq!(average_of_levels(root), result);

	let root = build_tree_from_vec(vec![3, 9, 20, 15, 7]);
	let result = vec![3.00000, 14.50000, 11.00000];
	assert_eq!(average_of_levels(root), result);
}