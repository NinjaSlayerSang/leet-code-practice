struct Solution;

impl Solution {
    pub fn reformat(s: String) -> String {
        use std::iter::once;

        let mut alphabet = Vec::<char>::new();
        let mut number = Vec::<char>::new();

        for c in s.chars() {
            if ('0'..='9').contains(&c) {
                number.push(c)
            } else if ('a'..='z').contains(&c) {
                alphabet.push(c)
            }
        }

        match alphabet.len() as isize - number.len() as isize {
            1 => alphabet
                .iter()
                .enumerate()
                .take(alphabet.len() - 1)
                .flat_map(|(i, a)| [*a, number[i]])
                .chain(once(*alphabet.last().unwrap()))
                .collect(),
            0 => alphabet
                .iter()
                .enumerate()
                .flat_map(|(i, a)| [*a, number[i]])
                .collect(),
            -1 => number
                .iter()
                .enumerate()
                .take(number.len() - 1)
                .flat_map(|(i, n)| [*n, alphabet[i]])
                .chain(once(*number.last().unwrap()))
                .collect(),
            _ => "".to_string(),
        }
    }
}

fn main() {
    println!("{}", Solution::reformat("covid2019".to_string()))
}
