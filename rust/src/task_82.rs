use crate::list::ListNode;

// Given the head of a sorted linked list, delete all nodes that have duplicate numbers, leaving only distinct numbers from the original list. Return the linked list sorted as well.
//
//
//
// Example 1:
//
// Input: head = [1,2,3,3,4,4,5]
// Output: [1,2,5]
//
// Example 2:
//
// Input: head = [1,1,1,2,3]
// Output: [2,3]
pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut dummy_head = Some(Box::new(ListNode{val: 0, next: head}));
    let mut node = &mut dummy_head;

    while let Some(ref mut current) = node.as_mut().unwrap().next {
      if current.next.is_some() && current.val == current.next.as_ref().unwrap().val {
        while current.next.as_ref().map_or(false, |next| next.val == current.val) {
          current.next = current.next.as_mut().unwrap().next.take();
        }
        node.as_mut().unwrap().next = current.next.take();
      } else {
        node = &mut node.as_mut().unwrap().next;
      }
    }


    dummy_head.unwrap().next
}