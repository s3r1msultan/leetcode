/*// Definition for a binary tree node.
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
impl Solution {
    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {

    }
}*/

    use std::cell::RefCell;
    use std::rc::Rc;
    use crate::data_structures::tree::TreeNode;

    fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(node: Option<Rc<RefCell<TreeNode>>>, min_diff: &mut i32, prev: &mut Option<Rc<RefCell<TreeNode>>>) {
            if let Some(node) = node {
                let borrowed = node.borrow();
                dfs(borrowed.left.clone(), min_diff, prev);
                if let Some(prev) = prev {
                    *min_diff = (*min_diff).min(borrowed.val - prev.borrow().val);
                }
                *prev = Some(node);
                dfs(borrowed.right.clone(), min_diff, prev);
            }
        }

        let mut min_diff = i32::MAX;
        let mut prev = None;
        dfs(root, &mut min_diff, &mut prev);
        min_diff
    }