struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut result = strs[0].chars().collect::<Vec<_>>();
        for i in 1..strs.len() {
            let mut k = -1;
            for (j, c) in strs[i].chars().enumerate() {
                if let Some(r) = result.get(j).copied() {
                    if r == c {
                        k = j as i32;
                        continue;
                    }
                }
                break;
            }
            result.truncate((k + 1) as usize);
        }
        result.into_iter().collect()
    }
}

fn main() {
    let strs = vec!["flower", "flow", "flight"]
        .into_iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
    println!("{}", Solution::longest_common_prefix(strs))
}
