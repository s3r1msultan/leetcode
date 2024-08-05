/*Given the root of a binary tree, each node in the tree has a distinct value.

After deleting all nodes with a value in to_delete, we are left with a forest (a disjoint union of trees).

Return the roots of the trees in the remaining forest. You may return the result in any order.



Example 1:

Input: root = [1,2,3,4,5,6,7], to_delete = [3,5]
Output: [[1,2,null,4],[6],[7]]

Example 2:

Input: root = [1,2,4,null,3], to_delete = [3]
Output: [[1,2,4]]



Constraints:

The number of nodes in the given tree is at most 1000.
Each node has a distinct value between 1 and 1000.
to_delete.length <= 1000
to_delete contains distinct values between 1 and 1000.

*/


use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

use crate::data_structures::tree::{build_tree_from_vec, TreeNode};

fn del_nodes(root: Option<Rc<RefCell<TreeNode>>>, to_delete: Vec<i32>) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
	let mut result = vec![root.clone()];
	let set: HashSet<i32> = to_delete.into_iter().collect();
	fn helper(node: Option<Rc<RefCell<TreeNode>>>, set: &HashSet<i32>, result: &mut Vec<Option<Rc<RefCell<TreeNode>>>>) -> Option<Rc<RefCell<TreeNode>>> {
		if let Some(n) = node.clone() {
			let mut n_borrowed = n.borrow_mut();
			n_borrowed.left = helper(n_borrowed.left.take(), set, result);
			n_borrowed.right = helper(n_borrowed.right.take(), set, result);
			if set.contains(&n_borrowed.val) {
				if n_borrowed.left.is_some() {
					result.push(n_borrowed.left.take());
				}

				if n_borrowed.right.is_some() {
					result.push(n_borrowed.right.take());
				}

				return None;
			}
		}
		node
	}
	let root = helper(root, &set, &mut result);
	if root.is_some() {
		result.push(root);
	}
	result
}


#[cfg(test)]
#[test]
fn test_del_nodes() {
	let root = build_tree_from_vec(vec![1, 2, 3, 4, 5, 6, 7]);
	let to_delete = vec![3, 5];
	let mut result = vec![];
	for tree in vec![vec![1, 2, -1, 4], vec![6], vec![7]] {
		result.push(build_tree_from_vec(tree));
	}
	assert_eq!(del_nodes(root, to_delete), result);

	let root = build_tree_from_vec(vec![1, 2, 3, 4, 5, 6, 7]);
	let to_delete = vec![3, 5];
	let mut result = vec![];
	for tree in vec![vec![1, 2, -1, 4], vec![6], vec![7]] {
		result.push(build_tree_from_vec(tree));
	}
	assert_eq!(del_nodes(root, to_delete), result);
}