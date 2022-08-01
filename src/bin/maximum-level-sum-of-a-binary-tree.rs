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
    pub fn traverse(
        root: Option<Rc<RefCell<TreeNode>>>,
        tier: usize,
        add: &mut dyn FnMut(i32, usize),
    ) {
        if let Some(root) = root {
            add(root.borrow().val, tier);
            Self::traverse(root.borrow().left.clone(), tier + 1, add);
            Self::traverse(root.borrow().right.clone(), tier + 1, add);
        }
    }

    pub fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut table = Vec::<i32>::new();

        let mut add = |v: i32, i: usize| {
            while i >= table.len() {
                table.push(0);
            }
            table[i] += v;
        };

        Self::traverse(root, 0, &mut add);

        let mut res = 0;

        let mut mv = i32::MIN;
        table.into_iter().enumerate().for_each(|(i, v)| {
            if v > mv {
                mv = v;
                res = i + 1;
            }
        });

        res as i32
    }
}

fn main() {
    let root = Some(Rc::new(RefCell::new(TreeNode::new(0))));

    println!("{}", Solution::max_level_sum(root));
}
