/*Given the root of a binary tree, replace the value of each node in the tree with the sum of all its cousins' values.

Two nodes of a binary tree are cousins if they have the same depth with different parents.

Return the root of the modified tree.

Note that the depth of a node is the number of edges in the path from the root node to it.



Example 1:

Input: root = [5,4,9,1,10,null,7]
Output: [0,0,0,7,7,null,11]
Explanation: The diagram above shows the initial binary tree and the binary tree after changing the value of each node.
- Node with value 5 does not have any cousins so its sum is 0.
- Node with value 4 does not have any cousins so its sum is 0.
- Node with value 9 does not have any cousins so its sum is 0.
- Node with value 1 has a cousin with value 7 so its sum is 7.
- Node with value 10 has a cousin with value 7 so its sum is 7.
- Node with value 7 has cousins with values 1 and 10 so its sum is 11.

Example 2:

Input: root = [3,1,2]
Output: [0,0,0]
Explanation: The diagram above shows the initial binary tree and the binary tree after changing the value of each node.
- Node with value 3 does not have any cousins so its sum is 0.
- Node with value 1 does not have any cousins so its sum is 0.
- Node with value 2 does not have any cousins so its sum is 0.



Constraints:

The number of nodes in the tree is in the range [1, 105].
1 <= Node.val <= 104

*/
use crate::data_structures::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn replace_value_in_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    let root = if let Some(root) = root { root } else { return None };
    let mut level_sum = 0;
    let mut queue = vec![(root.clone(), 0)];
    while !queue.is_empty() {
        let mut new_queue = vec![];
        let mut new_level_sum = 0;
        for (node, sibling_sum) in queue.into_iter() {
            node.borrow_mut().val = level_sum - sibling_sum;
            let mut sibling_sum = 0;
            let mut non_leaf_children = vec![];
            if let Some(child) = node.borrow().left.clone() {
                sibling_sum += child.borrow().val;
                non_leaf_children.push(child);
            }
            if let Some(child) = node.borrow().right.clone() {
                sibling_sum += child.borrow().val;
                non_leaf_children.push(child);
            }
            new_level_sum += sibling_sum;
            non_leaf_children.into_iter()
                .for_each(|node| new_queue.push((node, sibling_sum)));
        }

        queue = new_queue;
        level_sum = new_level_sum;
    }

    Some(root)
}