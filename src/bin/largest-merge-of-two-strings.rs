struct Solution;

impl Solution {
    pub fn largest_merge(word1: String, word2: String) -> String {
        let [mut l1, mut l2] = [&word1, &word2].map(|word| &word[0..]);

        let mut s = "".to_string();

        while !l1.is_empty() || !l2.is_empty() {
            if l1 >= l2 {
                s.push_str(&l1[0..=0]);
                l1 = &l1[1..];
            } else {
                s.push_str(&l2[0..=0]);
                l2 = &l2[1..];
            }
        }

        s
    }
}

fn main() {
    assert_eq!(
        Solution::largest_merge("abcabc".to_string(), "abdcaba".to_string()),
        "abdcabcabcaba"
    );
}
