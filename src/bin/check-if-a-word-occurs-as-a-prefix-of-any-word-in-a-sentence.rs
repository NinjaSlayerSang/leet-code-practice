struct Solution;

impl Solution {
    pub fn is_prefix_of_word(sentence: String, search_word: String) -> i32 {
        let sentence = sentence.split(' ').collect::<Vec<_>>();
        for i in 0..sentence.len() {
            if sentence[i].starts_with(&search_word) {
                return (i + 1) as i32;
            }
        }
        -1
    }
}

fn main() {
    println!(
        "{}",
        Solution::is_prefix_of_word(
            "this problem is an easy problem".to_string(),
            "pro".to_string()
        )
    )
}
