/*Given two binary search trees root1 and root2, return a list containing all the integers from both trees sorted in ascending order.



Example 1:

Input: root1 = [2,1,4], root2 = [1,0,3]
Output: [0,1,1,2,3,4]

Example 2:

Input: root1 = [1,null,8], root2 = [8,1]
Output: [1,1,8,8]



Constraints:

The number of nodes in each tree is in the range [0, 5000].
-105 <= Node.val <= 105

*/

use crate::data_structures::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn get_all_elements(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    fn dfs(node: Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
        if let Some(node) = node {
            let borrowed = node.borrow();
            result.push(borrowed.val);
            dfs(borrowed.left.clone(), result);
            dfs(borrowed.right.clone(), result);
        }
    }

    let mut result = vec![];
    dfs(root1, &mut result);
    dfs(root2, &mut result);
    result.sort_unstable();
    result
}
