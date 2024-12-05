/*Given the root of a binary tree, return the preorder traversal of its nodes' values.



Example 1:

Input: root = [1,null,2,3]

Output: [1,2,3]

Explanation:

Example 2:

Input: root = [1,2,3,4,5,null,8,null,null,6,7,9]

Output: [1,2,4,5,6,7,3,8,9]

Explanation:

Example 3:

Input: root = []

Output: []

Example 4:

Input: root = [1]

Output: [1]



Constraints:

The number of nodes in the tree is in the range [0, 100].
-100 <= Node.val <= 100



Follow up: Recursive solution is trivial, could you do it iteratively?
*/
use crate::data_structures::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    fn dfs(node: Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
        if let Some(node) = node {
            let borrowed = node.borrow();
            result.push(borrowed.val);
            dfs(borrowed.left.clone(), result);
            dfs(borrowed.right.clone(), result);
        }
    }
    let mut result = vec![];
    result
}