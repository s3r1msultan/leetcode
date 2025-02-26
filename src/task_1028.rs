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
use std::cell::{Ref, RefCell};
use std::rc::Rc;
use crate::data_structures::tree::TreeNode;

pub fn recover_from_preorder(traversal: String) -> Option<Rc<RefCell<TreeNode>>> {
    fn dfs(node: Option<&Rc<RefCell<TreeNode>>>, depth: i32, i: usize, bytes: &[u8]) -> usize {
        if node.is_none() {
            return i;
        }

        let mut i = i;
        let mut dashes = 0;
        while bytes[i] == b'-' {
            i += 1;
            dashes += 1;
        }

        if dashes < depth {
            return i - dashes as usize;
        }

        let mut num = 0;
        while bytes[i].is_ascii_alphanumeric() {
            num *= 10;
            num += (bytes[i] - b'0') as i32;
            i += 1;
        }

        let mut borrow_mut = node.borrow_mut();
        borrow_mut.val = num;


        i = dfs(borrow_mut.left.as_ref(), depth + 1, i, bytes);
        dfs(borrow_mut.right.as_ref(), depth + 1, i, bytes)

    }
    let root = Rc::new(RefCell::new(TreeNode::new(-1)));
    dfs(&root, 0, 0, traversal.as_bytes());
    Some(root)
}