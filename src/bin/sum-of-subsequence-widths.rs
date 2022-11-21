struct Solution;

const M: i64 = 1000000007;

impl Solution {
    pub fn sum_subseq_widths(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        nums.sort();

        let mut power_2 = vec![-1i64; n];
        power_2[0] = 1;
        for i in 1..n {
            power_2[i] = (power_2[i - 1] * 2) % M;
        }

        let mut r = 0;

        for i in 0..n {
            r = (r + (power_2[i] - power_2[n - 1 - i]) * nums[i] as i64) % M
        }

        r as i32
    }
}

fn main() {
    println!("{}", Solution::sum_subseq_widths(vec![2, 1, 3]));
}
