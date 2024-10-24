/*You are given the root of a binary tree and a positive integer k.

The level sum in the tree is the sum of the values of the nodes that are on the same level.

Return the kth largest level sum in the tree (not necessarily distinct). If there are fewer than k levels in the tree, return -1.

Note that two nodes are on the same level if they have the same distance from the root.



Example 1:

Input: root = [5,8,9,2,1,3,7,4,6], k = 2
Output: 13
Explanation: The level sums are the following:
- Level 1: 5.
- Level 2: 8 + 9 = 17.
- Level 3: 2 + 1 + 3 + 7 = 13.
- Level 4: 4 + 6 = 10.
The 2nd largest level sum is 13.

Example 2:

Input: root = [1,2,null,3], k = 1
Output: 3
Explanation: The largest level sum is 3.



Constraints:

The number of nodes in the tree is n.
2 <= n <= 105
1 <= Node.val <= 106
1 <= k <= n

*/
use std::cell::RefCell;
use std::rc::Rc;
use crate::data_structures::tree::TreeNode;

pub fn kth_largest_level_sum(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i64 {
    use std::collections::VecDeque;
    let k = k as usize;
    let mut queue = VecDeque::new();
    let mut level_sums = vec![];
    queue.push_back(root.unwrap());
    while !queue.is_empty() {
        let n = queue.len();
        let mut level_sum = 0i64;
        for _ in 0..n {
            let node = queue.pop_front().unwrap();
            let borrowed = node.borrow();
            level_sum += borrowed.val as i64;

            if let Some(left_node) = borrowed.left.as_ref() {
                queue.push_back(left_node.clone());
            }

            if let Some(right_node) = borrowed.right.as_ref() {
                queue.push_back(right_node.clone());
            }
        }
        level_sums.push(level_sum);
    }
    if k > level_sums.len() {
        return -1;
    }
    level_sums.sort_unstable_by(|a,b| b.cmp(&a));
    level_sums[k-1]
}