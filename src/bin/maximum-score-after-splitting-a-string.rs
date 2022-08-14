struct Solution;

impl Solution {
    pub fn max_score(s: String) -> i32 {
        let len = s.len();
        let s = s.chars().collect::<Vec<_>>();
        let mut r = s.iter().fold(0, |c, i| c + (*i == '1') as usize);
        let mut l = 0;

        let mut result = 0;
        for i in 0..len - 1 {
            match s[i] {
                '0' => {
                    l += 1;
                }
                '1' => {
                    r -= 1;
                }
                _ => {}
            }
            result = result.max(l + r);
        }

        result as i32
    }
}

fn main() {
    println!("{}", Solution::max_score("011101".to_string()))
}
