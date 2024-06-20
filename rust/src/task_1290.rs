/*Given head which is a reference node to a singly-linked list. The value of each node in the linked list is either 0 or 1. The linked list holds the binary representation of a number.

Return the decimal value of the number in the linked list.

The most significant bit is at the head of the linked list.



Example 1:

Input: head = [1,0,1]
Output: 5
Explanation: (101) in base 2 = (5) in base 10

Example 2:

Input: head = [0]
Output: 0



Constraints:

The Linked List is not empty.
Number of nodes will not exceed 30.
Each node's value is either 0 or 1.

*/

use crate::list::ListNode;

fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
    // let mut sum = "".to_string();
    // while let Some(node) = head {}

    0
}


#[cfg(test)]
#[test]
fn test_get_decimal_value() {
    let head = ListNode::from_vec(vec![1, 0, 1]);
    let result = 5;
    assert_eq!(get_decimal_value(head), result);

    let head = ListNode::from_vec(vec![0]);
    let result = 0;
    assert_eq!(get_decimal_value(head), result);
}
