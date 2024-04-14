use std::cell::RefCell;
use std::rc::Rc;

struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>
}

impl TreeNode {
    pub fn new(val:i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None
        }
    }
}

pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    helper(root.as_ref(), false)
}

fn helper(node: Option<&Rc<RefCell<TreeNode>>>, is_left: bool) -> i32 {
    match node {
        Some(node) => {
            let node = node.borrow();
            if node.left.is_none() && node.right.is_none() {
                if is_left {
                    node.val
                } else {
                    0
                }
            } else {
                helper(node.left.as_ref(), true) + helper(node.right.as_ref(), false)
            }
        },
        None => 0
    }
}