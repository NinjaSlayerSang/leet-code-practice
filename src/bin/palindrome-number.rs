struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let s = x.to_string().chars().collect::<Vec<_>>();
        for i in 0..=(s.len() - 1) / 2 {
            if s[i] != s[s.len() - i - 1] {
                return false;
            }
        }
        true
    }
}

fn main() {
    println!("{}", Solution::is_palindrome(101))
}
