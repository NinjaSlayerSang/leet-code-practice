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
    fn build(sorted: Vec<(usize, i32)>) -> Option<Rc<RefCell<TreeNode>>> {
        sorted.first().copied().map(|(i, val)| {
            Rc::new(RefCell::new(TreeNode {
                val,
                left: Self::build(sorted.iter().filter(|(k, _)| *k < i).copied().collect()),
                right: Self::build(sorted.iter().filter(|(k, _)| *k > i).copied().collect()),
            }))
        })
    }

    pub fn construct_maximum_binary_tree(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut nums = nums.into_iter().enumerate().collect::<Vec<_>>();
        nums.sort_by(|(ai, av), (bi, bv)| bv.cmp(av).then(ai.cmp(bi)));
        Self::build(nums)
    }
}

fn main() {
    let root = Solution::construct_maximum_binary_tree(vec![3, 2, 1, 6, 0, 5]);
    println!("{:#?}", root)
}
