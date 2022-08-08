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
    pub fn add_one_row(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
        depth: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match depth {
            d if d <= 0 => root,
            1 => Some(Rc::new(RefCell::new(TreeNode {
                val,
                left: root,
                right: None,
            }))),
            2 => {
                if let Some(ptr) = root.clone() {
                    let mut node = ptr.borrow_mut();
                    node.left = Some(Rc::new(RefCell::new(TreeNode {
                        val,
                        left: node.left.clone(),
                        right: None,
                    })));
                    node.right = Some(Rc::new(RefCell::new(TreeNode {
                        val,
                        left: None,
                        right: node.right.clone(),
                    })))
                }
                root
            }
            _ => {
                if let Some(ptr) = root.clone() {
                    let node = ptr.borrow();
                    Self::add_one_row(node.left.clone(), val, depth - 1);
                    Self::add_one_row(node.right.clone(), val, depth - 1);
                }
                root
            }
        }
    }
}

fn main() {
    let mut root = Some(Rc::new(RefCell::new(TreeNode::new(0))));
    root = Solution::add_one_row(root, 1, 2);
    println!(
        "{} {}",
        root.clone()
            .unwrap()
            .borrow()
            .left
            .clone()
            .unwrap()
            .borrow()
            .val,
        root.clone()
            .unwrap()
            .borrow()
            .right
            .clone()
            .unwrap()
            .borrow()
            .val
    )
}
