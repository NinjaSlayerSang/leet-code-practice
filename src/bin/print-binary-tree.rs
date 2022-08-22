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
    fn height(root: Option<Rc<RefCell<TreeNode>>>) -> usize {
        if let Some(node) = root {
            usize::max(
                Self::height(node.borrow().left.clone()),
                Self::height(node.borrow().right.clone()),
            ) + 1
        } else {
            0
        }
    }

    fn traverse(
        root: Option<Rc<RefCell<TreeNode>>>,
        r: usize,
        c: usize,
        height: usize,
        rec: &mut Vec<Vec<String>>,
    ) {
        if let Some(node) = root {
            rec[r][c] = node.borrow().val.to_string();
            if r < height - 1 {
                let i = 2usize.pow((height - r - 2) as u32);
                Self::traverse(node.borrow().left.clone(), r + 1, c - i, height, rec);
                Self::traverse(node.borrow().right.clone(), r + 1, c + i, height, rec);
            }
        }
    }

    pub fn print_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<String>> {
        let height = Self::height(root.clone());
        let row = height;
        let col = 2usize.pow(height as u32) - 1;
        let mut result = vec![vec!["".to_string(); col]; row];
        Self::traverse(root, 0, (col - 1) / 2, height, &mut result);
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
        build(
            2,
            build(4, build(7, None, None), None),
            build(5, None, None),
        ),
        build(3, None, build(6, None, build(8, None, None))),
    );

    let mut result = Solution::print_tree(root);
    result.iter_mut().for_each(|r| {
        r.iter_mut().for_each(|c| {
            if c.is_empty() {
                c.push_str("*")
            }
        });
        println!("{}", r.join(" "))
    })
}
