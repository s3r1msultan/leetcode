// Given the head of a linked list, remove the nth node from the end of the list and return its head.
//
//
//
// Example 1:
//
// Input: head = [1,2,3,4,5], n = 2
// Output: [1,2,3,5]
//
// Example 2:
//
// Input: head = [1], n = 1
// Output: []
//
// Example 3:
//
// Input: head = [1,2], n = 1
// Output: [1]
//
//
//
// Constraints:
//
// The number of nodes in the list is sz.
// 1 <= sz <= 30
// 0 <= Node.val <= 100
// 1 <= n <= sz


use crate::data_structures::list::ListNode;

pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
	let mut head = head;
	let mut stack: Vec<Box<ListNode>> = Vec::new();
	while let Some(mut node) = head {
		head = node.next.take();
		stack.push(node);
	}

	let mut result: Option<Box<ListNode>> = None;
	let mut k = 0;
	while let Some(mut node) = stack.pop() {
		k += 1;
		if k == n {
			continue;
		}
		node.next = result;
		result = Some(node);
	}
	result
}