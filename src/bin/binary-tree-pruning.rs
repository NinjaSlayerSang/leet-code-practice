struct Solution;

// Definition for a binary tree node.
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

impl Solution {
    pub fn prune_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node_ptr) = root.clone() {
            let mut node = node_ptr.borrow_mut();
            node.left = Self::prune_tree(node.left.clone());
            node.right = Self::prune_tree(node.right.clone());
            if node.left == None && node.right == None && node.val == 0 {
                return None;
            }
        }
        root
    }
}

fn main() {
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 0,
        left: None,
        right: None,
    })));

    let result = Solution::prune_tree(root);

    print!("{}", result == None);
}
