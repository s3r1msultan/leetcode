/*
Given the root of a binary tree, return the zigzag level order traversal of its nodes' values. (i.e., from left to right, then right to left for the next level and alternate between).



Example 1:

Input: root = [3,9,20,null,null,15,7]
Output: [[3],[20,9],[15,7]]

Example 2:

Input: root = [1]
Output: [[1]]

Example 3:

Input: root = []
Output: []
Constraints:

The number of nodes in the tree is in the range [0, 2000].
-100 <= Node.val <= 100*/

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
use crate::data_structures::tree::TreeNode;

pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if root.is_none() {
            return vec![];
        }

        let mut result = Vec::new();
        let mut queue = std::collections::VecDeque::new();
        queue.push_back((root.unwrap(), 0));
        while !queue.is_empty() {

            let n = queue.len();
            let mut sub_queue = std::collections::VecDeque::new();
            for _ in 0..n {
                let (node, depth) = queue.pop_front().unwrap();
                let borrowed = node.borrow();


                if depth % 2 == 0 {
                    sub_queue.push_back(borrowed.val);
                } else {
                    sub_queue.push_front(borrowed.val);
                }

                if let Some(left) = borrowed.left.as_ref() {
                    queue.push_back((left.clone(), depth+1));
                }

                if let Some(right) = borrowed.right.as_ref() {
                    queue.push_back((right.clone(), depth+1));
                }
            }

            result.push(sub_queue.into_iter().collect());

        }

        result
    }
