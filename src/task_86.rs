/*Given the head of a linked list and a value x, partition it such that all nodes less than x come before nodes greater than or equal to x.

You should preserve the original relative order of the nodes in each of the two partitions.



Example 1:

Input: head = [1,4,3,2,5,2], x = 3
Output: [1,2,2,4,3,5]

Example 2:

Input: head = [2,1], x = 2
Output: [1,2]



Constraints:

The number of nodes in the list is in the range [0, 200].
-100 <= Node.val <= 100
-200 <= x <= 200

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
pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
    let mut left_head = Box::new(ListNode::new(0));
    let mut left_tail = &mut left_head;
    let mut right_head = Box::new(ListNode::new(0));
    let mut right_tail = &mut right_head;
    let mut head = head;
    while let Some(node) = head {
        if node.val < x {
            left_tail.next = Some(Box::new(ListNode::new(node.val)));
            left_tail = left_tail.next.as_mut().unwrap();
        } else {
            right_tail.next = Some(Box::new(ListNode::new(node.val)));
            right_tail = right_tail.next.as_mut().unwrap();
        }
        head = node.next;
    }
    left_tail.next = right_head.next;
    left_head.next
}
