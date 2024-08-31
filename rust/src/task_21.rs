/*You are given the heads of two sorted linked lists list1 and list2.

Merge the two lists into one sorted list. The list should be made by splicing together the nodes of the first two lists.

Return the head of the merged linked list.



Example 1:

Input: list1 = [1,2,4], list2 = [1,3,4]
Output: [1,1,2,3,4,4]

Example 2:

Input: list1 = [], list2 = []
Output: []

Example 3:

Input: list1 = [], list2 = [0]
Output: [0]



Constraints:

The number of nodes in both lists is in the range [0, 50].
-100 <= Node.val <= 100
Both list1 and list2 are sorted in non-decreasing order.

*/

use crate::data_structures::list::ListNode;

fn merge_two_lists(
	list1: Option<Box<ListNode>>,
	list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
	let mut dummy = ListNode::new(0);
	let mut tail = &mut dummy;

	let mut l1 = list1;
	let mut l2 = list2;
	while l1.is_some() && l2.is_some() {
		let l1_node = l1.as_mut().unwrap();
		let l2_node = l2.as_mut().unwrap();

		if l1_node.val > l2_node.val {
			tail.next = l2.take();
			l2 = tail.next.as_mut().unwrap().next.take();
		} else {
			tail.next = l1.take();
			l1 = tail.next.as_mut().unwrap().next.take();
		}

		tail = tail.next.as_mut().unwrap();
	}

	if l1.is_some() {
		tail.next = l1;
	} else {
		tail.next = l2;
	}

	dummy.next
}