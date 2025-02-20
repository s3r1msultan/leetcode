/*Consider all the leaves of a binary tree, from left to right order, the values of those leaves form a leaf value sequence.

For example, in the given tree above, the leaf value sequence is (6, 7, 4, 9, 8).

Two binary trees are considered leaf-similar if their leaf value sequence is the same.

Return true if and only if the two given trees with head nodes root1 and root2 are leaf-similar.



Example 1:

Input: root1 = [3,5,1,6,2,9,8,null,null,7,4], root2 = [3,5,1,6,7,4,2,null,null,null,null,null,null,9,8]
Output: true

Example 2:

Input: root1 = [1,2,3], root2 = [1,3,2]
Output: false



Constraints:

The number of nodes in each tree will be in the range [1, 200].
Both of the given trees will have values in the range [0, 200].

*/
use crate::data_structures::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn leaf_similar(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn dfs(node: Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
        if let Some(node) = node {
            let borrowed = node.borrow();
            if borrowed.left.is_none() && borrowed.right.is_none() {
                result.push(borrowed.val);
                return;
            }

            if borrowed.left.is_some() {
                dfs(borrowed.left.clone(), result);
            }

            if borrowed.right.is_some() {
                dfs(borrowed.right.clone(), result);
            }
        }
    }
    let mut arr1 = vec![];
    let mut arr2 = vec![];
    dfs(root1, &mut arr1);
    dfs(root2, &mut arr2);
    arr1 == arr2
}