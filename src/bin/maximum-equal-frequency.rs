struct Solution;

impl Solution {
    pub fn max_equal_freq(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;

        let len = nums.len();
        let mut stat = HashMap::<i32, usize>::new();
        let mut check = HashMap::<usize, usize>::new();
        let mut result = 0;
        for i in 0..len {
            let n = nums[i];
            let m = stat.entry(n).or_default();
            if *m > 0 {
                let c = check.get_mut(m).unwrap();
                if *c > 1 {
                    *c -= 1;
                } else {
                    check.remove(m);
                }
            }
            *m += 1;
            *check.entry(*m).or_default() += 1;
            match check.len() {
                1 => {
                    let (m, c) = check.iter().nth(0).unwrap();
                    if *m == 1 || *c == 1 {
                        result = i + 1;
                    }
                }
                2 => {
                    let mut v = check.iter().collect::<Vec<_>>();
                    v.sort();
                    if *v[0].0 == 1 && *v[0].1 == 1 || *v[1].0 - *v[0].0 == 1 && *v[1].1 == 1 {
                        result = i + 1;
                    }
                }
                _ => {}
            }
        }
        result as i32
    }
}

fn main() {
    println!(
        "{}",
        Solution::max_equal_freq(vec![1, 1, 1, 2, 2, 2, 3, 3, 3, 4, 4, 4, 5])
    )
}
