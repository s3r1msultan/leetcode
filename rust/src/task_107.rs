/*Given the root of a binary tree, return the bottom-up level order traversal of its nodes' values. (i.e., from left to right, level by level from leaf to root).



Example 1:

Input: root = [3,9,20,null,null,15,7]
Output: [[15,7],[9,20],[3]]

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

use std::cell::{Ref, RefCell};
use std::rc::Rc;
use crate::data_structures::tree::TreeNode;

// BFS
/*pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    if root.is_none() {
        return vec![];
    }

    let mut queue = std::collections::VecDeque::new();
    let mut result = vec![];

    queue.push_back(root.unwrap());

    while !queue.is_empty() {
        let n = queue.len();
        let mut level = vec![];

        for _ in 0..n {
            let node = queue.pop_front().unwrap();
            let borrowed = node.borrow();
            level.push(borrowed.val);

            if let Some(left) = borrowed.left.as_ref() {
                queue.push_back(left.clone());
            }

            if let Some(right) = borrowed.right.as_ref() {
                queue.push_back(right.clone());
            }
        }

        result.push(level);
    }
    result.reverse();
    result
}
*/

pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    fn dfs(node: Option<Rc<RefCell<TreeNode>>>, depth: usize, result: &mut Vec<Vec<i32>>) {
        if let Some(node) = node {
            let borrowed = node.borrow();
            if result.len() == depth {
                result.push(vec![]);
            }
            dfs(borrowed.left.clone(), depth+1, result);
            dfs(borrowed.right.clone(), depth+1, result);
            result[depth].push(borrowed.val);
        }
    }
    let mut result = vec![];
    dfs(root, 0, &mut result);
    result
}