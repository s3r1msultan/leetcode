/*Given the root of a binary tree, imagine yourself standing on the right side of it, return the values of the nodes you can see ordered from top to bottom.



Example 1:

Input: root = [1,2,3,null,5,null,4]
Output: [1,3,4]

Example 2:

Input: root = [1,null,3]
Output: [1,3]

Example 3:

Input: root = []
Output: []



Constraints:

The number of nodes in the tree is in the range [0, 100].
-100 <= Node.val <= 100

*/
use std::cell::RefCell;
use std::rc::Rc;
use crate::data_structures::tree::TreeNode;

pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    if root.is_none() {
        return vec![];
    }

    use std::collections::VecDeque;
    let mut result = vec![];
    let mut queue = VecDeque::new();
    queue.push_back(root.unwrap());

    while !queue.is_empty() {
        let n = queue.len();
        for i in 0..n {
            let node = queue.pop_front().unwrap();
            let borrowed=  node.borrow();
            if i == n-1 {
                result.push(borrowed.val);
            }
            if let Some(left_node) = borrowed.left.as_ref() {
                queue.push_back(left_node.clone());
            }
            if let Some(right_node) = borrowed.right.as_ref() {
                queue.push_back(right_node.clone());
            }
        }
    }

    result
}