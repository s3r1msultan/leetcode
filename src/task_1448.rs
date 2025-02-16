/*Given a binary tree root, a node X in the tree is named good if in the path from root to X there are no nodes with a value greater than X.

Return the number of good nodes in the binary tree.



Example 1:

Input: root = [3,1,4,3,null,1,5]
Output: 4
Explanation: Nodes in blue are good.
Root Node (3) is always a good node.
Node 4 -> (3,4) is the maximum value in the path starting from the root.
Node 5 -> (3,4,5) is the maximum value in the path
Node 3 -> (3,1,3) is the maximum value in the path.

Example 2:

Input: root = [3,3,null,4,2]
Output: 3
Explanation: Node 2 -> (3, 3, 2) is not good, because "3" is higher than it.

Example 3:

Input: root = [1]
Output: 1
Explanation: Root is considered as good.



Constraints:

The number of nodes in the binary tree is in the range [1, 10^5].
Each node's value is between [-10^4, 10^4].
*/

// pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
//     fn dfs(node: Option<&Rc<RefCell<TreeNode>>>, max: i32) -> i32 {
//         if let Some(node) = node {
//             let borrowed = node.borrow();
//             return if borrowed.val >= max {
//                 let max = borrowed.val;
//                 dfs(borrowed.left.as_ref(), max) + dfs(borrowed.right.as_ref(), max) + 1
//             } else {
//                 dfs(borrowed.left.as_ref(), max) + dfs(borrowed.right.as_ref(), max)
//             }
//         }
//         0
//     }
//
//     dfs(root.as_ref(), i32::MIN)
// }

/*pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    use std::collections::VecDeque;
    let mut queue = VecDeque::new();
    queue.push_back((i32::MIN, root));
    let mut count = 0;
    while !queue.is_empty() {
        let (max, node) = queue.pop_front().unwrap();
        if node.is_none() {
            continue;
        }
        let node = node.unwrap();
        let borrowed = node.borrow();

        let mut val = borrowed.val;

        if val >= max {
            val = max;
            count += 1;
        }

        queue.push_back((val, borrowed.left.clone()));
        queue.push_back((val, borrowed.right.clone()));
    }
    count
}*/
use crate::data_structures::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn dfs(root: Option<&Rc<RefCell<TreeNode>>>, max: i32) -> i32 {
        if root.is_none() { return 0; }
        let mut max = max;
        let node = root.unwrap();
        let borrowed = node.borrow();
        let mut count = 0;
        if borrowed.val >= max {
            max = borrowed.val;
            count += 1;
        }
        if borrowed.left.is_some() {
            count += dfs(borrowed.left.as_ref(), max);
        }

        if borrowed.right.is_some() {
            count += dfs(borrowed.right.as_ref(), max);
        }

        count
    }

    dfs(root.as_ref(), i32::MIN)
}