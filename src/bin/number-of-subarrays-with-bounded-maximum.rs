struct Solution;

impl Solution {
    pub fn num_subarray_bounded_max(nums: Vec<i32>, left: i32, right: i32) -> i32 {
        let mut r = 0;

        let mut lm = -1;
        let mut lc = -1;
        for (i, v) in nums.into_iter().enumerate() {
            if v > right {
                lm = i as i32;
            } else {
                if v >= left {
                    lc = i as i32;
                }
                if lm < lc {
                    r += lc - lm;
                }
            }
        }

        r
    }
}

fn main() {
    println!(
        "{}",
        Solution::num_subarray_bounded_max(vec![2, 9, 2, 5, 6], 2, 8)
    );
}
