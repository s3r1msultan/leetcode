/*Medium
Topics
Companies
Hint

A binary tree is named Even-Odd if it meets the following conditions:

The root of the binary tree is at level index 0, its children are at level index 1, their children are at level index 2, etc.
For every even-indexed level, all nodes at the level have odd integer values in strictly increasing order (from left to right).
For every odd-indexed level, all nodes at the level have even integer values in strictly decreasing order (from left to right).

Given the root of a binary tree, return true if the binary tree is Even-Odd, otherwise return false.



Example 1:

Input: root = [1,10,4,3,null,7,9,12,8,6,null,null,2]
Output: true
Explanation: The node values on each level are:
Level 0: [1]
Level 1: [10,4]
Level 2: [3,7,9]
Level 3: [12,8,6,2]
Since levels 0 and 2 are all odd and increasing and levels 1 and 3 are all even and decreasing, the tree is Even-Odd.

Example 2:

Input: root = [5,4,2,3,3,7]
Output: false
Explanation: The node values on each level are:
Level 0: [5]
Level 1: [4,2]
Level 2: [3,3,7]
Node values in level 2 must be in strictly increasing order, so the tree is not Even-Odd.

Example 3:

Input: root = [5,9,1,3,5,7]
Output: false
Explanation: Node values in the level 1 should be even integers.



Constraints:

The number of nodes in the tree is in the range [1, 105].
1 <= Node.val <= 106

*/
use crate::data_structures::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn is_even_odd_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let mut queue = std::collections::VecDeque::new();
    queue.push_back(root.unwrap());
    let mut is_odd_index = false;
    while !queue.is_empty() {
        let n = queue.len();
        let mut temp = if is_odd_index {
            i32::MAX
        } else {
            0
        };

        for i in 0..n {
            let node = queue.pop_front().unwrap();
            let borrowed = node.borrow();
            let val = borrowed.val;
            if is_odd_index && (val % 2 == 1 || temp <= val) {
                return false;
            }

            if !is_odd_index && (val % 2 == 0 || temp >= val) {
                return false;
            }

            temp = val;
            if let Some(left) = borrowed.left.as_ref() {
                queue.push_back(left.clone());
            }

            if let Some(right) = borrowed.right.as_ref() {
                queue.push_back(right.clone());
            }
        }

        is_odd_index = !is_odd_index;
    }
    true
}
