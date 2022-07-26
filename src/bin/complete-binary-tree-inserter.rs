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
struct CBTInserter {
    root: Option<Rc<RefCell<TreeNode>>>,
    count: usize,
}

impl CBTInserter {
    fn count(root: Option<Rc<RefCell<TreeNode>>>) -> usize {
        match root {
            Some(ptr) => {
                1 + Self::count(ptr.borrow().left.clone()) + Self::count(ptr.borrow().right.clone())
            }
            None => 0,
        }
    }

    fn parse(i: usize) -> Vec<usize> {
        let mut i = i;
        let mut v = Vec::<usize>::new();
        while i > 1 {
            v.push(i % 2);
            i = i / 2;
        }
        v.reverse();
        v
    }
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl CBTInserter {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let count = Self::count(root.clone());
        Self { root, count }
    }

    fn insert(&mut self, val: i32) -> i32 {
        let new_node = Some(Rc::new(RefCell::new(TreeNode::new(val))));
        self.count += 1;
        let path = Self::parse(self.count);
        let mut pointer = self.root.clone();
        for k in &path[0..path.len() - 1] {
            match *k {
                0 => pointer = pointer.unwrap().borrow_mut().left.clone(),
                1 => pointer = pointer.unwrap().borrow_mut().right.clone(),
                _ => {}
            }
        }
        let result = pointer.clone().unwrap().borrow().val;
        match *path.last().unwrap() {
            0 => pointer.unwrap().borrow_mut().left = new_node,
            1 => pointer.unwrap().borrow_mut().right = new_node,
            _ => {}
        }
        result
    }

    fn get_root(&self) -> Option<Rc<RefCell<TreeNode>>> {
        self.root.clone()
    }
}

/**
 * Your CBTInserter object will be instantiated and called as such:
 * let obj = CBTInserter::new(root);
 * let ret_1: i32 = obj.insert(val);
 * let ret_2: Option<Rc<RefCell<TreeNode>>> = obj.get_root();
 */

fn main() {
    let root = Some(Rc::new(RefCell::new(TreeNode::new(0))));

    let mut obj = CBTInserter::new(root);

    obj.insert(1);

    let result = obj.get_root();

    print!("{}", result == None);
}
