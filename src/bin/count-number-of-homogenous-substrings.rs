struct Solution;

const M: i64 = 1000000007;

impl Solution {
    pub fn count_homogenous(s: String) -> i32 {
        let s = s.chars().collect::<Vec<_>>();
        let mut p = s[0];
        let mut n = 1;
        let mut r = 0i64;
        for i in 1..s.len() {
            if p == s[i] {
                n += 1;
            } else {
                r = (r + n * (n + 1) / 2) % M;
                p = s[i];
                n = 1;
            }
        }
        r = (r + n * (n + 1) / 2) % M;
        r as i32
    }
}

fn main() {
    println!("{}", Solution::count_homogenous("abbcccaa".to_string()));
}
