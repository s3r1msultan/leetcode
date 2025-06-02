/*Given the root of a complete binary tree, return the number of the nodes in the tree.

According to Wikipedia, every level, except possibly the last, is completely filled in a complete binary tree, and all nodes in the last level are as far left as possible. It can have between 1 and 2h nodes inclusive at the last level h.

Design an algorithm that runs in less than O(n) time complexity.



Example 1:

Input: root = [1,2,3,4,5,6]
Output: 6

Example 2:

Input: root = []
Output: 0

Example 3:

Input: root = [1]
Output: 1



Constraints:

The number of nodes in the tree is in the range [0, 5 * 104].
0 <= Node.val <= 5 * 104
The tree is guaranteed to be complete.

*/
use crate::data_structures::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if root.is_none() {
        return 0;
    }
    let mut count = 0;
    let mut queue = std::collections::VecDeque::new();

    queue.push_back(root.unwrap());

    while let Some(node) = queue.pop_back() {
        count += 1;

        let borrowed = node.borrow();

        if let Some(left) = borrowed.left.as_ref() {
            queue.push_back(left.clone());
        }

        if let Some(right) = borrowed.right.as_ref() {
            queue.push_back(right.clone());
        }
    }

    count
}