use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
	pub val: i32,
	pub left: Option<Rc<RefCell<TreeNode>>>,
	pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
	#[inline]
	pub fn new(val: i32) -> Self {
		TreeNode {
			val,
			left: None,
			right: None,
		}
	}
}

pub fn build_tree_from_vec(vals: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
	if vals.is_empty() {
		return None;
	}

	let root = Rc::new(RefCell::new(TreeNode::new(vals[0])));
	let mut nodes = vec![root.clone()];
	let mut current_index = 0;

	while current_index < nodes.len() {
		let node = nodes[current_index].clone();
		let left_idx = 2 * current_index + 1;
		if left_idx < vals.len() {
			if vals[left_idx] == -1 {
				node.borrow_mut().left = None;
			} else {
				let left_node = Rc::new(RefCell::new(TreeNode::new(vals[left_idx])));
				node.borrow_mut().left = Some(left_node.clone());
				nodes.push(left_node);
			}
		}

		let right_idx = 2 * current_index + 2;
		if right_idx < vals.len() {
			if vals[right_idx] == -1 {
				node.borrow_mut().right = None;
			} else {
				let right_node = Rc::new(RefCell::new(TreeNode::new(vals[right_idx])));
				node.borrow_mut().right = Some(right_node.clone());
				nodes.push(right_node);
			}
		}

		current_index += 1;
	}

	Some(root)
}

fn tree_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
	if let Some(node) = root {
		let borrow = node.borrow();
		let left_depth = tree_depth(borrow.left.clone());
		let right_depth = tree_depth(borrow.right.clone());
		return left_depth.max(right_depth) + 1;
	}
	0
}

fn sum_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
	if let Some(node) = root {
		let borrow = node.borrow();
		let left_sum = sum_tree(borrow.left.clone());
		let right_sum = sum_tree(borrow.right.clone());
		return left_sum + right_sum + borrow.val;
	}
	0
}


fn find_value(root: Option<Rc<RefCell<TreeNode>>>, value: i32) -> bool {
	if let Some(node) = root {
		let borrowed = node.borrow();
		if borrowed.val == value {
			return true;
		}
		let left_is_found = find_value(borrowed.left.clone(), value);
		if left_is_found {
			return true;
		}
		let right_is_found = find_value(borrowed.right.clone(), value);
		return right_is_found;
	}
	false
}

fn clone_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
	if let Some(node) = root {
		let borrowed = node.borrow();
		let mut new_tree_node = Rc::new(RefCell::new(TreeNode::new(borrowed.val)));
		new_tree_node.borrow_mut().left = clone_tree(borrowed.left.clone());
		new_tree_node.borrow_mut().right = clone_tree(borrowed.right.clone());
		Some(new_tree_node)
	} else {
		None
	}
}