struct Solution;

impl Solution {
    pub fn minimum_moves(s: String) -> i32 {
        let s = s.chars().collect::<Vec<_>>();
        let mut i = 0;
        let mut r = 0;
        while i < s.len() {
            if s[i] == 'X' {
                r += 1;
                i += 3;
            } else {
                i += 1;
            }
        }
        r
    }
}

fn main() {
    println!("{}", Solution::minimum_moves("XXOX".to_string()));
}
