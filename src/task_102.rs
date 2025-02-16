/*Given the root of a binary tree, return the level order traversal of its nodes' values. (i.e., from left to right, level by level).



Example 1:

Input: root = [3,9,20,null,null,15,7]
Output: [[3],[9,20],[15,7]]

Example 2:

Input: root = [1]
Output: [[1]]

Example 3:

Input: root = []
Output: []



Constraints:

The number of nodes in the tree is in the range [0, 2000].
-1000 <= Node.val <= 1000

*/
use std::cell::RefCell;
use std::rc::Rc;
use crate::data_structures::tree::TreeNode;

fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    use std::collections::VecDeque;
    if root.is_none() {
        return vec![];
    }

    let mut result = vec![];
    let mut queue = VecDeque::new();
    queue.push_back(root.unwrap());

    while !queue.is_empty() {
        let n = queue.len();
        let mut current_level = vec![];
        for _ in 0..n {
            let node = queue.pop_front().unwrap();
            let borrowed = node.borrow();
            current_level.push(borrowed.val);

            if let Some(left) = &borrowed.left {
                queue.push_back(left.clone());
            }

            if let Some(right) = &borrowed.right {
                queue.push_back(right.clone());
            }

        }

        result.push(current_level);
    }

    result

}



