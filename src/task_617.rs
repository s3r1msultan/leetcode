/*You are given two binary trees root1 and root2.

Imagine that when you put one of them to cover the other, some nodes of the two trees are overlapped while the others are not. You need to merge the two trees into a new binary tree. The merge rule is that if two nodes overlap, then sum node values up as the new value of the merged node. Otherwise, the NOT null node will be used as the node of the new tree.

Return the merged tree.

Note: The merging process must start from the root nodes of both trees.



Example 1:

Input: root1 = [1,3,2,5], root2 = [2,1,3,null,4,null,7]
Output: [3,4,5,5,4,null,7]

Example 2:

Input: root1 = [1], root2 = [1,2]
Output: [2,2]



Constraints:

The number of nodes in both trees is in the range [0, 2000].
-104 <= Node.val <= 104

*/
use crate::data_structures::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn merge_trees(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    if root1.is_none() && root2.is_none() {
        return None;
    }

    if root1.is_some() && root2.is_some() {
        let mut node = TreeNode::new(0);
        let root1 = root1.unwrap();
        let root2 = root2.unwrap();
        node.val += root1.borrow().val & root2.borrow().val;
        node.left = merge_trees(root1.borrow().left.clone(), root2.borrow().left.clone());
        node.right = merge_trees(root1.borrow().right.clone(), root2.borrow().right.clone());
        Some(Rc::new(RefCell::new(node)))
    } else if root1.is_some() {
        root1
    } else {
        root2
    }
}