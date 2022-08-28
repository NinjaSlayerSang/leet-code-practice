struct Solution;

impl Solution {
    fn count_factorial_divisor_5(mut x: i64) -> i64 {
        let mut r = 0;
        while x > 0 {
            x = x / 5;
            r += x;
        }
        r
    }

    fn search(l: i64, r: i64, k: &i64) -> bool {
        use std::cmp::Ordering::{Equal, Greater, Less};

        if l < r {
            let mid = (l + r) / 2;
            match Self::count_factorial_divisor_5(mid).cmp(k) {
                Less => Self::search(mid + 1, r, k),
                Equal => true,
                Greater => Self::search(l, mid, k),
            }
        } else {
            false
        }
    }

    pub fn preimage_size_fzf(k: i32) -> i32 {
        let k = k as i64;
        if Self::search(1, 5000000000, &k) {
            5
        } else {
            0
        }
    }
}

fn main() {
    println!("{}", Solution::preimage_size_fzf(11))
}
