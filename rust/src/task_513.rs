/*Given the root of a binary tree, return the leftmost value in the last row of the tree.



Example 1:

Input: root = [2,1,3]
Output: 1

Example 2:

Input: root = [1,2,3,4,null,5,6,null,null,7]
Output: 7



Constraints:

The number of nodes in the tree is in the range [1, 104].
-231 <= Node.val <= 231 - 1

*/

use std::cell::RefCell;
use std::rc::Rc;
use crate::data_structures::tree::TreeNode;
use crate::task_104::max_depth;
/*
pub fn find_bottom_left_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut left = 0;
    let mut queue = std::collections::VecDeque::new();
    queue.push_back(root.unwrap());

    while !queue.is_empty() {
        let n = queue.len();
        for i in 0..n {
            let node = queue.pop_front().unwrap();
            let borrowed = node.borrow();
            if i == 0 {
                left = borrowed.val;
            }

            if let Some(left) = borrowed.left.as_ref() {
                queue.push_back(left.clone());
            }

            if let Some(right) = borrowed.right.as_ref() {
                queue.push_back(right.clone());
            }
        }
    }

    left
}*/


pub fn find_bottom_left_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn dfs(node: Option<Rc<RefCell<TreeNode>>>, depth: i32, max_depth: &mut i32, leftmost: &mut i32) {
        if let Some(node) = node {
            let borrowed = node.borrow();
            if depth > *max_depth {
                *max_depth = depth;
                *leftmost = borrowed.val;
            }
            dfs(borrowed.left.clone(), depth + 1, max_depth, leftmost);
            dfs(borrowed.right.clone(), depth + 1, max_depth, leftmost);
        }
    }
    let mut leftmost = 0;
    dfs(root, 0, 0, &mut leftmost);
    leftmost
}