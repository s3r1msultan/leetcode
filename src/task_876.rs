/*Given the head of a singly linked list, return the middle node of the linked list.

If there are two middle nodes, return the second middle node.



Example 1:

Input: head = [1,2,3,4,5]
Output: [3,4,5]
Explanation: The middle node of the list is node 3.

Example 2:

Input: head = [1,2,3,4,5,6]
Output: [4,5,6]
Explanation: Since the list has two middle nodes with values 3 and 4, we return the second one.



Constraints:

The number of nodes in the list is in the range [1, 100].
1 <= Node.val <= 100

*/

use crate::data_structures::list::ListNode;

fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
	let mut slow = &head;
	let mut fast = &head;
	while let Some(ListNode { next: Some(fast_ref), .. }) = fast.as_deref() {
		fast = &fast_ref.next;
		slow = &slow.as_ref().unwrap().next;
	}

	slow.clone()
}