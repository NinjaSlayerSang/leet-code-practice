struct Solution;

impl Solution {
    pub fn wiggle_sort(nums: &mut Vec<i32>) {
        let mid = nums.len() / 2;
        nums.sort_by(|a, b| b.cmp(a));
        *nums = (0..nums.len())
            .map(|i| nums[i / 2 + (i & 1 == 0) as usize * mid])
            .collect::<Vec<_>>();
    }
}

fn main() {
    let mut input = vec![1, 5, 1, 1, 6, 4];

    Solution::wiggle_sort(&mut input);

    print!("{:?}", input);
}
