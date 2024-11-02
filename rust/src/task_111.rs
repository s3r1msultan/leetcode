/*Given a binary tree, find its minimum depth.

The minimum depth is the number of nodes along the shortest path from the root node down to the nearest leaf node.

Note: A leaf is a node with no children.



Example 1:

Input: root = [3,9,20,null,null,15,7]
Output: 2

Example 2:

Input: root = [2,null,3,null,4,null,5,null,6]
Output: 5



Constraints:

The number of nodes in the tree is in the range [0, 105].
-1000 <= Node.val <= 1000

*/
use crate::data_structures::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if root.is_none() {
        return 0;
    }
    let mut queue = std::collections::VecDeque::new();
    queue.push_back(root.unwrap());
    let mut min_depth = 0;
    while !queue.is_empty() {
        let n = queue.len();
        min_depth += 1;
        for _ in 0..n {
            let node = queue.pop_front().unwrap();
            let borrowed = node.borrow();
            if borrowed.left.is_none() && borrowed.right.is_none() {
                return min_depth;
            }
            if let Some(left) = borrowed.left.as_ref() {
                queue.push_back(left.clone());
            }
            if let Some(right) = borrowed.right.as_ref() {
                queue.push_back(right.clone());
            }
        }
    }
    min_depth
}