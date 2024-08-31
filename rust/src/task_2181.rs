/*You are given the head of a linked list, which contains a series of integers separated by 0's. The beginning and end of the linked list will have Node.val == 0.

For every two consecutive 0's, merge all the nodes lying in between them into a single node whose value is the sum of all the merged nodes. The modified list should not contain any 0's.

Return the head of the modified linked list.



Example 1:

Input: head = [0,3,1,0,4,5,2,0]
Output: [4,11]
Explanation:
The above figure represents the given linked list. The modified list contains
- The sum of the nodes marked in green: 3 + 1 = 4.
- The sum of the nodes marked in red: 4 + 5 + 2 = 11.

Example 2:

Input: head = [0,1,0,3,0,2,2,0]
Output: [1,3,4]
Explanation:
The above figure represents the given linked list. The modified list contains
- The sum of the nodes marked in green: 1 = 1.
- The sum of the nodes marked in red: 3 = 3.
- The sum of the nodes marked in yellow: 2 + 2 = 4.



Constraints:

The number of nodes in the list is in the range [3, 2 * 105].
0 <= Node.val <= 1000
There are no two consecutive nodes with Node.val == 0.
The beginning and end of the linked list have Node.val == 0.

*/


use crate::data_structures::list::ListNode;

fn merge_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
	let mut head = head.unwrap().next;
	let mut result_head = Box::new(ListNode::new(0));
	let mut result_next = &mut result_head;
	let mut sum = 0;
	while let Some(node) = head {
		if node.val == 0 {
			let new_node = Box::new(ListNode::new(sum));
			sum = 0;
			result_next.next = Some(new_node);
			result_next = result_next.next.as_mut().unwrap();
		} else {
			sum += node.val;
		}
		head = node.next;
	}
	result_head.next
}

#[cfg(test)]
#[test]
fn test_merge_nodes() {
	let head = ListNode::from_vec(vec![0, 3, 1, 0, 4, 5, 2, 0]);
	let result = ListNode::from_vec(vec![4, 11]);
	assert_eq!(merge_nodes(head), result);

	let head = ListNode::from_vec(vec![0, 1, 0, 3, 0, 2, 2, 0]);
	let result = ListNode::from_vec(vec![1, 3, 4]);
	assert_eq!(merge_nodes(head), result);
}
