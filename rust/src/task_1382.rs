/*Given the root of a binary search tree, return a balanced binary search tree with the same node values. If there is more than one answer, return any of them.

A binary search tree is balanced if the depth of the two subtrees of every node never differs by more than 1.



Example 1:

Input: root = [1,null,2,null,3,null,4,null,null]
Output: [2,1,3,null,null,null,4]
Explanation: This is not the only correct answer, [3,1,4,null,2] is also correct.

Example 2:

Input: root = [2,1,3]
Output: [2,1,3]



Constraints:

The number of nodes in the tree is in the range [1, 104].
1 <= Node.val <= 105

*/

use std::cell::RefCell;
use std::rc::Rc;

use crate::tree::{build_tree_from_vec, TreeNode};

fn balance_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    fn inorder_traversal(root: &Option<Rc<RefCell<TreeNode>>>, values: &mut Vec<i32>) {
        if let Some(node) = root {
            let borrowed = node.borrow_mut();
            inorder_traversal(&borrowed.left, values);
            values.push(borrowed.val);
            inorder_traversal(&borrowed.right, values);
        }
    }

    fn sorted_array_into_bst(values: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if values.is_empty() {
            return None;
        }
        let mid = values.len() / 2;
        let root = Rc::new(RefCell::new(TreeNode::new(values[mid])));
        root.borrow_mut().left = sorted_array_into_bst(&values[..mid]);
        root.borrow_mut().right = sorted_array_into_bst(&values[mid + 1..]);
        return Some(root);
    }

    let mut values = vec![];
    inorder_traversal(&root, &mut values);
    let root = sorted_array_into_bst(&values);
    root
}

#[cfg(test)]
#[test]
fn test_balance_bst() {
    let root = build_tree_from_vec(vec![1, -1, 2, -1, 3, -1, 4, -1, -1]);
    let result = build_tree_from_vec(vec![2, 1, 3, -1, -1, -1, 4]);
    assert_eq!(balance_bst(root), result);
    let root = build_tree_from_vec(vec![2, 1, 3]);
    let result = build_tree_from_vec(vec![2, 1, 3]);
    assert_eq!(balance_bst(root), result);
}