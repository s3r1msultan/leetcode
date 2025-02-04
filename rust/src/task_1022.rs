/*You are given the root of a binary tree where each node has a value 0 or 1. Each root-to-leaf path represents a binary number starting with the most significant bit.

For example, if the path is 0 -> 1 -> 1 -> 0 -> 1, then this could represent 01101 in binary, which is 13.

For all leaves in the tree, consider the numbers represented by the path from the root to that leaf. Return the sum of these numbers.

The test cases are generated so that the answer fits in a 32-bits integer.



Example 1:

Input: root = [1,0,1,0,1,0,1]
Output: 22
Explanation: (100) + (101) + (110) + (111) = 4 + 5 + 6 + 7 = 22

Example 2:

Input: root = [0]
Output: 0



Constraints:

The number of nodes in the tree is in the range [1, 1000].
Node.val is 0 or 1.

*/
use std::cell::RefCell;
use std::rc::Rc;
use crate::data_structures::tree::TreeNode;

pub fn sum_root_to_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if root.is_none() {
        return 0;
    }

    fn dfs(node: Option<&Rc<RefCell<TreeNode>>>) -> Vec<String> {
        if node.is_none() {
            return vec![];
        }

        let node = node.unwrap();
        let borrowed = node.borrow();
        let bit = if borrowed.val == 0 {
            "0"
        } else {
            "1"
        };

        if borrowed.left.is_none() && borrowed.right.is_none() {
            return vec![bit.to_string()];
        }

        let mut result = vec![];
        if borrowed.left.is_some() {
            result.append(&mut dfs(borrowed.left.as_ref()));
        }

        if borrowed.right.is_some() {
            result.append(&mut dfs(borrowed.right.as_ref()));
        }

        result.into_iter().map(|bin| bit.to_owned() + bin.as_str()).collect()
    }

    let bin_vector = dfs(root.as_ref());

    bin_vector.into_iter().map(|bin| i32::from_str_radix(&bin,2).unwrap()).sum()
}