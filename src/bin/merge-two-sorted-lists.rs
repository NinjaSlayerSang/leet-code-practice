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
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        use std::mem::swap;
        match (list1, list2) {
            (None, None) => None,
            (None, Some(p)) | (Some(p), None) => Some(p),
            (Some(mut l), Some(mut r)) => {
                if l.val > r.val {
                    swap(&mut l, &mut r)
                }
                let next = l.next;
                l.next = Self::merge_two_lists(next, Some(r));
                Some(l)
            }
        }
    }
}

fn build_list(v: &[i32]) -> Option<Box<ListNode>> {
    if v.is_empty() {
        None
    } else {
        Some(Box::new(ListNode {
            val: v[0],
            next: build_list(&v[1..]),
        }))
    }
}

fn print_list(list: Option<Box<ListNode>>) {
    if let Some(n) = list {
        print!("{} ", n.val);
        print_list(n.next);
    } else {
        println!();
    }
}

fn main() {
    print_list(Solution::merge_two_lists(
        build_list(&vec![1, 2, 4]),
        build_list(&vec![1, 3, 4]),
    ));
}
