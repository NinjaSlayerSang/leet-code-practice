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

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn check(node: Option<Rc<RefCell<TreeNode>>>, val: i32) -> bool {
        if let Some(node) = node {
            node.borrow().val == val
        } else {
            false
        }
    }

    fn traverse(root: Option<Rc<RefCell<TreeNode>>>, m: &mut i32) -> i32 {
        if let Some(root) = root {
            let root = root.borrow();
            let val = root.val;

            let tl = Self::traverse(root.left.clone(), m);
            let tr = Self::traverse(root.right.clone(), m);

            let ml = if Self::check(root.left.clone(), val) {
                tl + 1
            } else {
                0
            };
            let mr = if Self::check(root.right.clone(), val) {
                tr + 1
            } else {
                0
            };

            *m = i32::max(*m, ml + mr);

            i32::max(ml, mr)
        } else {
            0
        }
    }

    pub fn longest_univalue_path(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut result = 0;
        Self::traverse(root, &mut result);
        result
    }
}

fn main() {
    let build =
        |val: i32, left: Option<Rc<RefCell<TreeNode>>>, right: Option<Rc<RefCell<TreeNode>>>| {
            Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
        };

    let root = build(
        1,
        build(4, build(4, None, None), build(4, None, None)),
        build(5, None, build(5, None, None)),
    );

    println!("{}", Solution::longest_univalue_path(root));
}
