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

impl BinarySearchByPredicate for Vec<(i64, usize)> {}

struct Solution;

impl Solution {
    pub fn shortest_subarray(nums: Vec<i32>, k: i32) -> i32 {
        let len = nums.len();
        let k = k as i64;

        let mut r = -1;
        let mut sum = 0;
        let mut stack = vec![(sum, 0)];

        for j in 0..len {
            sum = sum + nums[j] as i64;
            let p = stack.binary_search_by_predicate(0..stack.len(), &|&(s, _)| s > sum - k);
            if p > 0 {
                let (_, i) = stack[p - 1];
                let d = (j + 1 - i) as i32;
                r = if r > 0 { r.min(d) } else { d }
            }
            let p = stack.binary_search_by_predicate(0..stack.len(), &|&(s, _)| s >= sum);
            let now = (sum, j + 1);
            if p < stack.len() {
                stack[p] = now;
                stack.truncate(p + 1);
            } else {
                stack.push(now);
            }
        }

        r
    }
}

fn main() {
    println!("{}", Solution::shortest_subarray(vec![2, -1, 2], 3));
}
