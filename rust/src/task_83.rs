// Given the head of a sorted linked list, delete all duplicates such that each element appears only once. Return the linked list sorted as well.
//
//
//
// Example 1:
//
// Input: head = [1,1,2]
// Output: [1,2]
//
// Example 2:
//
// Input: head = [1,1,2,3,3]
// Output: [1,2,3]
//
// Definition for singly-linked list.

use crate::data_structures::list::ListNode;

pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
	let mut current = head;
	let mut node = &mut current;
	while let Some(curr) = node {
		while curr.next.as_ref().map_or(false, |next| next.val == curr.val) {
			curr.next = curr.next.as_mut().unwrap().next.take();
		}
		node = &mut curr.next;
	}
	current
}

#[cfg(test)]
#[test]
fn test_delete_duplicates() {}