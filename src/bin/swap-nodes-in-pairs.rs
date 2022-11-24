// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution;

impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if let Some(mut a) = head {
            if let Some(mut b) = a.next {
                a.next = Self::swap_pairs(b.next);
                b.next = Some(a);
                return Some(b);
            } else {
                Some(a)
            }
        } else {
            head
        }
    }
}

fn build(v: &Vec<i32>) -> Option<Box<ListNode>> {
    v.iter().rev().fold(None, |next, val| {
        Some(Box::new(ListNode { val: *val, next }))
    })
}

fn collect(head: &Option<Box<ListNode>>) -> Vec<i32> {
    let mut v = Vec::new();
    let mut h = head.as_deref();
    while let Some(p) = h {
        v.push(p.val);
        h = p.next.as_deref();
    }
    v
}

fn main() {
    println!(
        "{:?}",
        collect(&Solution::swap_pairs(build(&vec![1, 2, 3, 4, 5, 6, 7])))
    );
}
