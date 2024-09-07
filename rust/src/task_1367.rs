/*Given a binary tree root and a linked list with head as the first node.

Return True if all the elements in the linked list starting from the head correspond to some downward path connected in the binary tree otherwise return False.

In this context downward path means a path that starts at some node and goes downwards.



Example 1:

Input: head = [4,2,8], root = [1,4,4,null,2,2,null,1,null,6,8,null,null,null,null,1,3]
Output: true
Explanation: Nodes in blue form a subpath in the binary Tree.

Example 2:

Input: head = [1,4,2,6], root = [1,4,4,null,2,2,null,1,null,6,8,null,null,null,null,1,3]
Output: true

Example 3:

Input: head = [1,4,2,6,8], root = [1,4,4,null,2,2,null,1,null,6,8,null,null,null,null,1,3]
Output: false
Explanation: There is no path in the binary tree that contains all the elements of the linked list from head.



Constraints:

The number of nodes in the tree will be in the range [1, 2500].
The number of nodes in the list will be in the range [1, 100].
1 <= Node.val <= 100 for each node in the linked list and binary tree.

*/
use crate::data_structures::list::ListNode;
use crate::data_structures::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn is_sub_path(head: Option<Box<ListNode>>, root: Option<Rc<RefCell<TreeNode>>>) -> bool {
	fn helper(head: Option<&Box<ListNode>>, root: Option<Rc<RefCell<TreeNode>>>) -> bool {
		if root.is_none() {
			return false;
		}

		if dfs(head, root.clone()) {
			return true;
		}
		let root = root.unwrap();
		let borrowed = root.borrow();
		helper(head, borrowed.left.clone()) || helper(head, borrowed.right.clone())
	}
	fn dfs(list: Option<&Box<ListNode>>, tree: Option<Rc<RefCell<TreeNode>>>) -> bool {
		match (list, tree) {
			(Some(list_node), Some(tree_node)) => {
				let tree_node = tree_node.borrow();
				if tree_node.val == list_node.val {
					if dfs(list_node.next.as_ref(), tree_node.left.clone()) || dfs(list_node.next.as_ref(), tree_node.right.clone()) {
						return true;
					}
				}
				false
			}

			(Some(_), None) => false,
			(None, _) => true,
		}
	}

	helper(head.as_ref(), root)
}

