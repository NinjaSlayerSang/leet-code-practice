struct Solution;

impl Solution {
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        use std::{
            cmp::Ordering::{Equal, Greater, Less},
            collections::HashSet,
        };

        let len = nums.len();
        if len < 4 {
            return vec![];
        }
        nums.sort();
        let target = target as i64;
        let mut result = HashSet::<[i32; 4]>::new();
        for i in 0..len - 3 {
            for j in i + 1..len - 2 {
                let mut l = j + 1;
                let mut r = len - 1;
                while l < r {
                    let sum = nums[i] as i64 + nums[j] as i64 + nums[l] as i64 + nums[r] as i64;
                    match sum.cmp(&target) {
                        Less => {
                            l += 1;
                        }
                        Equal => {
                            result.insert([nums[i], nums[j], nums[l], nums[r]]);
                            l += 1;
                        }
                        Greater => {
                            r -= 1;
                        }
                    }
                }
            }
        }
        result.into_iter().map(Vec::from).collect()
    }
}

fn main() {
    println!("{:?}", Solution::four_sum(vec![1, 0, -1, 0, -2, 2], 0))
}
