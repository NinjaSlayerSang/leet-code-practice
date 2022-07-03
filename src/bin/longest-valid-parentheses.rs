struct Solution;

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let length = s.len();
        let s = s.chars().collect::<Vec<_>>();
        let mut f = vec![0; length];
        if length >= 2 && s[0] == '(' && s[1] == ')' {
            f[1] = 2
        }
        for i in 2..length {
            match s[i] {
                '(' => f[i] = 0,
                ')' => match s[i - 1] {
                    '(' => f[i] = f[i - 2] + 2,
                    ')' => {
                        let j = i as i32 - 1 - f[i - 1];
                        if j >= 0 && s[j as usize] == '(' {
                            let k = j - 1;
                            if k >= 0 {
                                f[i] = f[k as usize] + f[i - 1] + 2;
                            } else {
                                f[i] = f[i - 1] + 2;
                            }
                        } else {
                            f[i] = 0;
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        }
        f.into_iter().max().unwrap_or(0)
    }
}

fn main() {
    let input = "()(())".to_string();

    print!("{}", Solution::longest_valid_parentheses(input))
}
