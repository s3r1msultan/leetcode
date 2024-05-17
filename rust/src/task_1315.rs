/*Given the root of a binary tree, return the sum of values of nodes with an even-valued grandparent. If there are no nodes with an even-valued grandparent, return 0.

A grandparent of a node is the parent of its parent if it exists.



Example 1:

Input: root = [6,7,8,2,7,1,3,9,null,1,4,null,null,null,5]
Output: 18
Explanation: The red nodes are the nodes with even-value grandparent while the blue nodes are the even-value grandparents.

Example 2:

Input: root = [1]
Output: 0



Constraints:

    The number of nodes in the tree is in the range [1, 104].
    1 <= Node.val <= 100

*/


use std::cell::RefCell;
use std::rc::Rc;
use crate::tree::{build_tree_from_vec, TreeNode};

pub fn sum_even_grandparent(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
	return traverse(root, false);
	fn traverse(root: Option<Rc<RefCell<TreeNode>>>, is_even: bool) -> i32 {
		if let Some(root_node) = root {
			let borrowed = root_node.borrow();
			let flag = borrowed.val % 2 == 0;
			let mut sum = 0;
			if borrowed.left.is_some() && is_even {
				sum+=borrowed.left.as_ref().unwrap().borrow().val;
			}

			if borrowed.right.is_some() && is_even {
				sum+=borrowed.right.as_ref().unwrap().borrow().val;
			}
			sum += traverse(borrowed.left.clone(), flag) + traverse(borrowed.right.clone(), flag);
			return sum;
		}
		return 0;
	}
}


#[cfg(test)]

#[test]

fn test_sum_even_grandparent() {
	let root = build_tree_from_vec(vec![6,7,8,2,7,1,3,9,-1,1,4,-1,-1,-1,5]);
	let result = 18;
	assert_eq!(sum_even_grandparent(root), result);

	let root = build_tree_from_vec(vec![1]);
	let result = 0;
	assert_eq!(sum_even_grandparent(root), result);
}