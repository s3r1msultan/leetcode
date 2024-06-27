/*Given a binary tree, determine if it is height-balanced.



Example 1:

Input: root = [3,9,20,null,null,15,7]
Output: true

Example 2:

Input: root = [1,2,2,3,3,null,null,4,4]
Output: false

Example 3:

Input: root = []
Output: true



Constraints:

The number of nodes in the tree is in the range [0, 5000].
-104 <= Node.val <= 104

*/

use std::cell::RefCell;
use std::rc::Rc;

use crate::tree::{build_tree_from_vec, TreeNode};

fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn check_is_balanced(root: &Option<Rc<RefCell<TreeNode>>>) -> (i32, bool) {
        if let Some(node) = root {
            let borrowed = node.borrow();
            let (left_height, left_is_balanced) = check_is_balanced(&borrowed.left);
            let (right_height, right_is_balanced) = check_is_balanced(&borrowed.right);

            let current_height = left_height.max(right_height) + 1;
            let is_balanced = right_is_balanced && left_is_balanced && left_height.abs_diff(right_height) <= 1;
            return (current_height, is_balanced);
        } else {
            return (0, true);
        }
    }

    let (_, is_balanced) = check_is_balanced(&root);
    is_balanced
}

#[cfg(test)]
#[test]
fn test_is_balanced() {
    let root = build_tree_from_vec(vec![3, 9, 20, -1, -1, 15, 7]);
    let result = true;
    assert_eq!(is_balanced(root), result);

    let root = build_tree_from_vec(vec![1, 2, 2, 3, 3, -1, -1, 4, 4]);
    let result = false;
    assert_eq!(is_balanced(root), result);

    let root = build_tree_from_vec(vec![]);
    let result = true;
    assert_eq!(is_balanced(root), result);
}
