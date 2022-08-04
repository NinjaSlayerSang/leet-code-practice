struct Solution;

impl Solution {
    pub fn min_subsequence(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort_by(|a, b| b.cmp(a));

        let mut standard = 0;
        for n in nums.iter() {
            standard += n
        }
        standard /= 2;

        let cal = || {
            let mut sum = 0;
            for (i, v) in nums.iter().enumerate() {
                sum += v;
                if sum > standard {
                    return i;
                }
            }
            0
        };

        nums[0..=cal()].iter().copied().collect::<Vec<_>>()
    }
}

fn main() {
    println!("{:?}", Solution::min_subsequence(vec![4, 3, 10, 9, 8]));
}
