/*1261. Find Elements in a Contaminated Binary Tree
Medium
Topics
Companies
Hint

Given a binary tree with the following rules:

root.val == 0
If treeNode.val == x and treeNode.left != null, then treeNode.left.val == 2 * x + 1
If treeNode.val == x and treeNode.right != null, then treeNode.right.val == 2 * x + 2

Now the binary tree is contaminated, which means all treeNode.val have been changed to -1.

Implement the FindElements class:

FindElements(TreeNode* root) Initializes the object with a contaminated binary tree and recovers it.
bool find(int target) Returns true if the target value exists in the recovered binary tree.



Example 1:

Input
["FindElements","find","find"]
[[[-1,null,-1]],[1],[2]]
Output
[null,false,true]
Explanation
FindElements findElements = new FindElements([-1,null,-1]);
findElements.find(1); // return False
findElements.find(2); // return True

Example 2:

Input
["FindElements","find","find","find"]
[[[-1,-1,-1,-1,-1]],[1],[3],[5]]
Output
[null,true,true,false]
Explanation
FindElements findElements = new FindElements([-1,-1,-1,-1,-1]);
findElements.find(1); // return True
findElements.find(3); // return True
findElements.find(5); // return False

Example 3:

Input
["FindElements","find","find","find","find"]
[[[-1,null,-1,-1,null,-1]],[2],[3],[4],[5]]
Output
[null,true,false,false,true]
Explanation
FindElements findElements = new FindElements([-1,null,-1,-1,null,-1]);
findElements.find(2); // return True
findElements.find(3); // return False
findElements.find(4); // return False
findElements.find(5); // return True



Constraints:

TreeNode.val == -1
The height of the binary tree is less than or equal to 20
The total number of nodes is between [1, 104]
Total calls of find() is between [1, 104]
0 <= target <= 106

*/

use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

use crate::data_structures::tree::TreeNode;

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
// struct FindElements {
//     root: Option<Rc<RefCell<TreeNode>>>,
//     set: HashSet<i32>
// }
//
//
// /**
//  * `&self` means the method takes an immutable reference.
//  * If you need a mutable reference, change it to `&mut self` instead.
//  */
// impl FindElements {
//
//     fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
//         let head = &root;
//         let mut set = std::collections::HashSet::new();
//         Self::dfs(root.as_ref(), &mut set, 0);
//         FindElements {
//             root: head.clone(),
//             set
//         }
//     }
//
//     fn dfs(node: Option<&Rc<RefCell<TreeNode>>>, set: &mut HashSet<i32>, val: i32) {
//         if let Some(node) = node {
//             let borrowed = node.borrow();
//             set.insert(val);
//             Self::dfs(borrowed.left.as_ref(), set, 2*val+1);
//             Self::dfs(borrowed.right.as_ref(), set, 2*val + 2);
//         }
//     }
//
//     fn find(&self, target: i32) -> bool {
//         self.set.contains(&target)
//     }
// }

/**
 * Your FindElements object will be instantiated and called as such:
 * let obj = FindElements::new(root);
 * let ret_1: bool = obj.find(target);
 */

struct FindElements {
    set: HashSet<i32>
}


impl FindElements {

    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut set = HashSet::new();
        if root.is_none() {
            return Self {
                set
            };
        }
        Self::dfs(root.as_ref().unwrap(), 0, &mut set);
        Self {
            set
        }
    }

    fn dfs(node: &Rc<RefCell<TreeNode>>, i: i32, set: &mut HashSet<i32>) {
        set.insert(i);
        let mut borrow = node.borrow();
        if let Some(left) = borrow.left.as_ref() {
            Self::dfs(left, 2*i + 1, set);
        }
        if let Some(right) = borrow.right.as_ref() {
            Self::dfs(right, 2*i + 2, set);
        }
    }

    fn find(&self, target: i32) -> bool {
        self.set.contains(&target)
    }
}