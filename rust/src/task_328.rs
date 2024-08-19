/*Given the head of a singly linked list, group all the nodes with odd indices together followed by the nodes with even indices, and return the reordered list.

The first node is considered odd, and the second node is even, and so on.

Note that the relative order inside both the even and odd groups should remain as it was in the input.

You must solve the problem in O(1) extra space complexity and O(n) time complexity.



Example 1:

Input: head = [1,2,3,4,5]
Output: [1,3,5,2,4]

Example 2:

Input: head = [2,1,3,5,6,4,7]
Output: [2,3,6,7,1,5,4]



Constraints:

The number of nodes in the linked list is in the range [0, 104].
-106 <= Node.val <= 106

*/
/*use crate::data_structures::list::ListNode;

pub fn odd_even_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
	if head.is_none() || head.clone().unwrap().next.is_none() {
		return head;
	}
	let mut head = head;
	let mut even_start: Option<Box<ListNode>> = None;
	let mut even_tail: &mut Option<Box<ListNode>> = &mut None;
	let mut odd_start: Option<Box<ListNode>> = None;
	let mut odd_tail: &mut Option<Box<ListNode>> = &mut None;
	let mut index = 0;

	while let Some(node) = head {
		if index % 2 == 0 {
			if even_tail.is_some() {
				even_tail.as_mut().unwrap().next = Some(node);
				even_tail = &mut even_tail.unwrap().next;
			} else {}
		} else {}

		index += 1;
	}

	if even_tail.is_some() {
		even_tail.as_mut().unwrap().next = odd_start;
	}

	even_start
}*/