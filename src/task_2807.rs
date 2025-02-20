/*Given the head of a linked list head, in which each node contains an integer value.

Between every pair of adjacent nodes, insert a new node with a value equal to the greatest common divisor of them.

Return the linked list after insertion.

The greatest common divisor of two numbers is the largest positive integer that evenly divides both numbers.



Example 1:

Input: head = [18,6,10,3]
Output: [18,6,6,2,10,1,3]
Explanation: The 1st diagram denotes the initial linked list and the 2nd diagram denotes the linked list after inserting the new nodes (nodes in blue are the inserted nodes).
- We insert the greatest common divisor of 18 and 6 = 6 between the 1st and the 2nd nodes.
- We insert the greatest common divisor of 6 and 10 = 2 between the 2nd and the 3rd nodes.
- We insert the greatest common divisor of 10 and 3 = 1 between the 3rd and the 4th nodes.
There are no more adjacent nodes, so we return the linked list.

Example 2:

Input: head = [7]
Output: [7]
Explanation: The 1st diagram denotes the initial linked list and the 2nd diagram denotes the linked list after inserting the new nodes.
There are no pairs of adjacent nodes, so we return the initial linked list.



Constraints:

The number of nodes in the list is in the range [1, 5000].
1 <= Node.val <= 1000

*/
// enum Something {
// 	Some(_),
// 	None,
// }


use crate::data_structures::list::ListNode;

fn insert_greatest_common_divisors(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
	fn gcd(a: i32, b: i32) -> i32 {
		if b == 0 {
			return a;
		}
		gcd(b, a % b)
	}
	let mut head = head;
	let mut current = head.as_mut();

	while let Some(mut curr_node) = current {
		if let Some(next_node) = curr_node.next.as_mut() {
			let curr_value = curr_node.val;
			let next_value = next_node.val;
			let gcd = gcd(curr_value, next_value);
			let list_node = Box::new(ListNode {
				val: gcd,
				next: curr_node.next.take(),
			});
			curr_node.next = Some(list_node);
			current = curr_node.next.as_mut().unwrap().next.as_mut();
		} else {
			break;
		}
	}

	head
}

#[cfg(test)]
#[test]
fn test_insert_greatest_common_divisors() {
	let head = ListNode::from_vec(vec![18, 6, 10, 3]);
	let result = ListNode::from_vec(vec![18, 6, 6, 2, 10, 1]);
	assert_eq!(insert_greatest_common_divisors(head), result);
	let head = ListNode::from_vec(vec![7]);
	let result = ListNode::from_vec(vec![7]);
	assert_eq!(insert_greatest_common_divisors(head), result);
}