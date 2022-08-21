struct Solution;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            vec![]
        } else {
            let alpha_bet = [
                vec!["a", "b", "c"],
                vec!["d", "e", "f"],
                vec!["g", "h", "i"],
                vec!["j", "k", "l"],
                vec!["m", "n", "o"],
                vec!["p", "q", "r", "s"],
                vec!["t", "u", "v"],
                vec!["w", "x", "y", "z"],
            ];
            let mut result = vec!["".to_string()];
            for i in 0..digits.len() {
                let a = &alpha_bet[digits[i..=i].parse::<usize>().unwrap() - 2];
                let new_items = a[1..]
                    .iter()
                    .flat_map(|c| {
                        result.iter().cloned().map(move |mut s| {
                            s.push_str(c);
                            s
                        })
                    })
                    .collect::<Vec<_>>();
                result.iter_mut().for_each(|item| item.push_str(a[0]));
                result.extend(new_items);
            }
            result
        }
    }
}

fn main() {
    println!("{:?}", Solution::letter_combinations("23".to_string()));
}
