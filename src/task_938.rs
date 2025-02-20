/*Given the root node of a binary search tree and two integers low and high, return the sum of values of all nodes with a value in the inclusive range [low, high].



Example 1:

Input: root = [10,5,15,3,7,null,18], low = 7, high = 15
Output: 32
Explanation: Nodes 7, 10, and 15 are in the range [7, 15]. 7 + 10 + 15 = 32.

Example 2:

Input: root = [10,5,15,3,7,13,18,1,null,6], low = 6, high = 10
Output: 23
Explanation: Nodes 6, 7, and 10 are in the range [6, 10]. 6 + 7 + 10 = 23.



Constraints:

    The number of nodes in the tree is in the range [1, 2 * 104].
    1 <= Node.val <= 105
    1 <= low <= high <= 105
    All Node.val are unique.

*/

use std::cell::RefCell;
use std::rc::Rc;
use crate::data_structures::tree::TreeNode;
/*
pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
	if let Some(root_node) = root {
		let mut borrowed = root_node.borrow_mut();
		let mut sum = 0;
		if borrowed.val >= low && borrowed.val <= high {
			sum+=borrowed.val;
		}
		sum += range_sum_bst(borrowed.left.take(), low, high) + range_sum_bst(borrowed.right.take(), low, high);
		return sum;
	}
	return 0;
}
*/

pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
	if root.is_none() {
		return 0;
	}
	let mut stack = vec![root.unwrap()];
	let mut sum = 0;
	while !stack.is_empty() {
		let root = stack.pop().unwrap();
		let mut borrowed = root.borrow_mut();
		if low <= borrowed.val && borrowed.val <= high {
			sum += borrowed.val;
		}
		if borrowed.left.is_some() {
			stack.push(borrowed.left.take().unwrap());
		}
		if borrowed.right.is_some() {
			stack.push(borrowed.right.take().unwrap());
		}
	}
	sum
}