struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows > 1 {
            let len = num_rows as usize;
            let mut result: Vec<Vec<char>> = vec![vec![]; len];

            let mut p = 0;
            let mut d = true;

            s.chars().for_each(|c| {
                result[p].push(c);
                if d {
                    if p < len - 1 {
                        p += 1;
                    } else {
                        p -= 1;
                        d = false;
                    }
                } else {
                    if p > 0 {
                        p -= 1;
                    } else {
                        p += 1;
                        d = true;
                    }
                }
            });

            result
                .into_iter()
                .flat_map(|item| item.into_iter())
                .collect()
        } else {
            s
        }
    }
}

fn main() {
    println!("{}", Solution::convert("PAYPALISHIRING".to_string(), 1))
}
