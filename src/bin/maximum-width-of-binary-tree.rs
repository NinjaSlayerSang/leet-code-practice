struct Solution;

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

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn traverse(root: Option<Rc<RefCell<TreeNode>>>, t: usize, k: i64, rec: &mut Vec<(i64, i64)>) {
        if let Some(node) = root {
            if t == rec.len() {
                rec.push((k, k));
            } else {
                let (l, r) = rec[t];
                rec[t] = (k.min(l), k.max(r));
            }
            Self::traverse(node.borrow().left.clone(), t + 1, 2 * k, rec);
            Self::traverse(node.borrow().right.clone(), t + 1, 2 * k + 1, rec);
        }
    }

    pub fn width_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut rec = Vec::<(i64, i64)>::new();
        Self::traverse(root, 0, 1, &mut rec);
        let mut result = 0;
        for (l, r) in rec {
            result = result.max(r - l + 1);
        }
        result as i32
    }
}

fn build(
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
}

fn main() {
    let root = build(
        1,
        build(3, build(5, build(6, None, None), None), None),
        build(2, None, build(9, build(7, None, None), None)),
    );

    println!("{}", Solution::width_of_binary_tree(root))
}
