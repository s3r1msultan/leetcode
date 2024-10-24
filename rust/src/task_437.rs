/*Given the root of a binary tree and an integer targetSum, return the number of paths where the sum of the values along the path equals targetSum.

The path does not need to start or end at the root or a leaf, but it must go downwards (i.e., traveling only from parent nodes to child nodes).



Example 1:

Input: root = [10,5,-3,3,2,null,11,3,-2,null,1], targetSum = 8
Output: 3
Explanation: The paths that sum to 8 are shown.

Example 2:

Input: root = [5,4,8,11,null,13,4,7,2,null,null,5,1], targetSum = 22
Output: 3



Constraints:

The number of nodes in the tree is in the range [0, 1000].
-109 <= Node.val <= 109
-1000 <= targetSum <= 1000

*/
use crate::data_structures::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
    pub fn dfs(node: Option<&Rc<RefCell<TreeNode>>>, target_sum: i32, current_sum: i64) -> i32 {
        if node.is_none() {
            return 0;
        }

        let node = node.unwrap();
        let borrowed = node.borrow();
        let current_sum = current_sum + borrowed.val as i64;

        let mut count = if target_sum as i64 == current_sum { 1 } else { 0 };

        count += dfs(borrowed.left.as_ref(), target_sum, current_sum);
        count += dfs(borrowed.right.as_ref(), target_sum, current_sum);

        count
    }

    pub fn count_paths(node: Option<&Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        if node.is_none() {
            return 0;
        }

        let node = node.unwrap();

        let mut count = dfs(Some(node), target_sum, 0);

        count += count_paths(node.borrow().left.as_ref(), target_sum);
        count += count_paths(node.borrow().right.as_ref(), target_sum);

        count
    }

    count_paths(root.as_ref(), target_sum)
}