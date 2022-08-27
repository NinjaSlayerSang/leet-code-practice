struct Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[allow(dead_code)]
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let n = n as usize;
        let mut refs = Vec::<&ListNode>::new();
        let mut pointer = head.as_deref();
        while let Some(p) = pointer {
            refs.push(p);
            pointer = p.next.as_deref();
        }
        let len = refs.len();

        if n < len {
            let p = unsafe { &mut *(refs[len - n - 1] as *const ListNode as *mut ListNode) };
            p.next = refs[len - n].next.to_owned();
            head
        } else {
            refs[0].next.to_owned()
        }
    }
}

fn debug(pointer: &Option<Box<ListNode>>) {
    if let Some(p) = pointer.as_deref() {
        print!("{} ", p.val);
        debug(&p.next);
    } else {
        println!();
    }
}

fn create(next: &[i32]) -> Option<Box<ListNode>> {
    if next.is_empty() {
        None
    } else {
        Some(Box::new(ListNode {
            val: next[0],
            next: create(&next[1..]),
        }))
    }
}

fn main() {
    let head = create(&vec![1, 2, 3, 4, 5]);
    let n = 3;
    debug(&Solution::remove_nth_from_end(head, n));
}
