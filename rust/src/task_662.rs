/*Given the root of a binary tree, return the maximum width of the given tree.

The maximum width of a tree is the maximum width among all levels.

The width of one level is defined as the length between the end-nodes (the leftmost and rightmost non-null nodes), where the null nodes between the end-nodes that would be present in a complete binary tree extending down to that level are also counted into the length calculation.

It is guaranteed that the answer will in the range of a 32-bit signed integer.



Example 1:

Input: root = [1,3,2,5,3,null,9]
Output: 4
Explanation: The maximum width exists in the third level with length 4 (5,3,null,9).

Example 2:

Input: root = [1,3,2,5,null,null,9,6,null,7]
Output: 7
Explanation: The maximum width exists in the fourth level with length 7 (6,null,null,null,null,null,7).

Example 3:

Input: root = [1,3,2,5]
Output: 2
Explanation: The maximum width exists in the second level with length 2 (3,2).



Constraints:

The number of nodes in the tree is in the range [1, 3000].
-100 <= Node.val <= 100

*/
use crate::data_structures::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn width_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if root.is_none() {
        return 0;
    }

    let mut queue = std::collections::VecDeque::new();
    queue.push_back((root.unwrap(), 1));
    let mut max_width = 0;

    while !queue.is_empty() {
        let n = queue.len();
        let mut left = 0;
        let mut right = 0;
        for i in 0..n {
            let (node, pos) = queue.pop_front().unwrap();
            let borrowed = node.borrow();
            if left == 0 {
                left = pos;
            }
            right = pos;
            if let Some(left) = borrowed.left.as_ref() {
                queue.push_back((left.clone(), 2 * pos));
            }

            if let Some(right) = borrowed.right.as_ref() {
                queue.push_back((right.clone(), 2 * pos + 1));
            }

            max_width = max_width.max(right - left + 1);
        }
    }

    max_width
}