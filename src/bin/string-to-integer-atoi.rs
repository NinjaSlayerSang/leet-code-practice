struct Solution;

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut digit = Vec::<char>::with_capacity(s.len());
        let mut sign = 1i128;
        for (i, c) in s.trim_start().chars().enumerate() {
            if i == 0 {
                match c {
                    '+' => {}
                    '-' => sign = -1,
                    '0'..='9' => digit.push(c),
                    _ => break,
                }
            } else {
                match c {
                    '0'..='9' => digit.push(c),
                    _ => break,
                }
            }
        }
        digit.reverse();
        let mut r = 0i128;
        for i in 0..digit.len() {
            let n = digit[i].to_digit(10).unwrap() as i128;
            if n > 0 {
                if let Some(e) = 10i128.checked_pow(i as u32) {
                    if let Some(k) = e.checked_mul(n) {
                        r = r.checked_add(k).unwrap_or(i128::MAX);
                        continue;
                    }
                }
                r += i64::MAX as i128;
            }
        }

        (sign * r).clamp(i32::MIN as i128, i32::MAX as i128) as i32
    }
}

fn main() {
    println!("{}", Solution::my_atoi("   -123 with words".to_string()))
}
