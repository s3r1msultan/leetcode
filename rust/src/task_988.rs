// You are given the root of a binary tree where each node has a value in the range [0, 25] representing the letters 'a' to 'z'.
//
// Return the lexicographically smallest string that starts at a leaf of this tree and ends at the root.
//
// As a reminder, any shorter prefix of a string is lexicographically smaller.
//
// For example, "ab" is lexicographically smaller than "aba".
//
// A leaf of a node is a node that has no children.
//
//
//
// Example 1:
//
// Input: root = [0,1,2,3,4,3,4]
// Output: "dba"
//
// Example 2:
//
// Input: root = [25,1,3,1,3,0,2]
// Output: "adz"
//
// Example 3:
//
// Input: root = [2,2,1,null,1,0,null,0]
// Output: "abc"
//
//
//
// Constraints:
//
// The number of nodes in the tree is in the range [1, 8500].
// 0 <= Node.val <= 25
//


use std::cell::RefCell;
use std::rc::Rc;
use crate::tree::TreeNode;

pub fn smallest_from_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> String {
    let mut smallest = String::new();
    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, current_path: &mut Vec<char>, smallest: &mut String) {
        if let Some(n) = node {
            let n = n.borrow();

        }
    }
    let mut current_path = Vec::new();
    dfs(&root, &mut current_path, &mut smallest);

    smallest
}