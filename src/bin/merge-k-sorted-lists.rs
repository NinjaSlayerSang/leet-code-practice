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

use std::{cmp::Ordering, collections::BinaryHeap};

impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.val.partial_cmp(&self.val)
    }
}

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.val.cmp(&self.val)
    }
}

impl Solution {
    fn merge(mut heap: BinaryHeap<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        if let Some(Some(mut head)) = heap.pop() {
            let next = head.next;
            heap.push(next);
            head.next = Self::merge(heap);
            Some(head)
        } else {
            None
        }
    }

    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        Self::merge(BinaryHeap::from(lists))
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

fn build_lists(list: Vec<Vec<i32>>) -> Vec<Option<Box<ListNode>>> {
    list.iter().map(|v| build_list(v)).collect()
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
    print_list(Solution::merge_k_lists(build_lists(vec![
        vec![1, 4, 5],
        vec![1, 3, 4],
        vec![2, 6],
    ])));
}
