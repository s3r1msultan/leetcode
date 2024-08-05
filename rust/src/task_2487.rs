use crate::data_structures::list::ListNode;

// You are given the head of a linked list.
//
// Remove every node which has a node with a greater value anywhere to the right side of it.
//
// Return the head of the modified linked list.
//
//
//
// Example 1:
//
// Input: head = [5,2,13,3,8]
// Output: [13,8]
// Explanation: The nodes that should be removed are 5, 2 and 3.
// - Node 13 is to the right of node 5.
// - Node 13 is to the right of node 2.
// - Node 8 is to the right of node 3.
//
// Example 2:
//
// Input: head = [1,1,1,1]
// Output: [1,1,1,1]
// Explanation: Every node has value 1, so no nodes are removed.
//
//
//
// Constraints:
//
// The number of the nodes in the given list is in the range [1, 105].
// 1 <= Node.val <= 105
//
pub fn remove_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
	let mut head = head;
	let mut stack: Vec<Box<ListNode>> = vec![];

	while let Some(mut node) = head {
		head = node.next.take();

		while !stack.is_empty() && stack.last().unwrap().val < node.val {
			stack.pop();
		}

		stack.push(node);
	}

	let mut result: Option<Box<ListNode>> = None;
	while let Some(mut node) = stack.pop() {
		node.next = result;
		result = Some(node);
	}
	result
}