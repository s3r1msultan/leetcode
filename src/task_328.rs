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
use crate::data_structures::list::ListNode;

pub fn odd_even_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    type OptionNode = Option<Box<ListNode>>;

    let mut even: OptionNode = None;
    let mut even_head = &mut even;

    let mut odd: OptionNode = None;
    let mut odd_head = &mut odd;

    let mut is_odd = false;

    while let Some(mut node) = head {
        head = node.next.take();
        if is_odd {
            *odd_head = Some(node);
            odd_head = &mut odd_head.as_mut().unwrap().next;
        } else {
            *even_head = Some(node);
            even_head = &mut even_head.as_mut().unwrap().next;
        }
        is_odd = !is_odd;
    }

    *odd_head = even;
    odd
}