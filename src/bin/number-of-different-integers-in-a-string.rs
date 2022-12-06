struct Solution;

impl Solution {
    pub fn num_different_integers(word: String) -> i32 {
        use std::{collections::HashSet, iter::once};

        let word = word.chars().chain(once('.')).collect::<Vec<_>>();
        let mut set = HashSet::new();

        let mut stack = String::new();
        for ch in word {
            match ch {
                '0'..='9' => {
                    stack.push(ch);
                }
                _ => {
                    if !stack.is_empty() {
                        set.insert(
                            stack[stack.find(|c| c != '0').unwrap_or(stack.len() - 1)..]
                                .to_string(),
                        );
                        stack.clear();
                    }
                }
            }
        }

        set.len() as i32
    }
}

fn main() {
    println!(
        "{}",
        Solution::num_different_integers("a123bc34d8ef34".to_string())
    );
}
