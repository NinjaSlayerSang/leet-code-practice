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
    pub fn add(
        l: Option<Box<ListNode>>,
        r: Option<Box<ListNode>>,
        carry: i32,
    ) -> Option<Box<ListNode>> {
        match (l, r) {
            (None, None) => {
                if carry > 0 {
                    Some(Box::new(ListNode::new(carry)))
                } else {
                    None
                }
            }
            (Some(mut l), None) | (None, Some(mut l)) => {
                if carry > 0 {
                    let computed = l.val + carry;
                    l.val = computed % 10;
                    l.next = Self::add(l.next, None, computed / 10);
                }
                Some(l)
            }
            (Some(mut l), Some(r)) => {
                let computed = l.val + r.val + carry;
                l.val = computed % 10;
                l.next = Self::add(l.next, r.next, computed / 10);
                Some(l)
            }
        }
    }

    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        Self::add(l1, l2, 0)
    }
}

fn create(s: &str) -> Option<Box<ListNode>> {
    if s.is_empty() {
        None
    } else {
        Some(Box::new(ListNode {
            val: s[s.len() - 1..s.len()].parse::<i32>().unwrap(),
            next: create(&s[0..s.len() - 1]),
        }))
    }
}

fn print(n: Option<Box<ListNode>>) -> String {
    if let Some(node) = n {
        format!("{}{}", print(node.next), node.val)
    } else {
        String::new()
    }
}

fn main() {
    let a = create("342");
    let b = create("465");
    println!("{}", print(Solution::add_two_numbers(a, b)));
}
