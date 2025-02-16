// You are given two non-empty linked lists representing two non-negative integers. The digits are stored in reverse order, and each of their nodes contains a single digit. Add the two numbers and return the sum as a linked list.
//
// You may assume the two numbers do not contain any leading zero, except the number 0 itself.
//
//
//
// Example 1:
//
// Input: l1 = [2,4,3], l2 = [5,6,4]
// Output: [7,0,8]
// Explanation: 342 + 465 = 807.
//
// Example 2:
//
// Input: l1 = [0], l2 = [0]
// Output: [0]
//
// Example 3:
//
// Input: l1 = [9,9,9,9,9,9,9], l2 = [9,9,9,9]
// Output: [8,9,9,9,0,0,0,1]
//
//
//
// Constraints:
//
// The number of nodes in each linked list is in the range [1, 100].
// 0 <= Node.val <= 9
// It is guaranteed that the list represents a number that does not have leading zeros.
//

// use crate::list::ListNode;

use crate::data_structures::list::ListNode;

pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
	let mut head_1 = l1;
	let mut head_2 = l2;
	let mut remainder = 0;
	let mut res: Option<Box<ListNode>> = None;
	let mut tail = &mut res;
	while head_1.is_some() || head_2.is_some() || remainder != 0 {
		let val1 = head_1.as_ref().map_or(0, |node| node.val);
		let val2 = head_2.as_ref().map_or(0, |node| node.val);
		let sum = val1 + val2 + remainder;
		remainder = sum / 10;
		let sum = sum % 10;

		let new_node = Box::new(ListNode::new(sum));
		if let Some(ref mut t) = *tail {
			t.next = Some(new_node);
		} else {
			*tail = Some(new_node);
		}

		tail = &mut tail.as_mut().unwrap().next;

		head_1 = head_1.and_then(|node| node.next);
		head_2 = head_2.and_then(|node| node.next);
	}
	res
}

#[cfg(test)]
#[test]
fn test_add_two_numbers() {
	let l1: Option<Box<ListNode>> = ListNode::from_vec(vec![2, 4, 3]);
	let l2: Option<Box<ListNode>> = ListNode::from_vec(vec![5, 6, 4]);
	let res: Option<Box<ListNode>> = ListNode::from_vec(vec![7, 0, 8]);
	assert_eq!(add_two_numbers(l1, l2), res);
}