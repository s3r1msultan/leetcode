/*We run a preorder depth-first search (DFS) on the root of a binary tree.

At each node in this traversal, we output D dashes (where D is the depth of this node), then we output the value of this node.  If the depth of a node is D, the depth of its immediate child is D + 1.  The depth of the root node is 0.

If a node has only one child, that child is guaranteed to be the left child.

Given the output traversal of this traversal, recover the tree and return its root.



Example 1:

Input: traversal = "1-2--3--4-5--6--7"
Output: [1,2,5,3,4,6,7]

Example 2:

Input: traversal = "1-2--3---4-5--6---7"
Output: [1,2,5,3,null,6,null,4,null,7]

Example 3:

Input: traversal = "1-401--349---90--88"
Output: [1,401,null,349,88,90]



Constraints:

The number of nodes in the original tree is in the range [1, 1000].
1 <= Node.val <= 109

*/
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
use crate::data_structures::tree::TreeNode;

pub fn recover_from_preorder(traversal: String) -> Option<Rc<RefCell<TreeNode>>> {
    fn dfs(i: &mut usize, depth: i32, chars: &[u8]) -> Option<Rc<RefCell<TreeNode>>> {
        let mut dashes = 0;
        while *i < chars.len() && chars[*i] == b'-' {
            dashes += 1;
            *i += 1;
        }

        if dashes != depth {
            *i -= dashes as usize;
            return None;
        }

        let mut num = 0;

        while *i < chars.len() && chars[*i].is_ascii_alphanumeric() {
            num *= 10;
            num += (chars[*i] - b'0') as i32;
            *i += 1;
        }

        let new_node = Rc::new(RefCell::new(TreeNode::new(num)));

        new_node.borrow_mut().left = dfs(i, depth+1, chars);
        new_node.borrow_mut().right = dfs(i, depth+1, chars);

        Some(new_node)
    }
    let chars = traversal.as_bytes();
    dfs(&mut 0, 0, &chars)
}
