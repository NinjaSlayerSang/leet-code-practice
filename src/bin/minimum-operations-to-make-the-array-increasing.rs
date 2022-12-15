struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut k = 0;
        let mut r = 0;
        for i in nums.into_iter() {
            if i <= k {
                k += 1;
                r += k - i;
            } else {
                k = i;
            }
        }
        r
    }
}

fn main() {
    println!("{}", Solution::min_operations(vec![1, 5, 2, 4, 1]));
}
