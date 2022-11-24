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

trait Operation
where
    Self: Sized,
{
    fn joint(self, other: Self) -> Self;
    fn cut(self, nth: usize) -> (Self, Self, usize);
}

impl Operation for Option<Box<ListNode>> {
    fn joint(self, other: Self) -> Self {
        if let Some(mut node) = self {
            node.next = node.next.joint(other);
            Some(node)
        } else {
            other
        }
    }

    fn cut(self, nth: usize) -> (Self, Self, usize) {
        if nth > 0 {
            if let Some(mut node) = self {
                let (next, other, n) = node.next.cut(nth - 1);
                node.next = next;
                (Some(node), other, n + 1)
            } else {
                (self, None, 0)
            }
        } else {
            (None, self, 0)
        }
    }
}

impl Solution {
    fn reverse(head: Option<Box<ListNode>>, k: usize) -> Option<Box<ListNode>> {
        if k > 1 {
            let (head, remain, _) = head.cut(1);
            let (body, tail, _) = remain.cut(k - 2);
            tail.joint(Self::reverse(body, k - 2).joint(head))
        } else {
            head
        }
    }

    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let k = k as usize;
        let (group, remain, n) = head.cut(k);
        if n == k {
            Self::reverse(group, k).joint(Self::reverse_k_group(remain, k as i32))
        } else {
            group
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
        collect(&Solution::reverse_k_group(
            build(&vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]),
            4
        ))
    )
}
