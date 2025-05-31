/*Given the root of a binary tree, find the maximum value v for which there exist different nodes a and b where v = |a.val - b.val| and a is an ancestor of b.

A node a is an ancestor of b if either: any child of a is equal to b or any child of a is an ancestor of b.



Example 1:

Input: root = [8,3,10,1,6,null,14,null,null,4,7,13]
Output: 7
Explanation: We have various ancestor-node differences, some of which are given below :
|8 - 3| = 5
|3 - 7| = 4
|8 - 1| = 7
|10 - 13| = 3
Among all possible differences, the maximum value of 7 is obtained by |8 - 1| = 7.

Example 2:

Input: root = [1,null,2,null,0,3]
Output: 3



Constraints:

The number of nodes in the tree is in the range [2, 5000].
0 <= Node.val <= 105

*/
use crate::data_structures::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn dfs(node: Option<&Rc<RefCell<TreeNode>>>, min: i32, max: i32) -> i32 {
        if node.is_none() {
            return 0;
        }

        let node = node.unwrap();
        let borrowed = node.borrow();

        let max_diff = (min - borrowed.val).abs().max((max - borrowed.val).abs());

        let min = min.min(borrowed.val);
        let max = max.max(borrowed.val);

        let left = dfs(borrowed.left.as_ref(), min, max);
        let right = dfs(borrowed.right.as_ref(), min, max);


        [left, right, max_diff].into_iter().max().unwrap()
    }

    if root.is_none() {
        return 0;
    }

    let root = root.unwrap();
    let borrowed = root.borrow();

    dfs(borrowed.left.as_ref(), borrowed.val, borrowed.val).max(dfs(borrowed.right.as_ref(), borrowed.val, borrowed.val))
}