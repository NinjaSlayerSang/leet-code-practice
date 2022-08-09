struct Solution;

impl Solution {
    pub fn make_largest_special(s: String) -> String {
        let mut stack = 0;
        let mut point = 0;
        let mut divided = Vec::<String>::new();

        for (i, c) in s.chars().enumerate() {
            match c {
                '0' => {
                    stack -= 1;
                }
                '1' => {
                    stack += 1;
                }
                _ => {}
            }
            if stack == 0 {
                divided.push({
                    let item = s[point..=i].to_string();
                    let inner = &item[1..item.len() - 1];
                    if inner.is_empty() {
                        item
                    } else {
                        format!("1{}0", Solution::make_largest_special(inner.to_string()))
                    }
                });
                point = i + 1;
            }
        }

        divided.sort_by(|a, b| b.cmp(a));

        divided.iter().fold(String::new(), |mut result, item| {
            result.push_str(item);
            result
        })
    }
}

fn main() {
    println!("{}", Solution::make_largest_special("11011000".to_string()))
}
