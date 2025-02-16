/*Given the root of a binary tree, return an array of the largest value in each row of the tree (0-indexed).



Example 1:

Input: root = [1,3,2,5,3,null,9]
Output: [1,3,9]

Example 2:

Input: root = [1,2,3]
Output: [1,3]



Constraints:

The number of nodes in the tree will be in the range [0, 104].
-231 <= Node.val <= 231 - 1

*/

/*pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut result = vec![];

    let mut queue = std::collections::VecDeque::new();
    queue.push_back(root.as_ref().unwrap());
    while !queue.is_empty() {
        let n = queue.len();
        let mut max = i32::MIN;
        for _ in 0..n {
            let node = queue.pop_front().unwrap();
            let borrowed = node.borrow();
            max = max.max(borrowed.val);

            if let Some(left) = borrowed.left.as_ref() {
                queue.push_back(left);
            }

            if let Some(right) = borrowed.right.as_ref() {
                queue.push_back(right);
            }
        }
        result.push(max);
    }

    result
}*/
use crate::data_structures::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut result = vec![];

    let mut queue = std::collections::VecDeque::new();
    queue.push_back(root.unwrap());

    while !queue.is_empty() {
        let n = queue.len();
        let mut max = i32::MIN;
        for _ in 0..n {
            let node = queue.pop_front().unwrap();
            let borrowed = node.borrow();
            max = max.max(borrowed.val);

            if let Some(left) = borrowed.left.as_ref() {
                queue.push_back(left.clone());
            }

            if let Some(right) = borrowed.right.as_ref() {
                queue.push_back(right.clone());
            }
        }
        result.push(max);
    }

    result
}