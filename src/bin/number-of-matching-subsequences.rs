struct Solution;

impl Solution {
    pub fn num_matching_subseq(s: String, words: Vec<String>) -> i32 {
        use std::collections::HashMap;

        let sl = s.len();
        let s = s.chars().collect::<Vec<_>>();
        let mut words = words
            .into_iter()
            .fold(
                HashMap::<String, (Vec<char>, usize, usize)>::new(),
                |mut map, word| {
                    map.entry(word.clone())
                        .or_insert((word.chars().collect(), 0, 0))
                        .1 += 1;
                    map
                },
            )
            .into_values()
            .collect::<Vec<_>>();

        for i in 0..sl {
            for (word, _, f) in words.iter_mut() {
                *f = *f + (*f < word.len() && s[i] == word[*f]) as usize;
            }
        }

        words.into_iter().fold(0, |r, (word, count, f)| {
            r + count as i32 * (word.len() == f) as i32
        })
    }
}

fn main() {
    println!(
        "{}",
        Solution::num_matching_subseq(
            "dsahjpjauf".to_string(),
            ["ahjpjau", "ja", "ahbwzgqnuk", "tnmlanowax"]
                .into_iter()
                .map(|str| str.to_string())
                .collect()
        )
    )
}
