/*Given the root of a binary tree with unique values and the values of two different nodes of the tree x and y, return true if the nodes corresponding to the values x and y in the tree are cousins, or false otherwise.

Two nodes of a binary tree are cousins if they have the same depth with different parents.

Note that in a binary tree, the root node is at the depth 0, and children of each depth k node are at the depth k + 1.



Example 1:

Input: root = [1,2,3,4], x = 4, y = 3
Output: false

Example 2:

Input: root = [1,2,3,null,4,null,5], x = 5, y = 4
Output: true

Example 3:

Input: root = [1,2,3,null,4], x = 2, y = 3
Output: false



Constraints:

The number of nodes in the tree is in the range [2, 100].
1 <= Node.val <= 100
Each node has a unique value.
x != y
x and y are exist in the tree.

*/

use std::cell::RefCell;
use std::rc::Rc;
use crate::data_structures::tree::TreeNode;

pub fn is_cousins(root: Option<Rc<RefCell<TreeNode>>>, x: i32, y: i32) -> bool {
    let mut queue = std::collections::VecDeque::new();
    queue.push_back((root.unwrap(), -1));

    while !queue.is_empty() {
        let n = queue.len();
        let mut p = -1;
        for i in 0..n {
            let (node, parent) = queue.pop_front().unwrap();
            let borrowed = node.borrow();
            if borrowed.val == x || borrowed.val == y {
                if p == -1 {
                    p = parent;
                } else {
                    return p == parent;
                }
            }

            if let Some(left) = borrowed.left.as_ref() {
                queue.push_back((left.clone(), borrowed.val));
            }

            if let Some(right) = borrowed.right.as_ref() {
                queue.push_back((right.clone(), borrowed.val));
            }
        }
    }

    false
}