struct Solution;

impl Solution {
    pub fn max_product(mut nums: Vec<i32>) -> i32 {
        let len = nums.len();
        nums.sort();
        (nums[len - 2] - 1) * (nums[len - 1] - 1)
    }
}

fn main() {
    println!("{}", Solution::max_product(vec![3, 4, 5, 2]))
}
