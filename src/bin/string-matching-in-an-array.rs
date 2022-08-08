struct Solution;

impl Solution {
    pub fn string_matching(words: Vec<String>) -> Vec<String> {
        let mut f = vec![0; words.len()];

        for i in 0..words.len() {
            for j in 0..words.len() {
                if i != j && words[j].contains(&words[i]) {
                    f[i] += 1;
                }
            }
        }

        f.into_iter()
            .enumerate()
            .filter(|(_, v)| *v > 0)
            .map(|(i, _)| words[i].clone())
            .collect()
    }
}

fn main() {
    let input = vec!["mass", "as", "hero", "superhero"];
    println!(
        "{:?}",
        Solution::string_matching(input.into_iter().map(|s| s.to_string()).collect())
    )
}
