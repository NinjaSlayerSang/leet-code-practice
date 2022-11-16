struct Solution;

use std::collections::HashSet;

impl Solution {
    fn f(nums: &Vec<i32>, m: &mut Vec<Vec<Option<HashSet<i32>>>>, i: usize, j: usize) -> Vec<i32> {
        if let Some(s) = &mut m[i][j] {
            s.iter().copied().collect()
        } else {
            let s = if i < j {
                HashSet::new()
            } else if j == 0 {
                nums[0..=i].iter().copied().collect()
            } else {
                Self::f(nums, m, i - 1, j)
                    .into_iter()
                    .chain(
                        Self::f(nums, m, i - 1, j - 1)
                            .into_iter()
                            .map(|x| x + nums[i]),
                    )
                    .collect()
            };
            let v = s.iter().copied().collect();
            m[i][j] = Some(s);
            v
        }
    }

    pub fn split_array_same_average(mut nums: Vec<i32>) -> bool {
        let n = nums.len();
        let t = nums.iter().sum::<i32>();

        nums.sort();
        nums.iter_mut().for_each(|x| *x *= n as i32);

        let mut m: Vec<Vec<Option<HashSet<i32>>>> = vec![vec![None; n / 2]; n];

        for j in 0..n / 2 {
            Self::f(&nums, &mut m, n - 1, j);
            if m[n - 1][j]
                .as_ref()
                .unwrap()
                .contains(&((j + 1) as i32 * t))
            {
                return true;
            }
        }

        false
    }
}

fn main() {
    println!(
        "{}",
        Solution::split_array_same_average(vec![1, 2, 3, 4, 5, 6, 7, 8])
    )
}
