// Given the root of a binary tree and two integers val and depth, add a row of nodes with value val at the given depth depth.
//
// Note that the root node is at depth 1.
//
// The adding rule is:
//
// Given the integer depth, for each not null tree node cur at the depth depth - 1, create two tree nodes with value val as cur's left subtree root and right subtree root.
// cur's original left subtree should be the left subtree of the new left subtree root.
// cur's original right subtree should be the right subtree of the new right subtree root.
// If depth == 1 that means there is no depth depth - 1 at all, then create a tree node with value val as the new root of the whole original tree, and the original tree is the new root's left subtree.
//
//
//
// Example 1:
//
// Input: root = [4,2,6,3,1,5], val = 1, depth = 2
// Output: [4,1,1,2,null,null,6,3,1,5]
//
// Example 2:
//
// Input: root = [4,2,null,3,1], val = 1, depth = 3
// Output: [4,2,null,1,1,3,null,null,1]


use std::cell::{Ref, RefCell};
use std::rc::Rc;
use crate::tree::{build_tree_from_vec, TreeNode};

pub fn add_one_row(root: Option<Rc<RefCell<TreeNode>>>, val: i32, depth: i32) -> Option<Rc<RefCell<TreeNode>>> {
    if depth == 1 {
        let new_root = Rc::new(RefCell::new(TreeNode::new(val)));
        new_root.borrow_mut().left = root;
        return Some(new_root);
    }

    fn helper(node: &Rc<RefCell<TreeNode>>, val: i32, depth: i32, curr_depth: i32) {
        if curr_depth == depth - 1 {
            let left_child = node.borrow_mut().left.take();
            let right_child = node.borrow_mut().right.take();

            let new_left = Rc::new(RefCell::new(TreeNode::new(val)));
            let new_right = Rc::new(RefCell::new(TreeNode::new(val)));

            new_left.borrow_mut().left = left_child;
            new_right.borrow_mut().right = right_child;

            node.borrow_mut().left = Some(new_left);
            node.borrow_mut().right = Some(new_right);
        } else {
            if let Some(ref left) = node.borrow().left {
                helper(left, val, depth, curr_depth + 1);
            }
            if let Some(ref right) = node.borrow().right {
                helper(right, val, depth, curr_depth + 1);
            }
        }
    }

    if let Some(ref root_node) = root {
        helper(root_node, val, depth, 1);
    }

    root
}



#[cfg(test)]

#[test]

fn test_add_one_row() {
    let tree = build_tree_from_vec(&[4,2,6,3,1,5]);
    let res = build_tree_from_vec(&[4,1,1,2,null,null,6,3,1,5]);
    assert_eq!(add_one_row(tree), )
}