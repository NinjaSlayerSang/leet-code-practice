use std::ops::{Index, Range};

trait BinarySearchByPredicate: Index<usize> {
    fn binary_search_by_predicate(
        &self,
        range: Range<usize>,
        predicate: &dyn Fn(&<Self as Index<usize>>::Output) -> bool,
    ) -> usize {
        if range.is_empty() {
            range.end
        } else {
            let mid = (range.start + range.end) / 2;
            if predicate(&self[mid]) {
                self.binary_search_by_predicate(range.start..mid, predicate)
            } else {
                self.binary_search_by_predicate((mid + 1)..range.end, predicate)
            }
        }
    }
}

impl BinarySearchByPredicate for Vec<usize> {}

struct StockSpanner {
    list: Vec<i32>,
    stack: Vec<usize>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl StockSpanner {
    pub fn new() -> Self {
        Self {
            list: vec![i32::MAX],
            stack: vec![0],
        }
    }

    pub fn next(&mut self, price: i32) -> i32 {
        self.list.push(price);
        let p = self.list.len() - 1;
        let i = self
            .stack
            .binary_search_by_predicate(0..self.stack.len(), &|i| self.list[*i] <= price);
        if i < self.stack.len() {
            self.stack[i] = p;
            self.stack.truncate(i + 1);
        } else {
            self.stack.push(p);
        }
        (self.stack[i] - self.stack[i - 1]) as i32
    }
}

/**
 * Your StockSpanner object will be instantiated and called as such:
 * let obj = StockSpanner::new();
 * let ret_1: i32 = obj.next(price);
 */

fn main() {
    let mut obj = StockSpanner::new();
    let queue = [100, 80, 60, 70, 60, 75, 85];
    let result = queue.map(|v| obj.next(v));
    println!("{:?}", result);
}
