/*Given two integer arrays, preorder and postorder where preorder is the preorder traversal of a binary tree of distinct values and postorder is the postorder traversal of the same tree, reconstruct and return the binary tree.

If there exist multiple answers, you can return any of them.



Example 1:

Input: preorder = [1,2,4,5,3,6,7], postorder = [4,5,2,6,7,3,1]
Output: [1,2,3,4,5,6,7]

Example 2:

Input: preorder = [1], postorder = [1]
Output: [1]



Constraints:

1 <= preorder.length <= 30
1 <= preorder[i] <= preorder.length
All the values of preorder are unique.
postorder.length == preorder.length
1 <= postorder[i] <= postorder.length
All the values of postorder are unique.
It is guaranteed that preorder and postorder are the preorder traversal and postorder traversal of the same binary tree.

*/
use std::cell::RefCell;
use std::rc::Rc;
use crate::data_structures::tree::TreeNode;

pub fn construct_from_pre_post(preorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    fn dfs(i: &mut usize, j: &mut usize, preorder: &Vec<i32>, postorder: &Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let node = Rc::new(RefCell::new(TreeNode::new(preorder[*i])));
        *i += 1;

        if node.borrow().val != postorder[*j] {
            node.borrow_mut().left = dfs(i, j, preorder, postorder);
        }

        if node.borrow().val != postorder[*j] {
            node.borrow_mut().right = dfs(i, j, preorder, postorder);
        }

        *j += 1;

        Some(node)
    }
    dfs(&mut 0, &mut 0, &preorder, &postorder)
}