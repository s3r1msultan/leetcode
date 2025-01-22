/*You are given the root of a binary tree with unique values.

In one operation, you can choose any two nodes at the same level and swap their values.

Return the minimum number of operations needed to make the values at each level sorted in a strictly increasing order.

The level of a node is the number of edges along the path between it and the root node.



Example 1:

Input: root = [1,4,3,7,6,8,5,null,null,null,null,9,null,10]
Output: 3
Explanation:
- Swap 4 and 3. The 2nd level becomes [3,4].
- Swap 7 and 5. The 3rd level becomes [5,6,8,7].
- Swap 8 and 7. The 3rd level becomes [5,6,7,8].
We used 3 operations so return 3.
It can be proven that 3 is the minimum number of operations needed.

Example 2:

Input: root = [1,3,2,7,6,5,4]
Output: 3
Explanation:
- Swap 3 and 2. The 2nd level becomes [2,3].
- Swap 7 and 4. The 3rd level becomes [4,6,5,7].
- Swap 6 and 5. The 3rd level becomes [4,5,6,7].
We used 3 operations so return 3.
It can be proven that 3 is the minimum number of operations needed.

Example 3:

Input: root = [1,2,3,4,5,6]
Output: 0
Explanation: Each level is already sorted in increasing order so return 0.



Constraints:

The number of nodes in the tree is in the range [1, 105].
1 <= Node.val <= 105
All the values of the tree are unique.

*/
use crate::data_structures::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn minimum_operations(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if root.is_none() {
        return 0;
    }
    let mut count = 0;

    let mut queue = std::collections::VecDeque::new();
    queue.push_back(root.unwrap());

    while !queue.is_empty() {
        let n = queue.len();
        let mut unsorted = vec![];
        let mut map = std::collections::HashMap::new();
        for i in 0..n {
            let node = queue.pop_front().unwrap();
            let borrowed = node.borrow();
            unsorted.push(borrowed.val);
            map.insert(borrowed.val, i);

            if let Some(left) = borrowed.left.as_ref() {
                queue.push_back(left.clone());
            }

            if let Some(right) = borrowed.right.as_ref() {
                queue.push_back(right.clone());
            }
        }

        let mut sorted = unsorted.clone();
        sorted.sort_unstable();

        for i in 0..n {
            if unsorted[i] != sorted[i] {
                let j = *map.get(&sorted[i]).unwrap();
                unsorted.swap(i, j);
                *map.entry(unsorted[i]).or_insert(i) = i;
                *map.entry(unsorted[j]).or_insert(j) = j;
                count += 1;
            }
        }
    }

    count
}