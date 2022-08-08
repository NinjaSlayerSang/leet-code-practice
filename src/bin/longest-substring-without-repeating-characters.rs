struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        use std::collections::HashMap;

        let len = s.len();
        let mut pred = vec![-1; len];
        let mut map = HashMap::<char, i32>::new();
        for (i, c) in s.chars().enumerate() {
            if let Some(p) = map.get(&c) {
                pred[i] = *p;
            }
            map.insert(c, i as i32);
        }

        let mut f = vec![1; len];
        for i in 1..len {
            f[i] = i32::min(f[i - 1] + 1, i as i32 - pred[i])
        }

        f.into_iter().max().unwrap_or(0)
    }
}

fn main() {
    println!(
        "{}",
        Solution::length_of_longest_substring("abcabcbb".to_string())
    )
}
