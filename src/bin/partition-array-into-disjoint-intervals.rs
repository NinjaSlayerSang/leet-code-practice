struct Solution;

impl Solution {
    pub fn partition_disjoint(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let mut l = 0;
        let mut lm = nums[0];
        let mut m = lm;
        for i in 1..len - 1 {
            m = m.max(nums[i]);
            if nums[i] < lm {
                l = i;
                lm = m;
            }
        }
        (l + 1) as i32
    }
}

fn main() {
    println!("{}", Solution::partition_disjoint(vec![1, 1, 1, 0, 6, 12]))
}
