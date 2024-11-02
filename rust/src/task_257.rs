/*Given the root of a binary tree, return all root-to-leaf paths in any order.

A leaf is a node with no children.



Example 1:

Input: root = [1,2,3,null,5]
Output: ["1->2->5","1->3"]

Example 2:

Input: root = [1]
Output: ["1"]



Constraints:

The number of nodes in the tree is in the range [1, 100].
-100 <= Node.val <= 100

*/
use crate::data_structures::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
    fn dfs(node: Option<&Rc<RefCell<TreeNode>>>, string: String, result: &mut Vec<String>) {
        if let Some(node) = node {
            let borrowed = node.borrow();
            let mut curr_path = string.clone();
            curr_path.push_str(&borrowed.val.to_string());
            if borrowed.left.as_ref().is_none() && borrowed.right.as_ref().is_none() {
                result.push(curr_path.clone());
            } else {
                curr_path.push_str("->");
                dfs(borrowed.left.as_ref(), curr_path.clone(), result);
                dfs(borrowed.right.as_ref(), curr_path.clone(), result);
            }
        }
    }
    let root = root.as_ref().unwrap();
    let mut result = Vec::new();
    dfs(Some(&root.clone()), String::new(), &mut result);
    result
}