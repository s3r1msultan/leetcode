/*You are given an array of integers nums and the head of a linked list. Return the head of the modified linked list after removing all nodes from the linked list that have a value that exists in nums.



Example 1:

Input: nums = [1,2,3], head = [1,2,3,4,5]

Output: [4,5]

Explanation:

Remove the nodes with values 1, 2, and 3.

Example 2:

Input: nums = [1], head = [1,2,1,2,1,2]

Output: [2,2,2]

Explanation:

Remove the nodes with value 1.

Example 3:

Input: nums = [5], head = [1,2,3,4]

Output: [1,2,3,4]

Explanation:

No node has value 5.



Constraints:

1 <= nums.length <= 105
1 <= nums[i] <= 105
All elements in nums are unique.
The number of nodes in the given list is in the range [1, 105].
1 <= Node.val <= 105
The input is generated such that there is at least one node in the linked list that has a value not present in nums.

*/
use crate::data_structures::list::ListNode;

pub fn modified_list(nums: Vec<i32>, head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
	use std::collections::HashSet;
	let nums: HashSet<i32> = nums.into_iter().collect();
	let mut head = head;
	let mut result = Box::new(ListNode {
		val: 0,
		next: head.take(),
	});
	let mut current = &mut result;
	while let Some(node) = current.next.take() {
		if nums.contains(&node.val) {
			current.next = node.next;
		} else {
			current.next = Some(node);
			current = current.next.as_mut()?;
		}
	}
	result.next
}