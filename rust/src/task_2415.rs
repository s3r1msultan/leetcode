/*Given the root of a perfect binary tree, reverse the node values at each odd level of the tree.

For example, suppose the node values at level 3 are [2,1,3,4,7,11,29,18], then it should become [18,29,11,7,4,3,1,2].

Return the root of the reversed tree.

A binary tree is perfect if all parent nodes have two children and all leaves are on the same level.

The level of a node is the number of edges along the path between it and the root node.



Example 1:

Input: root = [2,3,5,8,13,21,34]
Output: [2,5,3,8,13,21,34]
Explanation:
The tree has only one odd level.
The nodes at level 1 are 3, 5 respectively, which are reversed and become 5, 3.

Example 2:

Input: root = [7,13,11]
Output: [7,11,13]
Explanation:
The nodes at level 1 are 13, 11, which are reversed and become 11, 13.

Example 3:

Input: root = [0,1,2,0,0,0,0,1,1,1,1,2,2,2,2]
Output: [0,2,1,0,0,0,0,2,2,2,2,1,1,1,1]
Explanation:
The odd levels have non-zero values.
The nodes at level 1 were 1, 2, and are 2, 1 after the reversal.
The nodes at level 3 were 1, 1, 1, 1, 2, 2, 2, 2, and are 2, 2, 2, 2, 1, 1, 1, 1 after the reversal.



Constraints:

The number of nodes in the tree is in the range [1, 214].
0 <= Node.val <= 105
root is a perfect binary tree.

*/
use crate::data_structures::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn reverse_odd_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    if root.is_none() {
        return None;
    }
    let mut root = root.unwrap();

    use std::collections::VecDeque;
    let mut queue = VecDeque::new();
    let mut is_odd = false;
    queue.push_back(root.clone());
    while !queue.is_empty() {
        let n = queue.len();
        let mut current_level = vec![];
        for _ in 0..n {
            let node = queue.pop_front().unwrap();
            current_level.push(node.clone());
            let borrowed_mut = node.borrow();
            if let Some(left_node) = borrowed_mut.left.as_ref() {
                queue.push_back(left_node.clone());
            }

            if let Some(right_node) = borrowed_mut.right.as_ref() {
                queue.push_back(right_node.clone());
            }
        }
        if is_odd {
            let mut i = 0;
            let mut j = current_level.len() - 1;
            while i < j {
                let mut left_node = current_level[i].borrow_mut();
                let mut right_node = current_level[j].borrow_mut();
                std::mem::swap(&mut left_node.val, &mut right_node.val);
                i += 1;
                j -= 1;
            }
        }
        is_odd = !is_odd;
    }

    Some(root)
}