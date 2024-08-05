// Given the head of a singly linked list, reverse the list, and return the reversed list.
//
//
//
// Example 1:
//
// Input: head = [1,2,3,4,5]
// Output: [5,4,3,2,1]
//
// Example 2:
//
// Input: head = [1,2]
// Output: [2,1]
//
// Example 3:
//
// Input: head = []
// Output: []
//
//
//
// Constraints:
//
// The number of nodes in the list is the range [0, 5000].
// -5000 <= Node.val <= 5000

use crate::data_structures::list::ListNode;

pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
	let mut head = head;
	let mut result: Option<Box<ListNode>> = Option::from(Box::new(ListNode::new(0)));
	while let Some(mut node) = head {
		head = node.next.take();
		node.next = result;
		result = Some(node);
	}
	result
}