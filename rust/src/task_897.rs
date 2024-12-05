/*Given the root of a binary search tree, rearrange the tree in in-order so that the leftmost node in the tree is now the root of the tree, and every node has no left child and only one right child.



Example 1:

Input: root = [5,3,6,2,4,null,8,1,null,null,null,7,9]
Output: [1,null,2,null,3,null,4,null,5,null,6,null,7,null,8,null,9]

Example 2:

Input: root = [5,1,7]
Output: [1,null,5,null,7]



Constraints:

The number of nodes in the given tree will be in the range [1, 100].
0 <= Node.val <= 1000

*/
use crate::data_structures::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn increasing_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    fn dfs(node: Option<Rc<RefCell<TreeNode>>>, tail: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = node {
            let mut borrowed = node.borrow_mut();

            let tree = dfs(borrowed.left.take(), Some(node.clone()));
            borrowed.right = dfs(borrowed.right.take(), tail);
            return tree;
        }
        tail
    }

    dfs(root, None)
}

