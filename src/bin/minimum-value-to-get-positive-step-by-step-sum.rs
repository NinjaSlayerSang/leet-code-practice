struct Solution;

impl Solution {
    pub fn min_start_value(nums: Vec<i32>) -> i32 {
        let mut r = 0;
        -nums
            .into_iter()
            .map(|i| {
                r += i;
                r
            })
            .min()
            .unwrap_or(0)
            .min(0)
            + 1
    }
}

fn main() {
    println!("{}", Solution::min_start_value(vec![-3, 2, -3, 4, 2]))
}
