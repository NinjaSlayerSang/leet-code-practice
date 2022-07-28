use rand::{prelude::SliceRandom, thread_rng, Rng};
use std::{cell::RefCell, rc::Rc};

const MAX_LEVEL: usize = 8;
const SKIPLIST_P: f64 = 0.25;

struct Node {
    val: i32,
    next: Vec<Option<Rc<RefCell<Node>>>>,
}

struct Skiplist {
    head: Rc<RefCell<Node>>,
    lvl: usize,
}

impl Node {
    fn new(val: i32, level: usize) -> Self {
        Self {
            val,
            next: vec![None; level + 1],
        }
    }

    fn level(&self) -> usize {
        self.next.len() - 1
    }

    fn next(pointer: Rc<RefCell<Self>>, level: usize) -> Option<Rc<RefCell<Self>>> {
        pointer.borrow().next.get(level).cloned().unwrap_or(None)
    }

    fn find(from: Rc<RefCell<Self>>, target: i32, level: usize, eq: bool) -> Rc<RefCell<Self>> {
        let mut pointer = from;

        loop {
            let next_opt = Self::next(pointer.clone(), level);
            if let Some(nexter) = next_opt {
                if if eq {
                    nexter.borrow().val <= target
                } else {
                    nexter.borrow().val < target
                } {
                    pointer = nexter;
                    continue;
                }
            }
            break;
        }

        pointer
    }
}

impl Skiplist {
    fn find(&self, target: i32, eq: bool) -> Vec<Rc<RefCell<Node>>> {
        let mut level = self.lvl;
        let mut result = Vec::<Rc<RefCell<Node>>>::with_capacity(level + 1);
        let mut pointer = Node::find(self.head.clone(), target, level, eq);
        result.push(pointer.clone());
        while level > 0 {
            level -= 1;
            pointer = Node::find(pointer, target, level, eq);
            result.push(pointer.clone());
        }
        result.reverse();
        result
    }

    fn random_level() -> usize {
        let mut level: usize = 0;
        while level < MAX_LEVEL && thread_rng().gen::<f64>() < SKIPLIST_P {
            level += 1;
        }
        level
    }

    fn debug(&self) {
        println!();
        for i in (0..=self.lvl).rev() {
            print!("[{}]: ", i);
            let mut pointer = self.head.clone();
            while let Some(nexter) = Node::next(pointer.clone(), i) {
                pointer = nexter;
                print!("{}({}) ", pointer.borrow().val, pointer.borrow().next.len());
            }
            println!();
        }
        println!();
    }
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Skiplist {
    fn new() -> Self {
        Self {
            head: Rc::new(RefCell::new(Node::new(i32::MIN, MAX_LEVEL))),
            lvl: MAX_LEVEL,
        }
    }

    fn search(&self, target: i32) -> bool {
        self.find(target, true)[0].borrow().val == target
    }

    fn add(&self, num: i32) {
        let level = Self::random_level();
        let new_node = Rc::new(RefCell::new(Node::new(num, level)));
        let pred = self.find(num, true);
        for i in 0..=level {
            new_node.borrow_mut().next[i] = pred[i].borrow_mut().next[i].clone();
            pred[i].borrow_mut().next[i] = Some(new_node.clone());
        }
    }

    fn erase(&self, num: i32) -> bool {
        let pred = self.find(num, false);
        let ptr_opt = pred[0].borrow().next[0].clone();
        if let Some(pointer) = ptr_opt {
            if pointer.borrow().val == num {
                let level = pointer.borrow().level();
                for i in 0..=level {
                    pred[i].borrow_mut().next[i] = pointer.borrow().next[i].clone();
                }
                true
            } else {
                false
            }
        } else {
            false
        }
    }
}

/**
 * Your Skiplist object will be instantiated and called as such:
 * let obj = Skiplist::new();
 * let ret_1: bool = obj.search(target);
 * obj.add(num);
 * let ret_3: bool = obj.erase(num);
 */

fn main() {
    let obj = Skiplist::new();

    let mut source = Vec::from_iter(0..10);

    let mut rng = thread_rng();
    source.shuffle(&mut rng);
    println!("source: {:?}", source);
    source.iter().for_each(|i| obj.add(*i));
    source.shuffle(&mut rng);
    println!("source: {:?}", source);
    source.iter().for_each(|i| obj.add(*i));

    obj.debug();

    obj.erase(5);

    obj.debug();

    obj.erase(2);
    obj.erase(5);
    obj.erase(8);

    obj.debug();

    println!("{}", obj.search(0));
    println!("{}", obj.search(4));
    println!("{}", obj.search(9));
    println!("{}", obj.search(-1));
    println!("{}", obj.search(5));
    println!("{}", obj.search(10));
}
