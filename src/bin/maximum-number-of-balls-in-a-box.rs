struct Solution;

impl Solution {
    pub fn count_balls(low_limit: i32, high_limit: i32) -> i32 {
        use std::collections::HashMap;

        (low_limit..=high_limit)
            .into_iter()
            .fold(HashMap::new(), |mut m, i| {
                *(m.entry(
                    i.to_string()
                        .chars()
                        .fold(0, |r, d| r + d.to_digit(10).unwrap()),
                )
                .or_default()) += 1;
                m
            })
            .into_values()
            .max()
            .unwrap()
    }
}

fn main() {
    println!("{}", Solution::count_balls(19, 28));
}
