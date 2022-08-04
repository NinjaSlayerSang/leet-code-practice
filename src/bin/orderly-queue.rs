struct Solution;

impl Solution {
    pub fn orderly_queue(s: String, k: i32) -> String {
        use std::cmp::Ordering::Greater;

        if k == 1 {
            let mut current = s.clone();
            let mut result = s;
            for _ in 0..result.len() {
                let c = current.remove(0);
                current.push(c);
                if result.cmp(&current) == Greater {
                    result = current.clone();
                }
            }
            result
        } else {
            let mut chars = s.chars().collect::<Vec<_>>();
            chars.sort();
            chars.into_iter().collect::<String>()
        }
    }
}

fn main() {
    let s = "kuh".to_string();
    let k = 1;
    println!("{}", Solution::orderly_queue(s, k))
}
