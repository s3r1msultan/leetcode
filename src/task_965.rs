/*A binary tree is uni-valued if every node in the tree has the same value.

Given the root of a binary tree, return true if the given tree is uni-valued, or false otherwise.



Example 1:

Input: root = [1,1,1,1,1,null,1]
Output: true

Example 2:

Input: root = [2,2,2,5,2]
Output: false



Constraints:

The number of nodes in the tree is in the range [1, 100].
0 <= Node.val < 100

*/

/*pub fn is_unival_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let mut queue = std::collections::VecDeque::new();
    let value = root.as_ref().unwrap().borrow().val;
    queue.push_back(root.unwrap());

    while !queue.is_empty() {
        let n = queue.len();
        for i in 0..n {
            let node = queue.pop_front().unwrap();
            let borrowed = node.borrow();
            if borrowed.val != value {
                return false;
            }
            if let Some(left_node) = borrowed.left.as_ref() {
                queue.push_back(left_node.clone());
            }
            if let Some(right_node) = borrowed.right.as_ref() {
                queue.push_back(right_node.clone());
            }
        }
    }

    true
}*/
use crate::data_structures::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn is_unival_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, value: i32) -> bool {
        if let Some(node) = root {
            let borrowed = node.borrow();
            if value != borrowed.val {
                return false;
            }
            return dfs(borrowed.left.clone(), value) && dfs(borrowed.right.clone(), value);
        }
        true
    }

    let value = root.as_ref().unwrap().borrow().val;
    dfs(root, value)
}