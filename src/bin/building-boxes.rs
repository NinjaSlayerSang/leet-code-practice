struct Solution;

impl Solution {
    fn binary_search_ft(range: (i32, i32), predicate: &dyn Fn(i32) -> bool) -> i32 {
        if range.0 >= range.1 {
            range.1
        } else {
            let mid = (range.0 + range.1) / 2;
            if predicate(mid) {
                Self::binary_search_ft((range.0, mid), predicate)
            } else {
                Self::binary_search_ft((mid + 1, range.1), predicate)
            }
        }
    }

    fn f(mut m: i32) -> i32 {
        let mut t = 0;
        while m >= t + 1 {
            t += 1;
            m -= t;
        }
        (1..=t).into_iter().fold(0, |r, i| r + i * (t + 1 - i)) + m * (m + 1) / 2
    }

    pub fn minimum_boxes(n: i32) -> i32 {
        Self::binary_search_ft((0, 1650467), &|m| Solution::f(m) >= n)
    }
}

fn main() {
    println!("{}", Solution::minimum_boxes(103368013));
}
