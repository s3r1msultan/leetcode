/*Given the root of a binary tree, the level of its root is 1, the level of its children is 2, and so on.

Return the smallest level x such that the sum of all the values of nodes at level x is maximal.



Example 1:

Input: root = [1,7,0,7,-8,null,null]
Output: 2
Explanation:
Level 1 sum = 1.
Level 2 sum = 7 + 0 = 7.
Level 3 sum = 7 + -8 = -1.
So we return the level with the maximum sum which is level 2.

Example 2:

Input: root = [989,null,10250,98693,-89388,null,null,null,-32127]
Output: 2



Constraints:

The number of nodes in the tree is in the range [1, 104].
-105 <= Node.val <= 105

*/
use std::cell::RefCell;
use std::rc::Rc;
use crate::data_structures::tree::TreeNode;

pub fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if root.is_none() {
        return 0;
    }
    use std::collections::VecDeque;
    let mut queue = VecDeque::new();
    queue.push_back(root.unwrap());
    let mut max_level_sum = i32::MIN;
    let mut current_level = 0;
    let mut result_level = 1;
    while !queue.is_empty() {
        current_level += 1;
        let mut level_sum = 0;
        let n = queue.len();
        for _ in 0..n {
            let node = queue.pop_front().unwrap();
            let borrowed = node.borrow();
            level_sum += borrowed.val;
            if let Some(left_node) = borrowed.left.as_ref() {
                queue.push_back(left_node.clone());
            }

            if let Some(right_node) = borrowed.right.as_ref() {
                queue.push_back(right_node.clone());
            }
        }
        if level_sum > max_level_sum {
            max_level_sum = level_sum;
            result_level = current_level;
        }
    }
    result_level
}