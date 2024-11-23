/*Given the root of a binary search tree and an integer k, return true if there exist two elements in the BST such that their sum is equal to k, or false otherwise.



Example 1:

Input: root = [5,3,6,2,4,null,7], k = 9
Output: true

Example 2:

Input: root = [5,3,6,2,4,null,7], k = 28
Output: false



Constraints:

The number of nodes in the tree is in the range [1, 104].
-104 <= Node.val <= 104
root is guaranteed to be a valid binary search tree.
-105 <= k <= 105

*/

use std::cell::RefCell;
use std::rc::Rc;
use crate::data_structures::tree::TreeNode;

pub fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
    let mut queue = std::collections::VecDeque::new();
    let mut set =  std::collections::HashSet::new();
    queue.push_back(root.unwrap());

    while !queue.is_empty() {
        let n = queue.len();
        for i in 0..n {
            let node = queue.pop_front().unwrap();
            let borrowed = node.borrow();
            if set.contains(&(k - borrowed.val)) {
                return true;
            }
            set.insert(borrowed.val);
            if let Some(left) = borrowed.left.as_ref() {
                queue.push_back(left.clone());
            }

            if let Some(right) = borrowed.right.clone() {
                queue.push_back(right.clone());
            }
        }
    }

    false
}