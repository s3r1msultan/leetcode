/*Given the root of a Binary Search Tree (BST), convert it to a Greater Tree such that every key of the original BST is changed to the original key plus the sum of all keys greater than the original key in BST.

As a reminder, a binary search tree is a tree that satisfies these constraints:

The left subtree of a node contains only nodes with keys less than the node's key.
The right subtree of a node contains only nodes with keys greater than the node's key.
Both the left and right subtrees must also be binary search trees.



Example 1:

Input: root = [4,1,6,0,2,5,7,null,null,null,3,null,null,null,8]
Output: [30,36,21,36,35,26,15,null,null,null,33,null,null,null,8]

Example 2:

Input: root = [0,null,1]
Output: [1,null,1]



Constraints:

The number of nodes in the tree is in the range [1, 100].
0 <= Node.val <= 100
All the values in the tree are unique.
*/

use std::cell::RefCell;
use std::rc::Rc;

use crate::tree::{build_tree_from_vec, TreeNode};

fn bst_to_gst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    fn traverse(root: &Option<Rc<RefCell<TreeNode>>>, sum: &mut i32) {
        if let Some(head) = root {
            let mut borrowed = head.borrow_mut();
            traverse(&borrowed.right, sum);
            *sum += borrowed.val;
            borrowed.val = *sum;
            traverse(&borrowed.left, sum);
        }
    }

    let mut sum = 0;
    traverse(&root, &mut sum);
    return root;
}

#[cfg(test)]
#[test]
fn test_bst_to_gst() {
    let root = build_tree_from_vec(vec![4, 1, 6, 0, 2, 5, 7, -1, -1, -1, 3, -1, -1, -1, 8]);
    let result = build_tree_from_vec(vec![30, 36, 21, 36, 35, 26, 15, -1, -1, -1, 33, -1, -1, -1, 8]);
    assert_eq!(bst_to_gst(root), result);
    let root = build_tree_from_vec(vec![0, -1, 1]);
    let result = build_tree_from_vec(vec![1, -1, 1]);
    assert_eq!(bst_to_gst(root), result);
}