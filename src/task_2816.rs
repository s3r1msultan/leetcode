// You are given the head of a non-empty linked list representing a non-negative integer without leading zeroes.
//
// Return the head of the linked list after doubling it.
//
//
//
// Example 1:
//
// Input: head = [1,8,9]
// Output: [3,7,8]
// Explanation: The figure above corresponds to the given linked list which represents the number 189. Hence, the returned linked list represents the number 189 * 2 = 378.
//
// Example 2:
//
// Input: head = [9,9,9]
// Output: [1,9,9,8]
// Explanation: The figure above corresponds to the given linked list which represents the number 999. Hence, the returned linked list reprersents the number 999 * 2 = 1998.
//
//
//
// Constraints:
//
// The number of nodes in the list is in the range [1, 104]
// 0 <= Node.val <= 9
// The input is generated such that the list represents a number that does not have leading zeros, except the number 0 itself.
//

use crate::data_structures::list::ListNode;

pub fn double_it(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
	let mut head = head;
	let mut stack: Vec<Box<ListNode>> = vec![];
	let mut res = 0;
	while let Some(mut node) = head {
		head = node.next.take();
		stack.push(node);
	}

	let mut result: Option<Box<ListNode>> = None;
	while let Some(mut node) = stack.pop() {
		let double = node.val * 2 + res;
		res = double / 10;
		node.val = double % 10;
		node.next = result;
		result = Some(node);
	}

	if res == 1 {
		let mut head = Box::new(ListNode::new(1));
		head.next = result;
		return Some(head);
	}

	result
}