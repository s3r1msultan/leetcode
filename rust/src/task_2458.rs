<<<<<<< HEAD
/*You are given the root of a binary tree with n nodes. Each node is assigned a unique value from 1 to n. You are also given an array queries of size m.

You have to perform m independent queries on the tree where in the ith query you do the following:

Remove the subtree rooted at the node with the value queries[i] from the tree. It is guaranteed that queries[i] will not be equal to the value of the root.

Return an array answer of size m where answer[i] is the height of the tree after performing the ith query.

Note:

The queries are independent, so the tree returns to its initial state after each query.
The height of a tree is the number of edges in the longest simple path from the root to some node in the tree.



Example 1:

Input: root = [1,3,4,2,null,6,5,null,null,null,null,null,7], queries = [4]
Output: [2]
Explanation: The diagram above shows the tree after removing the subtree rooted at node with value 4.
The height of the tree is 2 (The path 1 -> 3 -> 2).

Example 2:

Input: root = [5,8,9,2,1,3,7,4,6], queries = [3,2,4,8]
Output: [3,2,3,2]
Explanation: We have the following queries:
- Removing the subtree rooted at node with value 3. The height of the tree becomes 3 (The path 5 -> 8 -> 2 -> 4).
- Removing the subtree rooted at node with value 2. The height of the tree becomes 2 (The path 5 -> 8 -> 1).
- Removing the subtree rooted at node with value 4. The height of the tree becomes 3 (The path 5 -> 8 -> 2 -> 6).
- Removing the subtree rooted at node with value 8. The height of the tree becomes 2 (The path 5 -> 9 -> 3).



Constraints:

The number of nodes in the tree is n.
2 <= n <= 105
1 <= Node.val <= n
All the values in the tree are unique.
m == queries.length
1 <= m <= min(n, 104)
1 <= queries[i] <= n
queries[i] != root.val

*/
use crate::data_structures::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn tree_queries(root: Option<Rc<RefCell<TreeNode>>>, queries: Vec<i32>) -> Vec<i32> {
    use std::collections::HashMap;
    fn dfs(node: Option<&Rc<RefCell<TreeNode>>>, depth: i32, heights: &mut HashMap<i32, i32>, depths: &mut HashMap<i32, i32>) -> i32 {
        if let Some(node) = node {
            let borrowed = node.borrow();
            depths.insert(borrowed.val, depth);

            let left_height = dfs(borrowed.left.as_ref(), depth + 1, heights, depths);
            let right_height = dfs(borrowed.right.as_ref(), depth + 1, heights, depths);
            let height = left_height.max(right_height) + 1;
            heights.insert(borrowed.val, height);
            height
        } else {
            -1
        }
    }

    let mut heights = HashMap::new();
    let mut depths = HashMap::new();

    dfs(root.as_ref(), 0, &mut heights, &mut depths);

    let mut max_depth_height = HashMap::new();

    for (&val, &h) in heights.iter() {
        let d = depths[&val];
        max_depth_height.entry(d).or_insert_with(Vec::new).push((h, val));
    }

    for heights in max_depth_height.values_mut() {
        heights.sort_unstable_by(|a, b| b.0.cmp(&a.0));
        if heights.len() > 2 {
            heights.truncate(2);
        }
    }

    let mut result = vec![0; queries.len()];
    for query in queries {
        let depth = depths[&query];
        let heights = &max_depth_height[&depth];

        let max_height = if heights.len() == 1 {
            if heights[0].1 == query {
                depth - 1
            } else {
                heights[0].0 + depth
            }
        } else {
            if heights[0].1 == query {
                heights[1].0 + depth
            } else {
                heights[0].0 + depth
            }
        };

        result.push(max_height);
    }


    result
}
=======
/*You are given the root of a binary tree with n nodes. Each node is assigned a unique value from 1 to n. You are also given an array queries of size m.

You have to perform m independent queries on the tree where in the ith query you do the following:

Remove the subtree rooted at the node with the value queries[i] from the tree. It is guaranteed that queries[i] will not be equal to the value of the root.

Return an array answer of size m where answer[i] is the height of the tree after performing the ith query.

Note:

The queries are independent, so the tree returns to its initial state after each query.
The height of a tree is the number of edges in the longest simple path from the root to some node in the tree.



Example 1:

Input: root = [1,3,4,2,null,6,5,null,null,null,null,null,7], queries = [4]
Output: [2]
Explanation: The diagram above shows the tree after removing the subtree rooted at node with value 4.
The height of the tree is 2 (The path 1 -> 3 -> 2).

Example 2:

Input: root = [5,8,9,2,1,3,7,4,6], queries = [3,2,4,8]
Output: [3,2,3,2]
Explanation: We have the following queries:
- Removing the subtree rooted at node with value 3. The height of the tree becomes 3 (The path 5 -> 8 -> 2 -> 4).
- Removing the subtree rooted at node with value 2. The height of the tree becomes 2 (The path 5 -> 8 -> 1).
- Removing the subtree rooted at node with value 4. The height of the tree becomes 3 (The path 5 -> 8 -> 2 -> 6).
- Removing the subtree rooted at node with value 8. The height of the tree becomes 2 (The path 5 -> 9 -> 3).



Constraints:

The number of nodes in the tree is n.
2 <= n <= 105
1 <= Node.val <= n
All the values in the tree are unique.
m == queries.length
1 <= m <= min(n, 104)
1 <= queries[i] <= n
queries[i] != root.val

*/

use std::cell::RefCell;
use std::rc::Rc;
use crate::data_structures::tree::TreeNode;

/*pub fn tree_queries(root: Option<Rc<RefCell<TreeNode>>>, queries: Vec<i32>) -> Vec<i32> {
    fn find_depth(node: Option<&Rc<RefCell<TreeNode>>>, depth: i32, query: i32) -> i32 {
        if let Some(node) = node {
            let borrowed = node.borrow();
            if borrowed.val == query
                return depth;
            }

            return find_depth(borrowed.left.as_ref(), depth + 1, query).max(find_depth(borrowed.right.as_ref(), depth + 1, query));
        }

        depth
    }

    let mut result = vec![];
    for query in queries {
        result.push(find_depth(root.as_ref(), 0, query));
    }
    result
}*/
>>>>>>> 2276d2c53b5e5f30758ccd90b506909ec90665d7
