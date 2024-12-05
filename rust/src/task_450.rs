/*Given a root node reference of a BST and a key, delete the node with the given key in the BST. Return the root node reference (possibly updated) of the BST.

Basically, the deletion can be divided into two stages:

Search for a node to remove.
If the node is found, delete the node.



Example 1:

Input: root = [5,3,6,2,4,null,7], key = 3
Output: [5,4,6,2,null,null,7]
Explanation: Given key to delete is 3. So we find the node with value 3 and delete it.
One valid answer is [5,4,6,2,null,null,7], shown in the above BST.
Please notice that another valid answer is [5,2,6,null,4,null,7] and it's also accepted.

Example 2:

Input: root = [5,3,6,2,4,null,7], key = 0
Output: [5,3,6,2,4,null,7]
Explanation: The tree does not contain a node with value = 0.

Example 3:

Input: root = [], key = 0
Output: []



Constraints:

The number of nodes in the tree is in the range [0, 104].
-105 <= Node.val <= 105
Each node has a unique value.
root is a valid binary search tree.
-105 <= key <= 105



Follow up: Could you solve it with time complexity O(height of tree)?
*/
use crate::data_structures::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn delete_node(root: Option<Rc<RefCell<TreeNode>>>, key: i32) -> Option<Rc<RefCell<TreeNode>>> {
    fn rec(node1: Option<Rc<RefCell<TreeNode>>>, node2: Option<Rc<RefCell<TreeNode>>>, key: i32, is_found: bool) -> Option<Rc<RefCell<TreeNode>>> {
        match (node1, node2) {
            (None, None) => { None }
            (None, Some(node)) | (Some(node), None) => {
                if is_found { Some(node) } else {
                    let value = node.borrow().val;
                    if value == key { return rec(node.borrow().left.clone(), node.borrow().right.clone(), key, true); }
                    {
                        let mut node = node.borrow_mut();
                        if value < key { node.right = rec(None, node.right.clone(), key, false); }
                        if value > key { node.left = rec(None, node.left.clone(), key, false); }
                    }
                    Some(node)
                }
            }
            (left, Some(right)) => {
                {
                    let mut right = right.borrow_mut();
                    right.left = rec(left, right.left.clone(), key, is_found);
                }
                Some(right)
            }
        }
    }
    rec(None, root, key, false)
}