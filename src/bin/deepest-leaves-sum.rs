use std::cell::RefCell;
use std::rc::Rc;

// Definition for a binary tree node.
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

struct Solution;

impl Solution {
    fn get_depth(root: Option<Rc<RefCell<TreeNode>>>) -> usize {
        if let Some(node) = root {
            usize::max(
                Self::get_depth(node.borrow().left.clone()),
                Self::get_depth(node.borrow().right.clone()),
            ) + 1
        } else {
            0
        }
    }

    fn get_sum(root: Option<Rc<RefCell<TreeNode>>>, depth: usize) -> i32 {
        if let Some(node) = root {
            if depth > 1 {
                Self::get_sum(node.borrow().left.clone(), depth - 1)
                    + Self::get_sum(node.borrow().right.clone(), depth - 1)
            } else {
                node.borrow().val
            }
        } else {
            0
        }
    }

    pub fn deepest_leaves_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let depth = Self::get_depth(root.clone());
        Self::get_sum(root, depth)
    }
}

fn main() {
    let build =
        |val: i32, left: Option<Rc<RefCell<TreeNode>>>, right: Option<Rc<RefCell<TreeNode>>>| {
            Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
        };

    let root = build(
        1,
        build(
            2,
            build(4, build(7, None, None), None),
            build(5, None, None),
        ),
        build(3, None, build(6, None, build(8, None, None))),
    );

    println!("{}", Solution::deepest_leaves_sum(root))
}
