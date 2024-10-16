/*Given the head of a linked list and an integer val, remove all the nodes of the linked list that has Node.val == val, and return the new head.



Example 1:

Input: head = [1,2,6,3,4,5,6], val = 6
Output: [1,2,3,4,5]

Example 2:

Input: head = [], val = 1
Output: []

Example 3:

Input: head = [7,7,7,7], val = 7
Output: []



Constraints:

The number of nodes in the list is in the range [0, 104].
1 <= Node.val <= 50
0 <= val <= 50

*/
use crate::data_structures::list::ListNode;

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
	let mut dummy = Some(Box::new(ListNode {
		val: 0,
		next: head,
	}));
	helper(dummy.as_mut(), val);
	dummy.unwrap().next
}

pub fn helper(head: Option<&mut Box<ListNode>>, val: i32) {
	if let Some(curr_node) = head {
		if let Some(next_node) = curr_node.next.as_mut() {
			if next_node.val == val {
				curr_node.next = next_node.next.take();
				helper(Some(curr_node), val);
			} else {
				helper(Some(next_node), val);
			}
		}
	}
}